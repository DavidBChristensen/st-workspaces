#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use st_workspaces::{
    app::SourceTreeWorkspacesApp,
    open_tabs::OpenTabs,
    sourcetree_actions,
    workspaces::{Workspace, Workspaces},
};

/// Responsible for managing workspaces
/// use cases:
/// - Manage Workspaces, no param
/// - Save Workspace, from custom action
/// - Launch SourceTree with current active configuration
fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    // try to close SourceTree first, as this should never be up at the same time.
    let close_result = sourcetree_actions::close_sourcetree();

    if close_result.is_err() {
        println!("Couldn't close SourceTree");
    }

    let mut workspaces = Workspaces::read().unwrap_or_default();
    workspaces.force_valid_workspace();

    if let Ok(open_tabs) = OpenTabs::read() {
        let mut last_workspace = Workspace::from(&open_tabs);
        let search_result = workspaces
            .workspaces
            .iter()
            .find(|workspace| workspace.uuid == last_workspace.uuid);

        if search_result.is_none() {
            last_workspace.name = "Last Workspace".to_owned();
            workspaces.workspaces.push(last_workspace);
        }

        workspaces
            .write()
            .expect("Couldn't write workspace after loading last workspace.");
    }

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
