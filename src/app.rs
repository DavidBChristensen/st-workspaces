use std::path::PathBuf;

use egui::{
    Align, Button, Color32, FontFamily, FontId, Label, Layout, RichText, Sense, TextStyle, Ui,
};
use log::info;
use uuid::Uuid;

use crate::{
    open_tabs::OpenTabs,
    paths::{sourcetree_exec_path, sourcetree_settings_path},
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
    fn update(&mut self, context: &egui::Context, frame: &mut eframe::Frame) {
        self.update_top_panel(context);
        self.update_central_panel(context);
        self.update_bottom_panel(context, frame);
    }
}

impl SourceTreeWorkspacesApp {
    fn update_top_panel(&mut self, context: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(context, |ui| {
            let dark_mode = ui.visuals().dark_mode;
            ui.horizontal(|ui| {
                ui.heading(contrast_text("SourceTree Workspaces", false, dark_mode));
                ui.label(format!("(v{})", self.version));
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }

    fn update_central_panel(&mut self, context: &egui::Context) {
        egui::CentralPanel::default().show(context, |ui| {
            ui.horizontal(|ui| {
                self.update_workspace_list_panel(ui);
                ui.separator();
                self.update_workspace_details_panel(ui);
            });
        });
    }

    fn update_workspace_list_panel(&mut self, ui: &mut Ui) {
        let dark_mode = ui.visuals().dark_mode;
        ui.vertical(|ui| {
            if self.workspaces.workspaces.is_empty() {
                ui.label(contrast_text(
                    "No workspaces exist... yet...",
                    false,
                    dark_mode,
                ));
            } else {
                let mut sorted_workspaces: Vec<_> =
                    self.workspaces.workspaces.iter().map(|w| w.1).collect();
                sorted_workspaces.sort();

                for workspace in sorted_workspaces.iter() {
                    if ui
                        .add(
                            Label::new(contrast_text(
                                workspace.name.as_str(),
                                workspace.uuid == self.workspaces.current_workspace,
                                dark_mode,
                            ))
                            .sense(Sense::click()),
                        )
                        .clicked()
                    {
                        self.workspaces.current_workspace = workspace.uuid;
                    };
                }
            }

            ui.horizontal(|ui| {
                if ui.add(Button::new("Create New\nWorkspace")).clicked() {
                    self.create_new_workspace();
                }

                if ui.add(Button::new("Create from\nCurrent Tabs")).clicked() {
                    self.create_workspace_from_current_tabs();
                }
            });
        });
    }

    fn update_workspace_details_panel(&mut self, ui: &mut Ui) {
        if self.workspaces.workspaces.is_empty() || self.workspaces.current_workspace.is_nil() {
            return;
        }
        ui.vertical(|ui| {
            let dark_mode = ui.visuals().dark_mode;
            let mut should_save = false;
            if let Some(current_workspace) = self.workspaces.current_workspace_mut() {
                ui.horizontal(|ui| {
                    ui.label(contrast_text("Name ", false, dark_mode));
                    if ui
                        .text_edit_singleline(&mut current_workspace.name)
                        .lost_focus()
                    {
                        should_save = true;
                    }
                });

                for repo_path in current_workspace.repo_paths.iter() {
                    ui.label(contrast_text(repo_path.as_str(), false, dark_mode));
                }
            }

            if should_save {
                let write_result = self.workspaces.write();
                if write_result.is_err() {
                    self.status = "Error occurred writing to disk.".to_owned();
                }
            }
        });

        ui.with_layout(Layout::bottom_up(Align::RIGHT), |ui| {
            if ui.add(Button::new("Remove Workspace")).clicked() {
                self.remove_current_workspace();
            }
        });
    }

    fn update_bottom_panel(&mut self, context: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .show(context, |ui| {
                let dark_mode = ui.visuals().dark_mode;
                ui.horizontal(|ui| {
                    if self.workspaces.current_workspace().is_some()
                        && ui.button("Open Workspace").clicked()
                    {
                        self.open_current_workspace(frame);
                    }

                    // if ui.button("Close SourceTree").clicked() && close_sourcetree().is_err() {
                    //     self.status = "Error closing SourceTree".to_owned();
                    // }
                });
                ui.separator();
                ui.label(contrast_text(&self.status, false, dark_mode));
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

    fn create_new_workspace(&mut self) {
        info!("Creating new workspace...");
        let new_workspace = Workspace::new("New Workspace", Uuid::new_v4());
        self.workspaces
            .workspaces
            .insert(new_workspace.uuid, new_workspace);
        let write_result = self.workspaces.write();

        if write_result.is_err() {
            self.status = "Error creating workspace from current tabs".to_owned();
        } else {
            self.status = "Created workspace from current tabs".to_owned();
        }

        self.workspaces.force_valid_workspace();
    }

    fn create_workspace_from_current_tabs(&mut self) {
        info!("Creating workspace from currently open tabs...");
        let open_tabs = OpenTabs::read().unwrap();
        let mut new_workspace: Workspace = (&open_tabs).into();
        new_workspace.uuid = Uuid::new_v4();
        self.workspaces
            .workspaces
            .insert(new_workspace.uuid, new_workspace);
        let write_result = self.workspaces.write();

        if write_result.is_err() {
            self.status = "Error creating workspace from current tabs".to_owned();
        } else {
            self.status = "Created workspace from current tabs".to_owned();
        }

        self.workspaces.force_valid_workspace();
    }

    fn remove_current_workspace(&mut self) {
        info!("Deleting current workspace...");
        self.workspaces
            .workspaces
            .remove(&self.workspaces.current_workspace);

        let write_result = self.workspaces.write();

        if write_result.is_err() {
            self.status = "Error creating workspace from current tabs".to_owned();
        } else {
            self.status = "Created workspace from current tabs".to_owned();
        }

        //        self.workspaces.force_valid_workspace();
    }

    fn open_current_workspace(&mut self, frame: &mut eframe::Frame) {
        if self.workspaces.write().is_err() {
            info!("Didn't save workspace when closing.");
        }

        let current_workspace = self.workspaces.current_workspace().unwrap();
        let open_tabs: OpenTabs = current_workspace.into();
        if OpenTabs::write(&open_tabs).is_err() {
            self.status = "Couldn't write open tabs, so can't launch SourceTree.".to_owned();
            return;
        }

        let spawn_result = std::process::Command::new(
            sourcetree_exec_path()
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap(),
        )
        .spawn();

        if spawn_result.is_err() {
            self.status = "Couldn't launch SourceTree. Is it installed?".to_owned();
            return;
        }

        if spawn_result.unwrap().wait().is_err() {
            self.status = "Couldn't switch to SourceTree fully. Process error?".to_owned();
            return;
        }

        frame.close();
    }
}

impl From<&OpenTabs> for Workspace {
    fn from(open_tabs: &OpenTabs) -> Self {
        let mut uuid = Uuid::new_v4();
        if open_tabs.workspace_id.is_some() {
            uuid = open_tabs.workspace_id.unwrap();
        }

        Workspace {
            uuid,
            name: "New Workspace".to_owned(),
            repo_paths: open_tabs.tabs.clone(),
        }
    }
}

impl From<&Workspace> for OpenTabs {
    fn from(workspace: &Workspace) -> Self {
        OpenTabs {
            tabs: workspace.repo_paths.clone(),
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

fn contrast_text(text: &str, highlight: bool, dark_mode: bool) -> RichText {
    if highlight == dark_mode {
        RichText::new(text)
            .color(Color32::from_rgb(10, 10, 10))
            .background_color(Color32::from_rgb(255, 255, 255))
    } else {
        RichText::new(text)
            .color(Color32::from_rgb(200, 200, 200))
            .background_color(Color32::from_rgb(27, 27, 27))
    }
}
