use egui::Widget;
use egui_dock::TabViewer;
use egui_hooks::UseHookExt;
use serde::{Deserialize, Serialize};

use crate::{backend::plugins::window_manager::WindowManager, ui::pages::{file_manager::FileManagerPage, library_window::LibraryWindow, logger_window::LoggerWindow, welcome_page::WelcomePage}, DesignerUI};

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-08 15:26:45
 * @modify date 2024-09-08 15:26:45
 * @desc [description]
*/


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EditorWindowState {
    WelcomePage {

    },
    FileManagerWindow,
    LogViewWindow {
        label: String
    },
    InspectorWindow {
        object: String
    },
    LibraryWindow {

    },
    ProjectSettingsWindow {

    },
    EditorSettingsWindow {

    },
    ExtensionsWindow {

    },
    NodeEditorWindow {
        file_name: String
    },
    CodeEditorWindow {
        file_name: String
    }
}

pub struct MyTab<'a> {
    pub app: &'a mut DesignerUI
}

impl<'a> TabViewer for MyTab<'a> {
    type Tab = EditorWindowState;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        let label = match tab {
            EditorWindowState::WelcomePage {  } => "Welcome".into(),
            EditorWindowState::FileManagerWindow => "File manager".to_string(),
            EditorWindowState::LogViewWindow { label } => format!("Log viewer ({})", label),
            EditorWindowState::InspectorWindow { object } => format!("Inspector ({})", object),
            EditorWindowState::LibraryWindow {  } => "Libraries".to_string(),
            EditorWindowState::ProjectSettingsWindow {  } => "Project settings".into(),
            EditorWindowState::EditorSettingsWindow {  } => "Preferences".into(),
            EditorWindowState::ExtensionsWindow {  } => "Extensions".into(),
            EditorWindowState::NodeEditorWindow { file_name } => format!("{}", file_name),
            EditorWindowState::CodeEditorWindow { file_name } => format!("{}", file_name),
        };
        egui::WidgetText::from(label)
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {

        match tab {
            EditorWindowState::WelcomePage {  } => {
                ui.add(WelcomePage {tab: self});
            },
            EditorWindowState::FileManagerWindow => {
                ui.add(FileManagerPage{tab: self});
            },
            EditorWindowState::LogViewWindow { label } => {
                ui.add(LoggerWindow {tab: self});
            },
            EditorWindowState::InspectorWindow { object } => {

            },
            EditorWindowState::LibraryWindow {  } => {
                ui.add(LibraryWindow {tab: self});
            },
            EditorWindowState::ProjectSettingsWindow {  } => {

            },
            EditorWindowState::EditorSettingsWindow {  } => {

            },
            EditorWindowState::ExtensionsWindow {  } => {

            },
            EditorWindowState::NodeEditorWindow { file_name } => {

            },
            EditorWindowState::CodeEditorWindow { file_name } => {

            },
        }
    }
}
