use std::ops::Deref;

use egui::{Layout, Response, Widget};
use egui_extras::{Size, StripBuilder};
use egui_hooks::UseHookExt;

use crate::dock_manager::MyTab;

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-08 16:39:55
 * @modify date 2024-09-08 16:39:55
 * @desc [description]
*/

pub struct WelcomePage<'a, 'b> {
    pub tab: &'a mut MyTab<'b>
}
impl<'a, 'b> Widget for WelcomePage<'a, 'b> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let state = ui.use_state(|| 0.0f32, ());
        let space = ui.available_size().y - *state;
        ui.add_space(space / 2.0);
        let response = ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
            ui.heading("Welcome to BlueBird Integrated development environment!");
            ui.add_space(20.);
            ui.label("Create new project or open an existing one from upper [File] menu item.");
        }).response;
        ui.add_space(space / 2.0);
        if *state == 0. {
            state.set_next(response.interact_rect.height());
            ui.ctx().request_repaint();
        }
        response
    }
}
