use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::backend::{commands::{Command, ICommand}, plugin::{IPlugin, PluginInfo, PluginStatus}};

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-08 23:28:43
 * @modify date 2024-09-08 23:28:43
 * @desc [description]
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logger {
    ///sender, category and content of logs.
    pub logs: Vec<(String, String, String)>,
    status: PluginStatus,
}

impl Logger {
    pub fn new() -> Self {
        Self { logs: vec![], status: PluginStatus::Disabled }
    }
}

impl IPlugin for Logger {
    fn info(&self) -> crate::backend::plugin::PluginInfo {
        PluginInfo {
            uuid: "aacd2e16-52b1-40e8-b504-0aceffd5b466",
            name: String::from("Logger (Native plugin)"),
            version: (0, 0, 1),
            author: "dream-lab",
            description: String::from("Collect logs from other plugin or extension.")
        }
    }
    fn load_state(&mut self, storage: &mut dyn crate::backend::plugin::IPluginStateStorage) {
        *self = serde_json::from_value(storage.load_state()).unwrap();
    }
    fn store_state(&mut self, storage: &mut dyn crate::backend::plugin::IPluginStateStorage) {
        
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

    fn get_state(&self) -> serde_json::Value {
        json!({
            "logs": self.logs
        })
    }

    fn handle_message(&mut self, root: &mut crate::DesignerCore, sender: &str, action: &str, args: Value) {
        if action != "designer.logger.log" {
            return;
        }
        let command: LogCommand  = serde_json::from_value(args).unwrap();
        match command {
            LogCommand::Generate { category, content } => {
                self.logs.push((sender.to_string(), category, content));
            },
            LogCommand::Clear => {
                self.logs.clear();
            }
        }
        root.notify_ui();
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogCommand {
    Generate {
        category: String,
        content: String,
    },
    Clear
}

pub enum LogType {
    Error,
    Warning,
    Into,
    Trace
}

impl ICommand for LogCommand {
    fn action<'a>(&'a self) -> &'a str {
        "designer.logger.log"
    }

    fn args<'a>(&'a self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}
