use std::{any::{Any, TypeId}, cell::RefCell, collections::VecDeque, ffi::OsStr, ops::Deref, path::PathBuf, rc::Rc};

use backend::{commands::ICommand, plugin::{IPlugin, PluginStatus}, plugins::{event_interceptor::SystemEventInterceptor, logger::Logger, project_manager::ProjectManager, window_manager::{WindowManager, WindowManagerCommand}}};
use dock_manager::{EditorWindowState, MyTab};
use eframe::{App, CreationContext, Frame};
use egui::{mutex::Mutex, CentralPanel, Color32, Label, Margin, SidePanel, Stroke, TopBottomPanel};
use egui_dock::{DockArea, DockState, TabViewer};
use egui_file_dialog::FileDialog;
use egui_hooks::UseHookExt;
use js_native_module::EguiJsModule;
use log::{info, warn};
use project_window::{ProjectManagerWindow};
use rquickjs::loader::{BuiltinLoader, BuiltinResolver, FileResolver, ModuleLoader, ScriptLoader};

mod dock_manager;
mod ui;
mod backend;
mod project_window;
mod js_native_module;
mod extensions;

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-08 03:21:26
 * @modify date 2024-09-08 03:21:26
 * @desc [description]
*/

fn main() {
    env_logger::init();
    let options = eframe::NativeOptions {
        centered: true,
        run_and_return: false,
        persist_window: true,
        window_builder: Some(Box::new(|builder| {
            builder.with_inner_size((1600f32, 800f32))
        })),
        ..Default::default()
    };

    eframe::run_native("bluebird-ide", options, Box::new(|context| {
        let mut core = DesignerCore::new(context);
        core.initialize_plugins();
        Ok(Box::new(DesignerUI::new(core)))
    })).unwrap();
    println!("Bye!");
}

pub struct DesignerCore {
    plugins: Vec<Rc<RefCell<dyn IPlugin>>>,
    notify_ui: Box<dyn Fn() -> ()>,
    message_loop_busy: bool,
    message_queue: VecDeque<(String, String, serde_json::Value)>,
}

impl DesignerCore {
    pub fn new(context: &CreationContext) -> Self {
        let context = context.egui_ctx.clone();
        Self {
            notify_ui: Box::new(move ||{
                context.request_repaint();
            }),
            plugins: vec![
                Rc::new(RefCell::new(backend::plugins::event_interceptor::SystemEventInterceptor::new())),
                Rc::new(RefCell::new(backend::plugins::logger::Logger::new())),
                Rc::new(RefCell::new(backend::plugins::project_manager::ProjectManager::new())),
                Rc::new(RefCell::new(backend::plugins::window_manager::WindowManager::new())),
                Rc::new(RefCell::new(backend::plugins::file_manager::FileManager::new())),
            ],
            message_loop_busy: false,
            message_queue:VecDeque::new(),
        }
    }

    pub fn initialize_plugins(&mut self) {
        self.plugins.clone().iter().for_each(|plugin| {
            let mut borrows = plugin.borrow_mut();
            borrows.initialize(self);
            borrows.enable(self);
        });
    }

    pub fn publish_message(&mut self, sender: &str, message: &dyn ICommand) {
        self.message_queue.push_back((sender.to_string(), message.action().to_string(), message.args()));
        // println!("+++{}, {}, {}\n", self.message_queue.len(), message.action(), message.args());
        self.publish_message_from_queue();
    }

    fn publish_message_from_queue(&mut self) {
        if self.message_loop_busy {
            return;
        }
        self.message_loop_busy = true;
        loop {
            if let Some((sender, action, args)) = self.message_queue.pop_front() {
                {
                    // println!("-{}, {}, {}", self.message_queue.len(), action, args);
                    self.plugins.clone().iter().for_each(|plugin| {
                        let mut borrows = plugin.borrow_mut();
                        if borrows.status() == PluginStatus::Enabled {
                            let name = borrows.info().name;
                            // println!("{} >> {}", action, name);
                            borrows.handle_message(self, sender.as_str(), &action, args.clone());
                        }
                    });
                }
            } else {
                break;
            }
        }
        self.message_loop_busy = false;
    }

    pub fn notify_ui(&self) {
        self.notify_ui.deref()();
    }

