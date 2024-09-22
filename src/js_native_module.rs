use egui::{Align, Button, Id, Label, Layout, Rect, Response, Separator, Ui, Vec2};
use rquickjs::{class::{self, JsClass, Readable, Trace}, function::Args, module::ModuleDef, qjs, Array, Class, Ctx, Function, IntoAtom, IntoJs, Object, Value};

use crate::extensions::AnyExts;

/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-10 02:49:45
 * @modify date 2024-09-10 02:49:45
 * @desc [description]
*/

pub struct EguiJsModule {

}

impl EguiJsModule {
    fn _invoke_add_content_method_of_widget<'js>(widget: &Object<'js>, ctx: &Ctx<'js>, ui: &mut Ui) {
        let add_content = widget.get::<_, Function>("__add_content").unwrap();
        let mut args = Args::new(ctx.clone(), 2);
        args.push_arg(Value::from_object(widget.clone())).unwrap();
        args.push_arg(ui as *const Ui as usize).unwrap();
        add_content.call_arg::<()>(args).expect("Cannot call __add_content function");
    }
}

impl ModuleDef for EguiJsModule {
    fn declare<'js>(decl: &rquickjs::module::Declarations<'js>) -> rquickjs::Result<()> {
        decl.declare("__version").unwrap();
        decl.declare("__ui_add").unwrap();
        decl.declare("__ui_close_menu").unwrap();
        // decl.declare("UiResponse").unwrap();
        Ok(())
    }

    fn evaluate<'js>(ctx: &rquickjs::Ctx<'js>, exports: &rquickjs::module::Exports<'js>) -> rquickjs::Result<()> {
        exports.export("__version", "0.28.1").unwrap();

        exports.export("__ui_add", Function::new(ctx.clone(), |ctx: rquickjs::Ctx<'js>, ui_pointer: usize, widget_object: rquickjs::Value<'js>| {
            let widget = widget_object.as_object();
            if widget.is_none() {
                let error_value = Value::from_string(rquickjs::String::from_str(ctx.clone(), "required argument of ui.add(widget) is cannot be empty.").unwrap());
                let error = ctx.throw(error_value);
                return Err(error);
            }
            let widget = widget.unwrap();
            let ui = unsafe {
                &mut *(ui_pointer as *const Ui as *mut Ui)
            };
            let mut response: Response;// = ui.horizontal(|ui|{}).response;
            let widget_type = widget.get::<_, i32>("__WIDGET_TYPE").unwrap() as i32;
            match widget_type {
                1 => { //Button
                    let text: String = widget.get("text").unwrap();
                    response = ui.add(Button::new(text));
                }
                2 => { //Menu button
                    let text: String = widget.get("title").unwrap();
                    response = ui.menu_button(&text, |ui| {
                        Self::_invoke_add_content_method_of_widget(widget, &ctx, ui);
                    }).response;
                }
                3 => { //Separator
                    response = ui.add(Separator::default().spacing(widget.get("spacing").unwrap()));
                }
                4 => { //Layout
                    response = ui.with_layout({
                        let mut layout = Layout::top_down(Align::Min);
                        let direction = widget.get::<_, u32>("direction").unwrap();
                        if direction == 1 {
                            layout = Layout::top_down(match widget.get::<_, u32>("mainAxisAlignment").unwrap() {
                                1 => Align::Min,
                                2 => Align::Center,
                                3 => Align::Max,
                                _ => {
                                    Align::Min
                                }
                            });
                        }
                        if direction == 2 {
                            layout = Layout::left_to_right(match widget.get::<_, u32>("mainAxisAlignment").unwrap() {
                                1 => Align::Min,
                                2 => Align::Center,
                                3 => Align::Max,
                                _ => {
                                    Align::Min
                                }
                            });
                        }
                        if direction == 3 {
                            layout = Layout::bottom_up(match widget.get::<_, u32>("mainAxisAlignment").unwrap() {
                                1 => Align::Min,
                                2 => Align::Center,
                                3 => Align::Max,
                                _ => {
                                    Align::Min
                                }
                            });
                        }
                        if direction == 4 {
                            layout = Layout::right_to_left(match widget.get::<_, u32>("mainAxisAlignment").unwrap() {
                                1 => Align::Min,
                                2 => Align::Center,
                                3 => Align::Max,
                                _ => {
                                    Align::Min
                                }
                            });
                        }
                        layout = layout.with_cross_align(match widget.get::<_, u32>("crossAxisAlignment").unwrap() {
                            1 => Align::Min,
                            2 => Align::Center,
                            3 => Align::Max,
                            _ => {
                                Align::Min
                            }
                        }).with_cross_justify(widget.get::<_, bool>("cross_justify").unwrap());
                        layout.with_main_justify(widget.get::<_, bool>("main_justify").unwrap())
                    }, |ui| {
                        Self::_invoke_add_content_method_of_widget(widget, &ctx, ui);
                    }).response;
                }
                5 => { //Label
                    let text: String = widget.get("text").unwrap();
                    response = ui.add(Label::new(&text));
                }
                6 => {
                    let text = widget.get::<_, String>(("text")).unwrap();
                    let mut state = widget.get::<_, Object>("checked").expect("You must provide an object with value property, for example {value: true}.");
                    let mut value = state.get("value").unwrap_or(false);
                    let previous_value = value;
                    response = ui.checkbox(&mut value, text);
                    if value != previous_value {
                        state.set("value", value).unwrap();
                    }
                }
                _ => {
                    let json_text = ctx.json_stringify(widget.clone().as_value()).unwrap().unwrap().to_string().unwrap();
                    let error = ctx.throw(Value::from_string(rquickjs::String::from_str(ctx.clone(), format!("Rust side error: Unknown widget type is received, widget: {}", json_text).as_str()).unwrap()));
                    return Err(error);
                }
            }
            // println!("{}, {:?}, {}", ui_pointer, widget_object, widget_type);
            {
                let widget_response_callback = widget.get::<_, Function>("__on_response_from_rust")?;
                
                let mut response_wrapper = UiResponse::new_from_rust_ui_response(&mut response).into_js(&ctx)?;
                let mut args = Args::new(ctx.clone(), 2);
                args.push_arg(Value::from_object(widget.clone()))?;
                args.push_arg(response_wrapper)?;
                response_wrapper = widget_response_callback.call_arg::<Value>(args)?;
                response_wrapper.as_object().expect("msg").set("__pointer", 0)?;
            }
            return Ok::<_, rquickjs::Error>(());
        }).unwrap().with_name("__ui_add")).expect("Cannot register native functions.");
        exports.export("__ui_close_menu", Function::new(ctx.clone(), |ctx: rquickjs::Ctx<'js>, ui_pointer: usize| {
            let ui = unsafe {
                &mut *(ui_pointer as *const Ui as *mut Ui)
            };
            ui.close_menu();
        })).unwrap();
        Result::Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct UiResponse {
    pub __pointer: usize,
    pub id: Id,
    pub clicked: bool,
    secondary_clicked: bool,
    pub hovered: bool,
    pub rect: Rect,
    pub interact_rect: Rect,
    pub clicked_by_primary: bool,
    pub clicked_by_secondary: bool,
    pub clicked_by_middle: bool,
    pub clicked_by_extra1: bool,
    pub clicked_by_extra2: bool,
    // pub long_touched: bool,
    // middle_clicked
    // double_clicked
    // triple_clicked
    // double_clicked_by
    // triple_clicked_by
    // clicked_elsewhere
    // enabled
    // contains_pointer
    // has_focus
    // gained_focus
    // lost_focus
    // request_focus
    // surrender_focus
    // drag_started
    // drag_started_by
    // dragged
}

