use std::{cell::{RefCell, UnsafeCell}, collections::HashMap, error::Error, marker::PhantomData, rc::Rc, sync::Arc};

use egui::{mutex::Mutex, Ui, Widget};
use egui_hooks::UseHookExt;
use rquickjs::{context::EvalOptions, function::{Args, RustFunction}, CatchResultExt, Ctx, FromJs, Function, Module, Runtime};
use serde_json::Value;

use crate::{backend::plugins::logger::{LogCommand, Logger}, js_native_module::UiResponse, DesignerUI};

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-09 17:16:54
 * @modify date 2024-09-09 17:16:54
 * @desc [description]
*/

pub struct JavaScriptWidget<'c> {
    source_code: String,
    name: String,
    app: &'c mut DesignerUI
}

impl<'a, 'c> JavaScriptWidget<'c> {
    pub fn new(source_code: String, name: String, app: &'c mut DesignerUI) -> Self {
        Self {source_code: source_code, name, app }
    }
}

impl<'a> Widget for JavaScriptWidget<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {

        let context = ui.use_state(|| {
            let source = self.source_code.clone();
            let context = Box::new(rquickjs::context::Context::full(&self.app.js_engine).expect("Cannot create JavaScript context."));
            let mut context = MyJsContextWrapper::new(context);
            let mr = unsafe {
                let pointer = self.app as *const DesignerUI as *mut DesignerUI;
                let mutable_reference = &mut *pointer;
                mutable_reference
            };
            mr.initialize_js_engine();
            let name = self.name.clone();
            let initialized = context.initialize(&source, mr, name);
            let context = Arc::new(context);
            (context, initialized)
        }, self.source_code.to_string());
        match &context.1 {
            Ok(_) => {
                // match context.0.execute(format!("import {{Ui}} from \"designer\"\n ui_main(new Ui({}))", ui as *const Ui as usize).as_str(), &mut self.app, self.name.clone()) {
                match context.0.call_ui_main(ui as *const Ui as usize, self.app) {
                    Ok(value) => {
                        // println!("{}", serde_json::to_string_pretty(&value).unwrap());
                    },
                    Err(error) => {
                        ui.heading(&error);
                    },
                }
            },
            Err(error) => {
                ui.label(format!("Initializing JavaScript Engine Error:\n {}", error));
            },
        }
        ui.vertical(|ui|{}).response
    }
}

struct MyJsContextWrapper {
    pointer: usize,
}

impl MyJsContextWrapper {
    pub fn new(obj: Box<rquickjs::Context>) -> Self {
        Self { pointer: Box::into_raw(obj) as usize }
    }
    pub fn use_context<T>(&self, user: impl FnOnce(&Box<rquickjs::Context>) -> T) -> T {
        let obj = unsafe {
            Box::from_raw(self.pointer as *mut rquickjs::Context)
        };
        let result = user(&obj);
        std::mem::forget(obj);
        result
    }
}

impl Drop for MyJsContextWrapper {
    fn drop(&mut self) {
        let will_be_dropped = unsafe {
            Box::from_raw(self.pointer as *mut rquickjs::Context)
        };
    }
}

impl MyJsContextWrapper {
    fn execute(&self, source: &str, app: & mut DesignerUI, name: String) -> Result<Value, String> {
        self.use_context(|context| {
            context.with(|ctx| {
                // let global = ctx.globals();
                // let global_value = SerdeJsonQuickJsValue::from_js(&ctx, global.as_value().clone()).unwrap().value;
                // println!("global: {:?}", global_value);
                // let result: Result<rquickjs::Value, rquickjs::Error> = ctx.eval_with_options::<rquickjs::Value, _>(source, Default::default());
                // let result = ctx.eval_with_options::<rquickjs::Value, _>(source, Default::default());
                let result = Module::evaluate(ctx.clone(), name.clone(), source).unwrap().finish();
                // println!("{:?}", result);
                // let result: Result<rquickjs::Value, rquickjs::Error> = Module::evaluate(ctx.clone(), name.clone(), source).expect("Cannot load javascript module").finish();
                match result {
                    Ok(result) => {
                        Ok(SerdeJsonQuickJsValue::from_js(&ctx, result).unwrap().value)
                    },
                    Err(error) => {
                        let error_message = Self::quicksj_error_to_string(&ctx, error);
                        app.publish_command("JavaScriptEngine", &LogCommand::Generate { category: "UI".to_string(), content: error_message.clone() });
                        Err(error_message)
                    },
                }
            })
        })
    }

