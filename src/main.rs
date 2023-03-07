#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::path::PathBuf;

use eframe::egui;
use egui::{FontFamily, FontId, TextStyle};
use st_workspaces::{paths::sourcetree_settings_path, workspaces::Workspace};

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        min_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "SourceTree Workspaces App",
        options,
        Box::new(|cc| Box::new(SourceTreeWorkspacesApp::new(cc))),
    )
}

struct SourceTreeWorkspacesApp {
    version: String,
    settings_path: Option<PathBuf>,
    workspaces: Vec<Workspace>,
}

impl SourceTreeWorkspacesApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_text_styles(&cc.egui_ctx);
        Self {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            workspaces: vec![],
            settings_path: sourcetree_settings_path(),
        }
    }
}

impl eframe::App for SourceTreeWorkspacesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("SourceTree Workspaces");
                ui.label(format!("(v{})", self.version));
            });

            ui.separator();

            if self.workspaces.is_empty() {
                ui.vertical(|ui| {
                    ui.label("No Workspaces Exist... Yet...");
                    ui.label("Save one from SourceTree using Custom Action \"Save Workspace\"");
                });
            } else {
                ui.vertical(|ui| {
                    for workspace in self.workspaces.iter() {
                        ui.label(workspace.name.to_string());
                    }
                });
            }

            ui.separator();

            ui.vertical(|ui| {
                if self.settings_path.is_some() {
                    ui.small(format!(
                        "SourceTree Settings Path: {}",
                        self.settings_path
                            .as_ref()
                            .unwrap()
                            .as_os_str()
                            .to_str()
                            .unwrap()
                    ));
                } else {
                    ui.small(
                        "** SourceTree Settings Path NOT found. Try installing SourceTree. **",
                    );
                }
            });
        });
    }
}

fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::{Monospace, Proportional};

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(28.0, Proportional)),
        (TextStyle::Body, FontId::new(18.0, Proportional)),
        (TextStyle::Monospace, FontId::new(14.0, Monospace)),
        (TextStyle::Button, FontId::new(14.0, Proportional)),
        (TextStyle::Small, FontId::new(10.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}
