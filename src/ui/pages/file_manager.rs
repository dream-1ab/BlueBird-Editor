use std::path::PathBuf;

use egui::{CollapsingHeader, Widget};
use egui_hooks::UseHookExt;

use crate::{backend::plugins::{file_manager::{FileEntry, FileManager, FileManagerCommand}, project_manager::ProjectManager}, dock_manager::MyTab};

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-08 18:53:44
 * @modify date 2024-09-08 18:53:44
 * @desc [description]
*/

pub struct FileManagerPage<'a, 'b> {
    pub tab: &'a mut MyTab<'b>
}

impl<'a, 'b> Widget for FileManagerPage<'a, 'b> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                if ui.button("Refresh").clicked() {
                    self.tab.app.publish_command("FileManagerUI", &FileManagerCommand::ReloadProjectFiles);
                }
                // ui.button("Collapse folders");
            });
            ui.separator();

            let manager = self.tab.app.core.get_plugin::<FileManager>().unwrap();
            let (project_is_available, path) = {
                (manager.root.is_some(), manager.root.clone())
            };
            if project_is_available {
                self.render_file_entry(ui, &path.unwrap(), true);
            } else {
                ui.label("Please open or create a project first.");
            }
        }).response
    }
}

impl<'a, 'b> FileManagerPage<'a, 'b> {
    fn render_file_entry(&self, ui: &mut egui::Ui, file_entry: &FileEntry, is_root_dir: bool) {
        match file_entry.file_type.clone() {
            crate::backend::plugins::file_manager::FileEntryType::File => {
                ui.label(&file_entry.name).context_menu(|ui|{
                    ui.button("Open");
                    ui.button("Delete");
                    ui.button("Rename");
                    ui.separator();
                    ui.button("New Folder");
                    ui.button("New File");
                });
            },
            crate::backend::plugins::file_manager::FileEntryType::Folder(children) => {
                CollapsingHeader::new(if is_root_dir {"project://"} else {&file_entry.name}).show(ui, |ui|{
                    for entry in &*children {
                        self.render_file_entry(ui, entry, false);
                    }
                }).header_response.context_menu(|ui| {
                    ui.button("Open");
                    ui.button("Delete");
                    ui.button("Rename");
                    ui.separator();
                    ui.button("New Folder");
                    ui.button("New File");
                });
            },
        };
    }
}
