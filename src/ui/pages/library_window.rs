use egui::Widget;
use egui_hooks::UseHookExt;

use crate::{dock_manager::MyTab, ui::javascript_widget::JavaScriptWidget};

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-10 06:12:50
 * @modify date 2024-09-10 06:12:50
 * @desc [description]
*/


pub struct LibraryWindow<'a, 'b> {
    pub tab: &'a mut MyTab<'b>
}

impl<'a, 'b> Widget for LibraryWindow<'a, 'b> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let source = ui.use_state(||{
            std::fs::read_to_string("./assets/app.js").unwrap()
        }, ());
        ui.add(JavaScriptWidget::new((*source).clone(), "JsView".to_string(), self.tab.app));
        ui.label("JavaScript widget")
    }
}
