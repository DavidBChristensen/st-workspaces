#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use st_workspaces::{
    app::SourceTreeWorkspacesApp,
    open_tabs::OpenTabs,
    sourcetree_actions::{self, CloseResult},
    workspaces::{Workspace, Workspaces},
};

fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init(); // Log to stdout (if you run with `RUST_LOG=debug`).
    close_sourcetree();

    let mut workspaces = get_workspaces();
    update_last_workspace(&mut workspaces);
    save_workspaces(&workspaces);
    launch_app(workspaces)
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
        Ok(CloseResult::Closed) => println!("Closed SourceTree."),
        Ok(CloseResult::ProcessNotRunning) => {
            println!("Didn't close SourceTree, because it wasn't running")
        }
        Err(_) => println!("Error occurred closing SourceTree."),
    }
}

fn update_last_workspace(workspaces: &mut Workspaces) {

    // States
    // open tab has last id => update workspace with associated id
    // open tab has no id => save as last workspace, and let the user know you did that

    if let Ok(open_tabs) = OpenTabs::read() {
        let mut last_workspace = Workspace::from(&open_tabs);

        if workspaces.workspaces.contains_key(&last_workspace.uuid) {
        } else {
            last_workspace.name = "Last Workspace".to_owned();
            workspaces
                .workspaces
                .insert(last_workspace.uuid, last_workspace);
        }
    }
}

fn launch_app(workspaces: Workspaces) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        min_window_size: Some(egui::vec2(640.0, 480.0)),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "SourceTree Workspaces App",
        options,
        Box::new(|cc| Box::new(SourceTreeWorkspacesApp::new(cc, workspaces))),
    )
}
