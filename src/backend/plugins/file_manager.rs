use std::fs::FileType;

use serde::{Deserialize, Serialize};

use crate::{backend::{commands::ICommand, plugin::{IPlugin, PluginInfo, PluginStatus}}, DesignerCore};

use super::project_manager::{ProjectManager, ProjectManagerEvent};

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-15 11:25:13
 * @modify date 2024-09-15 11:25:13
 * @desc [description]
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileEntryType {
    File,
    Folder(Vec<FileEntry>)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub file_type: FileEntryType
}

impl FileEntry {
    pub fn from_path(path: &std::path::Path) -> Self {
        let mut me = Self {name: path.file_name().unwrap().to_str().unwrap().into(), file_type: if path.is_dir() {FileEntryType::Folder(vec![])} else {FileEntryType::File}};
        if path.exists() {
            if path.is_dir() {
                let mut entries = vec![];
                for i in path.read_dir().unwrap() {
                    if let Ok(entry) = i {
                        let p = entry.path();
                        entries.push(FileEntry::from_path(&p));
                    }
                }
                me.file_type = FileEntryType::Folder(entries);
            }
        }
        me
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileManager {
    #[serde(skip_serializing)]
    status: PluginStatus,
    pub root: Option<FileEntry>,
}

impl FileManager {
    pub fn new() -> Self {
        Self { status: PluginStatus::Disabled, root: None }
    }
}

impl IPlugin for FileManager {
    fn info(&self) -> crate::backend::plugin::PluginInfo {
        PluginInfo {
            name: String::from("File manager (Native Plugin)"),
            author: "dream-lab",
            uuid: "cfa0bc17-f2f7-4ca4-bd6d-1957ab5bfeff",
            version: (0, 0, 1),
            description: String::from("Responsible to load or store files, as well as serve for file manager window."),
        }
    }

    fn status(&self) -> crate::backend::plugin::PluginStatus {
        self.status.clone()
    }

    fn initialize(&mut self, root: &mut crate::DesignerCore) {
        
    }

    fn enable(&mut self, root: &mut crate::DesignerCore) {
        self.status = PluginStatus::Enabled
    }

    fn disable(&mut self, root: &mut crate::DesignerCore) {
        self.status = PluginStatus::Disabled;
    }

    fn load_state(&mut self, storage: &mut dyn crate::backend::plugin::IPluginStateStorage) {
        *self = serde_json::from_value(storage.load_state()).unwrap();
    }

    fn store_state(&mut self, storage: &mut dyn crate::backend::plugin::IPluginStateStorage) {
        storage.store_state(serde_json::to_value(self).unwrap());
    }

    fn get_state(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn handle_message(&mut self, root: &mut crate::DesignerCore, sender: &str, action: &str, args: serde_json::Value) {
        if sender == "designer.project_manager" {
            if action == "designer.project_manager.event" {
                let args = serde_json::from_value::<ProjectManagerEvent>(args.clone()).unwrap();
                match args {
                    ProjectManagerEvent::Created { path } => {},
                    ProjectManagerEvent::Opened { path } => {
                        root.publish_message("designer.file_manager", &FileManagerCommand::ReloadProjectFiles);
                    },
                }
            }
        }

        if action == "designer.file_manager.management" {
            let args = serde_json::from_value::<FileManagerCommand>(args.clone()).unwrap();
            match args {
                FileManagerCommand::OpenFile(_) => {},
                FileManagerCommand::CloseFile(_) => {},
                FileManagerCommand::RequestContextMenu(_) => {},
                FileManagerCommand::ReloadProjectFiles => {
                    self.read_all_file_list(root);
                    root.publish_message("designer.file_manager", &FileManagerEvent::ProjectFilesAreReopened);
                },
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl FileManager {
    fn read_all_file_list(&mut self, root: &mut DesignerCore) {
        let project_manager = root.get_plugin::<ProjectManager>().unwrap();
        self.root = Some(FileEntry::from_path(project_manager.path.clone().unwrap().as_path()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileManagerCommand {
    ReloadProjectFiles,
    OpenFile(String),
    CloseFile(String),
    RequestContextMenu(String)
}

impl ICommand for FileManagerCommand {
    fn action<'a>(&'a self) -> &'a str {
        "designer.file_manager.management"
    }

    fn args<'a>(&'a self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileManagerEvent {
    ProjectFilesAreReopened,
}

impl ICommand for FileManagerEvent {
    fn action<'a>(&'a self) -> &'a str {
        "designer.file_manager.event"
    }

    fn args<'a>(&'a self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