    pub fn get_plugin<T: IPlugin>(&self) -> Option<&T> {
        /// Very careful there, program may crash of the plugin reference returned by this function is misused, so don't store the reference, request by your needs instead.
        /// but hopefully plugin instances are stored in editor-state and the editor-state is lives long as until program exits so it's much much safe to use instances,
        /// but when the program architecture changes may cause the core editor-state is shorter than the program lifetime.
        let plugins = self.plugins.clone();
        for i in plugins {
            let any_type = unsafe {
                (&*i.as_ptr()).as_any()
            };
            if let Some(x) = any_type.downcast_ref::<T>() {
                return Some(x);
            }
        }
        None
    }
}

pub struct DesignerUI {
    dock_state: Rc<Mutex<DockState<EditorWindowState>>>,
    file_dialog: FileDialog,
    project_manager: ProjectManagerWindow,
    recent_projects: Vec<String>,
    core: DesignerCore,
    js_engine: rquickjs::Runtime,
}

impl DesignerUI {
    fn new(core: DesignerCore) -> Self {
        let mut app = DesignerUI {
            dock_state: Rc::new(Mutex::new(DockState::new(vec![]))),
            file_dialog: FileDialog::new(),
            project_manager: ProjectManagerWindow::new(),
            recent_projects: vec![],
            core,
            js_engine: rquickjs::Runtime::new().expect("Cannot initialize JavaScript engine."),
        };
        app.initialize_app();
        app
    }

    fn initialize_app(&mut self) {
        self.initialize_tabs();
        self.initialize_js_engine();
    }

    fn initialize_tabs(&mut self) {
        let mut dock_guard = self.dock_state.lock();
        let dock = &mut *dock_guard;
        dock.push_to_first_leaf(EditorWindowState::WelcomePage {});
        dock.push_to_first_leaf(EditorWindowState::FileManagerWindow);
        dock.push_to_first_leaf(EditorWindowState::InspectorWindow { object: "Empty".into() });
        dock.push_to_first_leaf(EditorWindowState::LogViewWindow { label: "Empty".into() });
        dock.push_to_first_leaf(EditorWindowState::LibraryWindow {  });
        // dock.push_to_first_leaf(EditorWindowState::EditorSettingsWindow {  });
        dock.push_to_first_leaf(EditorWindowState::ProjectSettingsWindow {  });
        dock.push_to_first_leaf(EditorWindowState::ExtensionsWindow {  });
    }

    fn initialize_js_engine(&mut self) {
        let resolver = (
            BuiltinResolver::default()
                .with_module("designer")
                .with_module("designer/egui"),
            FileResolver::default()
                // .with_path("")
        );
        let designer_module_source = include_str!("../assets/js_modules/build/designer.js");
        let loader = (
            BuiltinLoader::default().with_module("designer", designer_module_source),
            ModuleLoader::default().with_module("designer/egui", EguiJsModule {}),
            ScriptLoader::default(),
        );
        self.js_engine.set_loader(resolver, loader)
    }

    fn reset_file_dialog(&mut self) {
        self.file_dialog = FileDialog::new().as_modal(true).initial_directory(dirs::home_dir().unwrap()).directory_separator("/").default_file_filter("BlueBird Project file").add_file_filter("BlueBird Project file", std::sync::Arc::new(|path|{
            // println!("{:?}", path);
            path.extension().unwrap_or(OsStr::new("")) == "blueproj"
        }))
    }

    fn store_recent_files(&mut self, frame: &mut Frame, path: &PathBuf) {
        if self.recent_projects.iter().any(|item| item.as_str() == path.to_str().unwrap()) {
            return;
        }
        self.recent_projects.push(path.to_str().unwrap().to_string());
        self.write_recent_opened_files_into_storage(frame);
    }

    fn write_recent_opened_files_into_storage(&mut self, frame: &mut Frame) {
        let projects = serde_json::to_string(&self.recent_projects).unwrap();
        if let Some(storage) = frame.storage_mut() {
            storage.set_string("recently_opened", projects);
        } else {
            warn!("App storage is not available so cannot save recent project list.");
        }
    }

    fn clear_recents(&mut self, frame: &mut Frame) {
        self.recent_projects.clear();
        self.write_recent_opened_files_into_storage(frame);
    }

