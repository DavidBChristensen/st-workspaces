use std::path::PathBuf;

use egui::{Color32, FontFamily, FontId, RichText, TextStyle};

use crate::{paths::sourcetree_settings_path, workspaces::Workspace};

/// Main UI application struct.
pub struct SourceTreeWorkspacesApp {
    version: String,
    settings_path: Option<PathBuf>,
    workspaces: Vec<Workspace>,
}

impl SourceTreeWorkspacesApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
            let dark_mode = ui.visuals().dark_mode;
            ui.horizontal(|ui| {
                ui.heading(contrast_text("SourceTree Workspaces", dark_mode));
                ui.label(format!("(v{})", self.version));
                egui::widgets::global_dark_light_mode_buttons(ui);
            });

            ui.separator();

            if self.workspaces.is_empty() {
                ui.vertical(|ui| {
                    ui.label(contrast_text("No Workspaces Exist... Yet...", dark_mode));
                    ui.label(contrast_text(
                        "Save one from SourceTree using Custom Action \"Save Workspace\"",
                        dark_mode,
                    ));
                });
            } else {
                ui.vertical(|ui| {
                    for workspace in self.workspaces.iter() {
                        ui.label(workspace.name.to_string());
                    }
                });
            }
        });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .show(ctx, |ui| {
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

pub fn configure_text_styles(ctx: &egui::Context) {
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

pub fn contrast_text(text: &str, dark_mode: bool) -> RichText {
    if dark_mode {
        RichText::new(text).color(Color32::from_rgb(200, 200, 200))
    } else {
        RichText::new(text).color(Color32::from_rgb(10, 10, 10))
    }
}