impl<'js> IntoJs<'js> for UiResponse {
    fn into_js(self, ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<Value<'js>> {
        let obj = Object::new(ctx.clone()).unwrap();
        obj.set("__pointer", self.__pointer).unwrap();
        obj.set("clicked", self.clicked).unwrap();
        obj.set("hovered", self.hovered).unwrap();
        obj.set("interact_rect", Self::rect_to_object(ctx, self.interact_rect)).unwrap();
        obj.set("rect", Self::rect_to_object(ctx, self.rect)).unwrap();
        obj.set("id", self.id.value()).unwrap();
        {
            let clicked_by = Object::new(ctx.clone()).unwrap();
            clicked_by.set("primary", self.clicked_by_primary).unwrap();
            clicked_by.set("secondary_clicked", self.secondary_clicked).unwrap();
            clicked_by.set("secondary", self.clicked_by_secondary).unwrap();
            clicked_by.set("middle", self.clicked_by_middle).unwrap();
            clicked_by.set("extra1", self.clicked_by_extra1).unwrap();
            clicked_by.set("extra2", self.clicked_by_extra2).unwrap();
            obj.set("clicked_by", Value::from_object(clicked_by)).unwrap();
        }
        obj.set("__context_menu_rust", Function::new(ctx.clone(), |ctx: Ctx<'js>, internal_pointer: usize, add_contents_js: Function<'js>|{
            let rust_ui_response = unsafe {&mut *(internal_pointer as *const Response as *mut Response)};
            // println!("content_menu_outside {}, thread_id: {:?}, context_menu_opened: {}", internal_pointer, std::thread::current().id(), rust_ui_response.context_menu_opened());

            rust_ui_response.context_menu(|ui| {
                // println!("content_menu_inside");
                let mut args = Args::new(ctx, 1);
                args.push_arg(ui as *const Ui as usize).unwrap();
                add_contents_js.call_arg::<()>(args).unwrap();
            });
        }).unwrap()).unwrap();
        return Ok(Value::from_object(obj));
    }
}

