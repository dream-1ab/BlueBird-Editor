use std::{fs::File, io::{Read, Write}, path::PathBuf};

use eframe::WindowBuilder;
use egui::{Align2, CentralPanel, DragValue, Grid, Layout, TopBottomPanel, Ui, Vec2, Widget};
use egui_dock::DockState;
use log::{error, info};
use serde::{de::IntoDeserializer, Deserialize, Serialize};

use crate::{backend::plugins::project_manager::{Project, ProjectManager, ProjectManagerCommand}, dock_manager::EditorWindowState, DesignerUI};


/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-08 10:51:31
 * @modify date 2024-09-08 10:51:31
 * @desc [description]
*/


impl Project {
    pub fn new_from_empty() -> Self {
        Self { name: "".into(), description: "".into(), version: (0, 0, 1), package_name: "".into(), author: "".into(), email: "".into() }
    }
}

#[derive(Debug, Clone)]
pub enum WindowStatus {
    Open,
    New,
    Closed,
}

pub struct ProjectManagerWindow {
    status: WindowStatus,
    dialog_has_open: bool,
    project: Project,
    path: Option<PathBuf>,
}

impl Project {
    pub fn editor_ui(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        Grid::new("grid").striped(true).spacing(Vec2::new(0.0, 3.0)).max_col_width(500.0).show(ui, |ui|{
            let project = self;
            ui.label("project name");
            ui.text_edit_singleline(&mut project.name).on_hover_text("Project name here");
            ui.end_row();

            ui.label("description");
            ui.text_edit_multiline(&mut project.description);
            ui.end_row();

            ui.label("version");
            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut project.version.0).range(0..=9999));
                ui.label(":");
                ui.add(DragValue::new(&mut project.version.1).range(0..=9999));
                ui.label(":");
                ui.add(DragValue::new(&mut project.version.2).range(0..=9999));
            });
            ui.end_row();

            ui.label("package name");
            ui.text_edit_singleline(&mut project.package_name);
            ui.end_row();

            ui.label("author");
            ui.text_edit_singleline(&mut project.author);
            ui.end_row();

            ui.label("email");
            ui.text_edit_singleline(&mut project.email);
            ui.end_row();
        });
    }
}

impl ProjectManagerWindow {
    pub fn new() -> Self {
        Self {dialog_has_open: false, status: WindowStatus::Closed, project: Project::new_from_empty(), path: None }
    }
    pub fn update(app: &mut DesignerUI, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut create_project_button_is_pressed = false;
        if let WindowStatus::New = app.project_manager.status {
            egui::Window::new("Create new project").open(&mut app.project_manager.dialog_has_open).movable(true).resizable(true).collapsible(false).show(ctx, |ui|{
                ui.vertical_centered_justified(|ui|{
                    app.project_manager.project.editor_ui(ctx, ui);
                    ui.allocate_space(Vec2::new(0.0, 10.0));
                    ui.separator();
                    ui.allocate_space(Vec2::new(0.0, 10.0));
                    ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                        create_project_button_is_pressed = ui.button("Create project").clicked();
                    });
                });
            });
        }
        if create_project_button_is_pressed {
            app.publish_command("ProjectManagerWindow", &ProjectManagerCommand::CreateProject { path: app.project_manager.path.clone().unwrap().to_str().unwrap().to_string(), project: app.project_manager.project.clone() });
            app.project_manager.dialog_has_open = false;
            app.project_manager.status = WindowStatus::Closed;
            app.project_manager.path = None;
        }
    }
    
    pub fn create_project(&mut self, path: PathBuf) {
        self.path = Some(path);
        self.dialog_has_open = true;
        self.status = WindowStatus::New;
    }
    
    pub fn open_project(app: &mut DesignerUI, path: &PathBuf) {
        app.publish_command("ProjectManagerWindow", &ProjectManagerCommand::OpenProject { path: path.to_str().unwrap().to_string() });
    }

    

}

