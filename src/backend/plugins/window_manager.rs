use std::{fs::File, io::Write};

use log::error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{backend::{commands::ICommand, counter::Counter, plugin::{IPlugin, PluginInfo, PluginStatus}, plugins::logger::LogCommand}, DesignerCore};

use super::project_manager::{ProjectManager, ProjectManagerEvent};

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-11 23:42:45
 * @modify date 2024-09-11 23:42:45
 * @desc [description]
*/


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowManager {
    pub status: PluginStatus,
    pub dock_state: Option<Value>,
    #[serde(skip_serializing)]
    pub dock_changes: Counter,
}

impl WindowManager {
    pub fn new() -> Self {
        Self { status: PluginStatus::Disabled, dock_state: None, dock_changes: Counter::new() }
    }

    pub fn load_dock_state(&mut self, root: &mut DesignerCore) -> Option<Value> {
        let project_manager = root.get_plugin::<ProjectManager>().unwrap();
        let path = project_manager.path.clone().as_ref().unwrap().join(".designer").join("settings.json");
        if !path.exists() {
            return None;
        }
        let state = {
            serde_json::from_str::<Value>(&std::fs::read_to_string(path).expect("Cannot read project settings.json file"))
        };
        if let Ok(state) = state {
            root.publish_message("WindowManager", &LogCommand::Generate { category: String::from("Succeed"), content: String::from("Window dock state is stored.") });
            root.publish_message("WindowManager", &WindowManagerEvent::DockStateRestored);
            self.dock_changes.count();
            Some(state)
        } else {
            error!("Cannot deserialize window dock state so discarded.");
            root.publish_message("WindowManager", &LogCommand::Generate { category: String::from("Error"), content: String::from("Cannot deserialize window dock state so discarded.") });
            None
        }
    }

    pub fn store_dock_state(&mut self, root: &mut DesignerCore, dock_state: Value) {
        let json_text = {
            serde_json::to_string_pretty(&dock_state).unwrap_or("".into())
        };
        if json_text.is_empty() {
            error!("Cannot store window dock state.");
            root.publish_message("WindowManager", &LogCommand::Generate { category: String::from("Error"), content: String::from("Cannot store window dock state.") });
        } else {
            let project_manager = root.get_plugin::<ProjectManager>().unwrap();
            let path = project_manager.path.clone().unwrap().join(".designer").join("settings.json");
            if !path.parent().unwrap().exists() {
                std::fs::create_dir_all(&path.parent().unwrap()).expect("Cannot create project settings files directory.");
            }
            let mut file = File::create(&path).expect(format!("Cannot open/create editor settings file in your project {:?}.", &path).as_str());
            file.write_all(json_text.as_bytes()).expect("Cannot write editor settings file in your project.");
            root.publish_message("WindowManager", &LogCommand::Generate { category: String::from("Succeed"), content: String::from("Window dock state is stored.") });
            root.publish_message("WindowManager", &WindowManagerEvent::DockStateSaved);
        }
    }
}

impl IPlugin for WindowManager {
    fn info(&self) -> crate::backend::plugin::PluginInfo {
        PluginInfo {
            uuid: "35f27394-492c-4af5-804d-803a18a606e8",
            name: String::from("Window manager"),
            version: (0, 0, 1),
            author: "dream-lab",
            description: String::from("Manages all the windowing functionality."),
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
        self.status = PluginStatus::Disabled
    }

    fn load_state(&mut self, storage: &mut dyn crate::backend::plugin::IPluginStateStorage) {
        *self = serde_json::from_value(storage.load_state()).unwrap()
    }

    fn store_state(&mut self, storage: &mut dyn crate::backend::plugin::IPluginStateStorage) {
        storage.store_state(serde_json::to_value(self).unwrap());
    }

    fn get_state(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn handle_message(&mut self, root: &mut crate::DesignerCore, sender: &str, action: &str, args: Value) {
        if action != "designer.window_manager.management" {
            if action == "designer.project_manager.event" {
                let command = serde_json::from_value::<ProjectManagerEvent>(args).unwrap();
                match command {
                    ProjectManagerEvent::Opened { path } => {
                        root.publish_message("WindowManager", &WindowManagerCommand::LoadDockState);
                    },
                    ProjectManagerEvent::Created { path } => {
                        
                    },
                }
            }
            return;
        }
        let command = serde_json::from_value::<WindowManagerCommand>(args).unwrap();
        match command {
            WindowManagerCommand::OpenWindow(id) => {

            },
            WindowManagerCommand::CloseWindow(id) => {

            },
            WindowManagerCommand::SaveDockState(state) => {
                self.store_dock_state(root, state);
            },
            WindowManagerCommand::LoadDockState => {
                let state = self.load_dock_state(root);
                self.dock_state = state;
            }
        }
        root.notify_ui();
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowManagerCommand {
    OpenWindow(EditorWindowId),
    CloseWindow(EditorWindowId),
    SaveDockState(Value),
    LoadDockState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub enum EditorWindowId {
    WelcomePage ,
    FileManagerWindow,
    LogViewWindow,
    InspectorWindow,
    LibraryWindow,
    ProjectSettingsWindow,
    EditorSettingsWindow,
    ExtensionsWindow,
    NodeEditorWindow,
    CodeEditorWindow,
}

impl ICommand for WindowManagerCommand {
    fn action<'a>(&'a self) -> &'a str {
        "designer.window_manager.management"
    }

    fn args<'a>(&'a self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowManagerEvent {
    DockStateSaved,
    DockStateRestored,
}

impl ICommand for WindowManagerEvent {
    fn action<'a>(&'a self) -> &'a str {
        "designer.window_manager.event"
    }

    fn args<'a>(&'a self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}
