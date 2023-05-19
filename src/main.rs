#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use anyhow::{bail, Error};
use flexi_logger::{FileSpec, Logger, WriteMode};

use log::{error, info, warn};
use st_workspaces::{
    app::SourceTreeWorkspacesApp,
    open_tabs::OpenTabs,
    paths::sourcetree_settings_path,
    sourcetree_actions::{self, CloseResult},
    workspaces::{Workspace, Workspaces},
};
use uuid::Uuid;

fn main() -> Result<(), Error> {
    let settings_path = sourcetree_settings_path().unwrap().join("log");
    let _logger = Logger::try_with_str("info, my::critical::module=trace")?
        .log_to_file(FileSpec::default().directory(settings_path))
        .write_mode(WriteMode::BufferAndFlush)
        .start()?;

    let last_workspace_id = discover_last_workspace_id();
    info!("Last workspace id is {:?}", last_workspace_id);

    close_sourcetree();

    let mut workspaces = get_workspaces();

    if last_workspace_id.is_some() {
        update_last_workspace(&mut workspaces, last_workspace_id.unwrap());
        save_workspaces(&workspaces);
        save_open_tabs(&workspaces)
    }

    launch_app(workspaces)
}

fn discover_last_workspace_id() -> Option<Uuid> {
    if let Ok(open_tabs) = OpenTabs::read() {
        info!("Was able to open tabs.");
        let last_workspace = Workspace::from(&open_tabs);
        if !last_workspace.uuid.is_nil() {
            return Some(last_workspace.uuid);
        }
    }
    None
}

fn save_open_tabs(workspaces: &Workspaces) {
    if let Some(current_workspace) = workspaces.current_workspace() {
        let open_tabs = OpenTabs::from(current_workspace);
        let write_result = OpenTabs::write(&open_tabs);
        match write_result {
            Ok(_) => info!("Saved current open tabs"),
            Err(_) => warn!("Couldn't save current open tabs"),
        }
    }
}

fn get_workspaces() -> Workspaces {
    let mut workspaces = Workspaces::read().unwrap_or_default();
    workspaces.force_valid_workspace();
    workspaces
}

fn save_workspaces(workspaces: &Workspaces) {
    workspaces
        .write()
        .expect("Couldn't write workspace after loading last workspace.");
}

fn close_sourcetree() {
    // try to close SourceTree first, as this should never be up at the same time.
    let close_result = sourcetree_actions::close_sourcetree();

    match close_result {
        Ok(CloseResult::Closed) => info!("Closed SourceTree."),
        Ok(CloseResult::ProcessNotRunning) => {
            info!("Didn't close SourceTree, because it wasn't running")
        }
        Err(_) => error!("Error occurred closing SourceTree."),
    }

    // TODO(dchristensen) do we have to wait for the open tabs file to be written
    // before exiting this function?
    // How would we do that? Maybe sit around and wait until the task is killed, and if it's
    // already killed have some timeout?

    std::thread::sleep(std::time::Duration::from_secs(4));
}

fn update_last_workspace(workspaces: &mut Workspaces, last_workspace_id: Uuid) {
    // States:
    // - open tabs has last id => update workspace with associated id
    // - open tabs has no id => save as last workspace,
    //      Some considerations:
    //          - let the user know you did that?
    //          - what if there is already a last workspace? replace
    //          - Maybe just prompt the user what to do? like select a workspace to save or cancel?
    //          - Maybe just save over the last selected id, no matter what?
    //
    // Is there a way to hook into SourceTree, and after it closes, save the current over the
    // current workspace?

    info!("Updating last workspace.");

    if let Ok(open_tabs) = OpenTabs::read() {
        info!("Was able to open tabs.");
        let mut last_workspace = Workspace::from(&open_tabs);

        if workspaces.workspaces.contains_key(&last_workspace_id) {
            info!(
                "Last workspace {} in saved workspace. Updating with lastest.",
                last_workspace_id
            );

            last_workspace.uuid = last_workspace_id;
            last_workspace.name = workspaces.workspaces[&last_workspace_id].name.clone();
        } else {
            info!(
                "Last workspace {} not in saved workspaces. Creating new workspace.",
                last_workspace_id
            );

            last_workspace.name = "Last Workspace".to_owned();
        };

        info!("The last workspace is {:?}", last_workspace);
        workspaces
            .workspaces
            .insert(last_workspace.uuid, last_workspace);
    } else {
        info!("Couldn't open SourceTree's Open Tabs from last session.");
    }
}

fn launch_app(workspaces: Workspaces) -> Result<(), anyhow::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        min_window_size: Some(egui::vec2(640.0, 480.0)),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "SourceTree Workspaces App",
        options,
        Box::new(|cc| Box::new(SourceTreeWorkspacesApp::new(cc, workspaces))),
    )
    .or_else(|_| bail!("Error runnning ui"))
}
