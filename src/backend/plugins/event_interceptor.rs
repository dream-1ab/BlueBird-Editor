use std::collections::VecDeque;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::backend::plugin::{IPlugin, PluginInfo, PluginStatus};

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-09 12:00:20
 * @modify date 2024-09-09 12:00:20
 * @desc [description]
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    pub sender: String,
    pub action: String,
    pub args: Value
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEventInterceptor {
    status: PluginStatus,
    pub collected_events: VecDeque<SystemEvent>,
    maximum_log_count: u32,
}

impl SystemEventInterceptor {
    pub fn new() -> Self {
        Self { status: PluginStatus::Disabled, collected_events: VecDeque::with_capacity(512), maximum_log_count: 512 }
    }
}

impl IPlugin for SystemEventInterceptor {
    fn info(&self) -> crate::backend::plugin::PluginInfo {
        PluginInfo {
            uuid: "f9006025-8a2c-424f-b9a7-e9fb5aeddedf",
            name: String::from("System event logger (Core)"),
            description: String::from("Collects all the system events, commands to provide debuggrable feature."),
            author: "dream-lab",
            version: (0, 0, 1),
        }
    }

    fn status(&self) -> PluginStatus {
        self.status.clone()
    }

    fn initialize(&mut self, root: &mut crate::DesignerCore) {
        
    }

    fn enable(&mut self, root: &mut crate::DesignerCore) {
        self.status = PluginStatus::Enabled;
    }

    fn disable(&mut self, root: &mut crate::DesignerCore) {
        self.status = PluginStatus::Disabled;
    }

    fn get_state(&self) -> serde_json::Value {
        serde_json::to_value(&self).unwrap()
    }

    fn load_state(&mut self, storage: &mut dyn crate::backend::plugin::IPluginStateStorage) {
        *self = serde_json::from_value(storage.load_state()).unwrap();
    }

    fn store_state(&mut self, storage: &mut dyn crate::backend::plugin::IPluginStateStorage) {
        storage.store_state(serde_json::to_value(self).unwrap());
    }

    fn handle_message(&mut self, root: &mut crate::DesignerCore, sender: &str, action: &str, args: Value) {
        self.collected_events.push_back(SystemEvent { sender: sender.to_string(), action: action.to_string(), args: args });
        if self.collected_events.len() > self.maximum_log_count as usize {
            self.collected_events.pop_front();
        }
        root.notify_ui();
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}