use std::{any::Any, cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{DesignerCore};

use super::commands::ICommand;

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-08 21:15:30
 * @modify date 2024-09-08 21:15:30
 * @desc [description]
*/

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginStatus {
    Enabled,
    Disabled
}

pub trait IPluginStateStorage {
    fn store_state(&mut self, value: Value);
    fn load_state(&mut self) -> Value;
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginInfo {
    pub uuid: &'static str,
    pub name: String,
    pub description: String,
    pub version: (usize, usize, usize),
    pub author: &'static str,
}

pub trait IPlugin: Any {
    fn info(&self) -> PluginInfo;
    fn status(&self) -> PluginStatus;
    fn initialize(&mut self, root: &mut DesignerCore);
    fn enable(&mut self, root: &mut DesignerCore);
    fn disable(&mut self, root: &mut DesignerCore);
    fn load_state(&mut self, storage: &mut dyn IPluginStateStorage);
    fn store_state(&mut self, storage: &mut dyn IPluginStateStorage);
    fn get_state(&self) -> Value;
    fn handle_message(&mut self, root: &mut DesignerCore, sender: &str, action: &str, args: Value);
    fn as_any(&self) -> &dyn Any;
}