    fn initialize(&mut self, source: &str, app: &'static mut DesignerUI, name: String) -> Result<(), String> {
        self.use_context(|context| {
            context.with(|ctx| {
                let app = app as *const DesignerUI as *mut DesignerUI;
                let global = ctx.globals();
                global.set("print", Function::new(ctx.clone(), move |args: SerdeJsonQuickJsValue|{
                    let value = args.value;
                    let app = unsafe {&mut *app};
                    let command = LogCommand::Generate { category: "UI".to_string(), content: serde_json::to_string(&value).unwrap() };
                    app.publish_command("JavaScriptEngine", &command);
                })).expect("Cannot register print function into javascript context");
                // let result: Result<(), rquickjs::Error> = ctx.eval_with_options(source, Default::default());
                // let result: Result<(), rquickjs::Error> = Module::evaluate(ctx.clone(), name.clone(), source).expect("Cannot load javascript module").finish();
                let result = Module::evaluate(ctx.clone(), name.clone(), source);
                match result {
                    Ok(promise) => {
                        let app = unsafe {&mut *app};
                        match promise.finish::<()>() {
                            Ok(r) => {
                                app.publish_command("JavaScriptEngine", &LogCommand::Generate { category: "UI".to_string(), content: format!("Initialization of JavaScript engine on Widget {} succeed with result", name) });
                                Ok(())
                            },
                            Err(error) => {
                                let error = Err::<(), _>(error).catch(&ctx).unwrap_err();
                                let error_message = format!("{:?}", error);
                                // let mut error_message = serde_json::to_string(&SerdeJsonQuickJsValue::from_js(&ctx, ctx.catch()).unwrap().value).unwrap();
                                app.publish_command("JavaScriptEngine", &LogCommand::Generate { category: "UI".to_string(), content: format!("Executing javascript on Widget {} ERROR: {}", name, format!("{}", error_message)) });
                                Err(error_message)
                            },
                        }
                        // println!("Initialized.");
                    },
                    Err(error) => {
                        let error_message = Self::quicksj_error_to_string(&ctx, error);
                        let app = unsafe {&mut *app};
                        app.publish_command("JavaScriptEngine", &LogCommand::Generate { category: "UI".to_string(), content: format!("Initialization of JavaScript engine on Widget {} ERROR: {}", name, format!("{}", error_message)) });
                        Err(error_message)
                    },
                }
            })
        })
    }

    fn call_ui_main(&self, ui_address: usize, app: & mut DesignerUI) -> Result<(), String> {
        let result = self.use_context(|context|{
            context.with(|ctx|{
                let result = ctx.globals().get::<_, Function>("_ui_main");
                if let Ok(ui_main) = result {
                    let mut args = Args::new(ctx.clone(), 1);
                    args.push_arg(ui_address).unwrap();
                    match ui_main.call_arg::<()>(args) {
                        Ok(result) => {
                            Ok(result)
                        },
                        Err(error) => {
                            Err(Self::quicksj_error_to_string(&ctx, error))
                        },
                    }
                } else {
                    Err("ui_main function is not defined on global (globalThis variable) object.".to_string())
                }
            })
        });

        match result {
            Ok(result) => {
                Ok(())
            },
            Err(error) => {
                app.publish_command("JavaScriptEngine", &LogCommand::Generate { category: "UI".to_string(), content: format!("Calling ui_main error: {}", format!("{}", error)) });
                Err(error)
            },
        }
    }

    fn quicksj_error_to_string(ctx: &Ctx, error: rquickjs::Error) -> String {
        let error = Err::<(), _>(error).catch(&ctx).unwrap_err();
        let error_message = format!("{:?}", error);
        error_message
    }
}

pub struct SerdeJsonQuickJsValue {
    value: serde_json::Value
}

impl<'js> FromJs<'js> for SerdeJsonQuickJsValue {
    fn from_js(ctx: &rquickjs::Ctx<'js>, value: rquickjs::Value<'js>) -> rquickjs::Result<Self> {
        let mut _value = serde_json::Value::Null;
        if value.is_string() {
            _value = serde_json::Value::String(value.as_string().unwrap().to_string().unwrap());
        }
        if value.is_bool() {
            _value = serde_json::Value::Bool(value.as_bool().unwrap());
        }
        if value.is_number() {
            _value = serde_json::Value::Number(serde_json::Number::from_f64(value.as_number().unwrap()).unwrap());
        }
        if value.is_int() {
            _value = serde_json::Value::Number(serde_json::Number::from(value.as_int().unwrap()));
        }
        if value.is_float() {
            _value = serde_json::Value::Number(serde_json::Number::from_f64(value.as_float().unwrap()).unwrap());
        }
        if value.is_int() {
            _value = serde_json::Value::Number(serde_json::Number::from(value.as_int().unwrap()));
        }
        if value.is_null() || value.is_undefined() {
            _value = serde_json::Value::Null;
        }
        if value.is_function() {
            _value = serde_json::Value::String("js_function".to_string());
        }
        if value.is_object() {
            let obj = value.as_object().unwrap();
            let mut map = serde_json::Map::new();
            obj.props::<String, SerdeJsonQuickJsValue>().for_each(|item| {
                let (key, value) = item.unwrap();
                map.insert(key, value.value);
            });
            _value = serde_json::Value::Object(map);
        }
        if value.is_exception() {
            let obj = value.as_exception().unwrap();
            let mut map = serde_json::Map::new();
            obj.props::<String, SerdeJsonQuickJsValue>().for_each(|item| {
                let (key, value) = item.unwrap();
                map.insert(key, value.value);
            });
            _value = serde_json::Value::Object(map);
        }
        if value.is_array() {
            let mut array = vec![];
            let obj = value.as_array().unwrap();
            for i in 0..obj.len() {
                let item = obj.get(i).unwrap();
                array.push(SerdeJsonQuickJsValue::from_js(ctx, item).unwrap().value);
            }
            _value = serde_json::Value::Array(array);
        }
        Ok(Self {value: _value})
    }
}
