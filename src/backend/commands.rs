use std::{any::Any, env::args, fmt::Arguments};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-08 20:56:37
 * @modify date 2024-09-08 20:56:37
 * @desc [description]
*/


pub trait ICommand : Any {
    fn action<'a>(&'a self) -> &'a str;
    fn args<'a>(&'a self) -> Value;
}

#[derive(Debug, Clone)]
pub struct Command<T> {
    pub action: String,
    pub args: T
}

impl<T> ICommand for Command<T> where T: Serialize + 'static {
    fn action<'a>(&'a self) -> &'a str {
        self.action.as_str()
    }

    fn args<'a>(&'a self) -> Value {
        let result = serde_json::to_value(&self.args).unwrap();
        result
    }
}

impl <T> ICommand for (&'static str, T) where T: Serialize + 'static {
    fn action<'a>(&'a self) -> &'a str {
        self.0
    }

    fn args<'a>(&'a self) -> Value {
        serde_json::to_value(&self.1).unwrap()
    }
}

impl<T> From<(&str, T)> for Command<T> where T: Serialize {
    fn from(value: (&str, T)) -> Self {
        Self {
            action: value.0.to_string(),
            args: value.1
        }
    }
}

// impl<T: DeserializeOwned> From<Value> for T {
//     fn from(value: Value) -> Self {
//         serde_json::from_value(value).unwrap()
//     }
// }