    fn load_recent_files(&mut self, frame: &mut Frame) {
        if self.recent_projects.is_empty() {
            frame.storage_mut().inspect(|storage| {
                self.recent_projects = serde_json::from_str(&storage.get_string("recently_opened").or(Some("[]".into())).unwrap()).expect("Cannot parse recently opened projects.");
            });
        }
    }

    fn publish_command(&mut self, sender: &str, message: &dyn ICommand) {
        self.core.publish_message(sender, message);
    }
}


impl App for DesignerUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.load_recent_files(frame);
        TopBottomPanel::top("top").show_separator_line(false).show(ctx, |ui|{
            ui.use_state(||0u32, ());

            egui::menu::bar(ui, |ui|{
                ui.menu_button("Project", |ui| {
                    if ui.button("New").clicked() {
                        self.reset_file_dialog();
                        self.file_dialog.open(egui_file_dialog::DialogMode::SelectDirectory, true, Some("create_project_files")).unwrap();
                        ui.close_menu();
                    }
                    if ui.button("Open").clicked() {
                        self.reset_file_dialog();
                        self.file_dialog.open(egui_file_dialog::DialogMode::SelectFile, true, Some("open_project_files")).unwrap();
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {

                    }
                    ui.menu_button("Open recent", |ui| {
                        if self.recent_projects.is_empty() {
                            ui.label("No recent projects opened.");
                        }
                        self.recent_projects.clone().iter().for_each(|path| {
                            if ui.button(path).clicked() {
                                ProjectManagerWindow::open_project(self, &PathBuf::from(path));
                                ui.close_menu();
                            }
                            ui.separator();
                        });
                        if ui.button("clear").clicked() {
                            self.clear_recents(frame);
                        }
                    });
                    // if (ui.button("Close and exit")).clicked() {
                    //     self.quit_app = true;
                    // }
                });
                // ui.separator();
                ui.menu_button("Edit", |ui|{
                    if ui.button("Undo").clicked() {
    
                    }
                    if ui.button("Redo").clicked() {
    
                    }
                    ui.separator();
                    if ui.button("Preference").clicked() {
    
                    }
                });
                // ui.separator();
                ui.menu_button("Window", |ui| {
                    ui.menu_button("Layout", |ui| {
                        let project_manager = self.core.get_plugin::<ProjectManager>().unwrap();
                        if project_manager.project_is_available() {
                            if ui.button("Save to project").clicked() {
                                // ProjectManagerWindow::store_dock_state(self);
                                let state = {
                                    let guard = self.dock_state.lock();
                                    serde_json::to_value(&*guard).unwrap()
                                };
                                self.publish_command("MainWindow", &WindowManagerCommand::SaveDockState(state));
                                ui.close_menu();
                            }
                            ui.button("Load from file");
                        } else {
                            ui.label("Please open/or create a project first.");
                        }
                        ui.button("Reset");
                    });
                });
                // ui.separator();
                ui.menu_button("Developer", |ui|{
                    ui.button("Inspect JavaScript objects in Engine")
                });
            });
        });
        CentralPanel::default().show(ctx, |ui|{
            let dock_state = self.dock_state.clone();
            let mut dock_guard = dock_state.lock();
            {
                //restore dock layout from opened project.
                let dock_changes = ui.use_state(|| 0usize, ());
                let wm = self.core.get_plugin::<WindowManager>().unwrap();
                if *dock_changes != wm.dock_changes.count && wm.dock_state.is_some() {
                    *dock_guard = serde_json::from_value(wm.dock_state.clone().unwrap()).unwrap();
                    dock_changes.set_next(wm.dock_changes.count);
                }
            }
            DockArea::new(&mut dock_guard).show(ctx, &mut MyTab{app: self});
        });

        if let Some(path) = self.file_dialog.update(ctx).selected().and_then(|p| Some(p.to_path_buf())) {
            if let Some(id) = self.file_dialog.operation_id().map(|id| id.to_string()) {
                if id == "create_project_files" {
                    self.project_manager.create_project(path.clone());
                    self.store_recent_files(frame, &path);
                }
    
                if id == "open_project_files" {
                    let path = path.parent().unwrap().into();
                    ProjectManagerWindow::open_project(self, &path);
                    self.store_recent_files(frame, &path);
                }
                self.reset_file_dialog();
            }
        }

        ProjectManagerWindow::update(self, ctx, frame);
    }
}



