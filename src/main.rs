#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use st_workspaces::{app::SourceTreeWorkspacesApp, workspaces::Workspaces};

/// Responsible for managing workspaces
/// use cases:
/// - Manage Workspaces, no param
/// - Save Workspace, from custom action
/// - Launch SourceTree with current active configuration
fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let mut workspaces = Workspaces::read().unwrap_or_default();
    workspaces.force_valid_workspace();

    if workspaces.current_workspace.is_nil() && !workspaces.workspaces.is_empty() {
        workspaces.current_workspace = workspaces.workspaces.first().unwrap().uuid;
    }

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        min_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "SourceTree Workspaces App",
        options,
        Box::new(|cc| Box::new(SourceTreeWorkspacesApp::new(cc, workspaces))),
    )
}
