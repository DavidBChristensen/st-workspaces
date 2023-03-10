use std::path::PathBuf;

use egui::{Color32, FontFamily, FontId, RichText, TextStyle};
use uuid::Uuid;

use crate::{
    open_tabs::OpenTabs,
    paths::sourcetree_settings_path,
    workspaces::{Workspace, Workspaces},
};

/// Main UI application struct.
pub struct SourceTreeWorkspacesApp {
    version: String,
    settings_path: Option<PathBuf>,
    workspaces: Workspaces,
    status: String,
}

impl SourceTreeWorkspacesApp {
    pub fn new(cc: &eframe::CreationContext<'_>, workspaces: Workspaces) -> Self {
        configure_text_styles(&cc.egui_ctx);

        Self {
            version: env!("CARGO_PKG_VERSION").to_owned(),
            workspaces,
            settings_path: sourcetree_settings_path(),
            status: "".to_owned(),
        }
    }
}

impl eframe::App for SourceTreeWorkspacesApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_top_panel(context);
        self.update_central_panel(context);
        self.update_bottom_panel(context);
    }
}

impl SourceTreeWorkspacesApp {
    fn update_top_panel(&mut self, context: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(context, |ui| {
            let dark_mode = ui.visuals().dark_mode;
            ui.horizontal(|ui| {
                ui.heading(contrast_text("SourceTree Workspaces", dark_mode));
                ui.label(format!("(v{})", self.version));
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }

    fn update_central_panel(&mut self, context: &egui::Context) {
        egui::CentralPanel::default().show(context, |ui| {
            let dark_mode = ui.visuals().dark_mode;
            if self.workspaces.workspaces.is_empty() {
                ui.vertical(|ui| {
                    ui.label(contrast_text("No Workspaces Exist... Yet...", dark_mode));
                });
            } else {
                ui.vertical(|ui| {
                    for workspace in self.workspaces.workspaces.iter() {
                        ui.label(workspace.name.to_string());
                    }
                });
            }

            if ui
                .add(egui::Button::new("Create Workspace from Current Tabs"))
                .clicked()
            {
                self.create_workspace_from_current_tabs();
            }
        });
    }

    fn update_bottom_panel(&mut self, context: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .show(context, |ui| {
                let dark_mode = ui.visuals().dark_mode;
                ui.label(contrast_text(&self.status, dark_mode));
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

    fn create_workspace_from_current_tabs(&mut self) {
        println!("Creating workspace...");
        let open_tabs = OpenTabs::read().unwrap();
        self.workspaces.workspaces.push(open_tabs.into());
        let write_result = self.workspaces.write();
        if write_result.is_err() {
            self.status = "Error creating workspace from current tabs".to_owned();
        } else {
            self.status = "Created workspace from current tabs".to_owned();
        }
    }
}

impl From<OpenTabs> for Workspace {
    fn from(open_tabs: OpenTabs) -> Self {
        Workspace {
            uuid: Uuid::new_v4(),
            name: "New Workspace".to_owned(),
            repo_paths: open_tabs.tabs,
        }
    }
}

impl From<Workspace> for OpenTabs {
    fn from(workspace: Workspace) -> Self {
        OpenTabs {
            tabs: workspace.repo_paths,
            workspace_id: Some(workspace.uuid),
        }
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

fn contrast_text(text: &str, dark_mode: bool) -> RichText {
    if dark_mode {
        RichText::new(text).color(Color32::from_rgb(200, 200, 200))
    } else {
        RichText::new(text).color(Color32::from_rgb(10, 10, 10))
    }
}