impl UiResponse {
    pub fn new_from_rust_ui_response(response: &mut Response) -> Self {
        let addr = response as *const Response as usize;
        // println!("ui_response: {}, thread_id: {:?}", addr, std::thread::current().id());
        // response.clicked_by(egui::PointerButton::Extra1)
        Self {
            __pointer: addr,
            clicked: response.clicked(),
            hovered: response.hovered(),
            interact_rect: response.interact_rect.clone(),
            rect: response.rect.clone(),
            id: response.id,
            clicked_by_primary: response.clicked_by(egui::PointerButton::Primary),
            clicked_by_middle: response.clicked_by(egui::PointerButton::Middle),
            clicked_by_secondary: response.clicked_by(egui::PointerButton::Secondary),
            clicked_by_extra1: response.clicked_by(egui::PointerButton::Extra1),
            clicked_by_extra2: response.clicked_by(egui::PointerButton::Extra2),
            secondary_clicked: response.secondary_clicked(),
        }
    }

    pub fn empty() -> Self {
        Self {
            __pointer: 0,
            clicked: false,
            hovered: false,
            interact_rect: Rect::ZERO,
            rect: Rect::ZERO,
            id: Id::NULL,
            clicked_by_extra1: false,
            clicked_by_extra2: false,
            clicked_by_middle: false,
            clicked_by_primary: false,
            clicked_by_secondary: false,
            secondary_clicked: false,
        }
    }

    fn rect_to_object<'a>(ctx: &Ctx<'a>, rect: Rect) -> Value<'a> {
        Value::from_object(Object::new(ctx.clone()).unwrap().let_self(|mut me| {
            me.set("left", rect.left()).unwrap();
            me.set("top", rect.top()).unwrap();
            me.set("width", rect.width()).unwrap();
            me.set("height", rect.height()).unwrap();
            me
        }))
    }
}

