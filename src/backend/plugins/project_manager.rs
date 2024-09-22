use std::{cell::RefCell, fs::File, io::Write, ops::Deref, path::PathBuf, rc::Rc, str::FromStr, vec};

use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{backend::{commands::ICommand, plugin::{IPlugin, PluginInfo, PluginStatus}, plugins::logger::LogCommand}, DesignerCore, DesignerUI};



/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-08 21:21:33
 * @modify date 2024-09-08 21:21:33
 * @desc [description]
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub version: (u32, u32, u32),
    pub package_name: String,
    pub author: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectManager {
    pub path: Option<PathBuf>,
    pub recent_projects: Vec<String>,
    pub status: PluginStatus,
    #[serde(skip_serializing)]
    pub project: Option<Project>
}

impl ProjectManager {
    pub fn project_is_available(&self) -> bool {
        self.project.is_some()
    }
}

impl<'b> IPlugin for ProjectManager {
    fn info(&self) -> crate::backend::plugin::PluginInfo {
        PluginInfo {
            name: String::from("Project manager (Native plugin)"),
            author: "dream-lab",
            uuid: "3979dec2-8e5c-4860-8b1c-07a8fd2d560f",
            description: String::from("Provides Opening, Creating And Analyzing features for IDE"),
            version: (0, 0, 1)
        }
    }

    fn initialize(&mut self, root: &mut DesignerCore) {
        
    }

    fn enable(&mut self, root: &mut DesignerCore) {
        self.status = PluginStatus::Enabled;
    }

    fn disable(&mut self, root: &mut DesignerCore) {
        self.status = PluginStatus::Disabled;
    }

    fn get_state(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn status(&self) -> PluginStatus {self.status.clone()}
    
    fn load_state(&mut self, storage: &mut dyn crate::backend::plugin::IPluginStateStorage) {
        *self = serde_json::from_value(storage.load_state()).unwrap();
    }
    
    fn store_state(&mut self, storage: &mut dyn crate::backend::plugin::IPluginStateStorage) {
        storage.store_state(serde_json::to_value(self).unwrap());
    }

    fn handle_message(&mut self, root: &mut DesignerCore, sender: &str, action: &str, args: Value) {
        if action != "designer.project_manager.management" {return;}
        match serde_json::from_value::<ProjectManagerCommand>(args).unwrap() {
            ProjectManagerCommand::OpenProject { path } => {
                self.open_project(root, &path);
            },
            ProjectManagerCommand::CloseProject => {

            },
            ProjectManagerCommand::CreateProject { path, project } => {
                self.create_project_directory(root, &path, &project);
            }
        }
        root.notify_ui();
    }
}

impl ProjectManager {
    fn open_project(&mut self, root: &mut DesignerCore, path: &String) {
        let original = path;
        let path = PathBuf::from_str(&path).unwrap();
        let mut buffer = std::fs::read_to_string(path.join("project.blueproj")).expect("Unable to join path");
        self.project = serde_json::from_str(&buffer).expect("Cannot open project file");
        self.path = Some(path.clone());

        info!("Project file is opened.");
        root.publish_message("designer.project_manager", &ProjectManagerEvent::Opened { path: original.clone() });
    }

    fn create_project_directory(&mut self, root: &mut DesignerCore, path: &String, project: &Project) {
        let original = path;
        let path = PathBuf::from_str(&path).unwrap();
        let mut file = File::create(path.join("project.blueproj")).unwrap();
        let json_content = serde_json::to_string_pretty(project).expect("Cannot serialize project into file");
        file.write_all(json_content.as_bytes()).expect("Cannot write project file");

        info!("Project file is created.");
        root.publish_message("designer.project_manager", &ProjectManagerEvent::Created { path: original.clone() });
    }
}


impl ProjectManager {
    pub fn new() -> Self {
        Self { path: None, status: PluginStatus::Disabled, recent_projects: vec![], project: None, }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectManagerCommand {
    OpenProject {
        path: String
    },
    CreateProject {
        path: String,
        project: Project
    },
    CloseProject,
}

impl ICommand for ProjectManagerCommand {
    fn action<'a>(&'a self) -> &'a str {
        "designer.project_manager.management"
    }

    fn args<'a>(&'a self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectManagerEvent {
    Opened {
        path: String,
    },
    Created {
        path: String,
    }
}

impl ICommand for ProjectManagerEvent {
    fn action<'a>(&'a self) -> &'a str {
        "designer.project_manager.event"
    }

    fn args<'a>(&'a self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}
