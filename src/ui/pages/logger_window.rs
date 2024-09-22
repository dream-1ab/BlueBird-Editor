use std::{any::Any, cell::RefCell, ops::DerefMut, rc::Rc};

use egui::{Button, ScrollArea, Vec2b, Widget};
use egui_extras::{Column, TableBuilder};
use egui_hooks::UseHookExt;

use crate::{backend::{plugin::IPlugin, plugins::{event_interceptor::SystemEventInterceptor, logger::{LogCommand, Logger}}}, dock_manager::MyTab, extensions::AnyExts, ui::javascript_widget::JavaScriptWidget, DesignerUI};

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-08 23:26:02
 * @modify date 2024-09-08 23:26:02
 * @desc [description]
*/

pub struct LoggerWindow<'a, 'b> {
    pub tab: &'a mut MyTab<'b>
}

impl<'a, 'b> Widget for LoggerWindow<'a, 'b> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let log_mode = ui.use_state(|| 0u8, ());

        ui.vertical_centered_justified(|ui| {
            let mut follow_to_bottom = ui.use_state(||true, ()).into_var();
            let mut force_follow_to_bottom = ui.use_state(||false, ()).into_var();
            if !*follow_to_bottom {
                *force_follow_to_bottom = false;
            }
            ui.horizontal(|ui|{
                // if ui.button("log gen").clicked() {
                //     self.tab.app.publish_command("Logger UI", &LogCommand::Generate {category: "xxx".into(), content: "Hello world".into()});
                // }
                ["Generic", "System", "JavaScript"].iter().enumerate().for_each(|(index, item)|{
                    if ui.selectable_label(index as u8 == *log_mode, *item).clicked() {
                        log_mode.set_next(index as u8);
                    }
                });
                ui.separator();
                ui.checkbox(follow_to_bottom.deref_mut(), "Follow to bottom");
                if *follow_to_bottom {
                    ui.checkbox(&mut force_follow_to_bottom.deref_mut(), "Always");
                }
            });
            ui.separator();
            if *log_mode == 0 {
                ui.horizontal(|ui|{
                    if ui.button("Clear").clicked() {
                        self.tab.app.publish_command("Logger UI", &LogCommand::Clear);
                    }
                });
                ui.add_space(10f32);
                let logger = self.tab.app.core.get_plugin::<Logger>().unwrap();
                TableBuilder::new(ui)
                    .auto_shrink(Vec2b::new(false, false))
                    .column(Column::exact(40f32).resizable(true))
                    .column(Column::auto().at_least(50.0f32).resizable(true))
                    .column(Column::auto().at_least(30.0f32).resizable(true))
                    .column(Column::auto().at_least(100f32).resizable(true))
                    .stick_to_bottom(*follow_to_bottom)
                    .let_self(|mut me| {
                        if *force_follow_to_bottom {
                            me = me.scroll_to_row(logger.logs.len(), None);
                        }
                        me
                    })
                    .header(20.0f32, |mut header| {
                        header.col(|ui| {
                            ui.label("Index");
                        });
                        header.col(|ui| {
                            ui.label("Sender");
                        });
                        header.col(|ui| {
                            ui.label("Category");
                        });
                        header.col(|ui| {
                            ui.label("Content");
                        });
                }).body(|mut body|{
                    body.rows(20.0f32, logger.logs.len(), |mut rows|{
                        let row_index = rows.index();
                        let (sender, category, content) = &logger.logs[row_index];
                        rows.col(|ui|{
                            ui.label(format!("{}", row_index));
                        });
                        rows.col(|ui|{
                            ui.label(sender);
                        });
                        rows.col(|ui|{
                            ui.label(category);
                        });
                        rows.col(|ui|{
                            ui.label(content);
                        });
                    });
                });
            }
            if *log_mode == 1 {
                // ui.horizontal(|ui|{
                //     if ui.button("Clear").clicked() {
                //         self.tab.app.publish_command("Logger UI", &LogCommand::Clear);
                //     }
                // });
                ui.add_space(10f32);
                let logger = self.tab.app.core.get_plugin::<SystemEventInterceptor>().unwrap();
                TableBuilder::new(ui)
                    .auto_shrink(Vec2b::new(false, false))
                    .column(Column::exact(40f32).resizable(true))
                    .column(Column::auto().at_least(100.0f32).resizable(true))
                    .column(Column::auto().at_least(60.0f32).resizable(true))
                    .column(Column::auto().at_least(100f32).resizable(true))
                    .stick_to_bottom(*follow_to_bottom)
                    .let_self(|mut me| {
                        if *force_follow_to_bottom {
                            me = me.scroll_to_row(logger.collected_events.len(), None);
                        }
                        me
                    })
                    .header(20.0f32, |mut header| {
                        header.col(|ui| {
                            ui.label("Index");
                        });
                        header.col(|ui| {
                            ui.label("Sender");
                        });
                        header.col(|ui| {
                            ui.label("Action");
                        });
                        header.col(|ui| {
                            ui.label("Aegs");
                        });
                }).body(|mut body|{
                    body.rows(20.0f32, logger.collected_events.len(), |mut rows|{
                        let row_index = rows.index();
                        let event = &logger.collected_events[row_index];
                        rows.col(|ui|{
                            ui.label(format!("{}", row_index));
                        });
                        rows.col(|ui|{
                            ui.label(&event.sender);
                        });
                        rows.col(|ui|{
                            ui.label(&event.action);
                        });
                        rows.col(|ui|{
                            ui.label(serde_json::to_string(&event.args).unwrap().as_str());
                        });
                    });
                });
            }
            if *log_mode == 2 {
                ui.add_space(10f32);
                let logger = self.tab.app.core.get_plugin::<Logger>().unwrap();
                let logs: Vec<_> = logger.logs.iter().filter(|item| item.0.as_str() == "JavaScriptEngine").collect();
                TableBuilder::new(ui)
                    .auto_shrink(Vec2b::new(false, false))
                    .column(Column::exact(40f32).resizable(true))
                    .column(Column::auto().at_least(50.0f32).resizable(true))
                    .column(Column::auto().at_least(30.0f32).resizable(true))
                    .column(Column::auto().at_least(100f32).resizable(true))
                    .stick_to_bottom(*follow_to_bottom)
                    .let_self(|mut me| {
                        if *force_follow_to_bottom {
                            me = me.scroll_to_row(logs.len(), None);
                        }
                        me
                    })
                    .header(20.0f32, |mut header| {
                        header.col(|ui| {
                            ui.label("Index");
                        });
                        header.col(|ui| {
                            ui.label("Sender");
                        });
                        header.col(|ui| {
                            ui.label("Category");
                        });
                        header.col(|ui| {
                            ui.label("Content");
                        });
                }).body(|mut body|{
                    body.rows(20.0f32, logs.len(), |mut rows|{
                        let row_index = rows.index();
                        let (sender, category, content) = &logs[row_index];
                        rows.col(|ui|{
                            ui.label(format!("{}", row_index));
                        });
                        rows.col(|ui|{
                            ui.label(sender);
                        });
                        rows.col(|ui|{
                            ui.label(category);
                        });
                        rows.col(|ui|{
                            ui.label(content);
                        });
                    });
                });
            }
        }).response
    }
}

