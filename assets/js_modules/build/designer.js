/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-10 03:11:55
 * @modify date 2024-09-10 03:11:55
 * @desc [description]
*/
import { __ui_add, __ui_close_menu,
//@ts-ignore
 } from "designer/egui";
export class PointerWrapper {
    constructor(__pointer) {
        this.__pointer = __pointer;
    }
}
export class Ui extends PointerWrapper {
    constructor(__pointer) {
        super(__pointer);
    }
    add(widget) {
        __ui_add(this.__pointer, widget);
    }
    closeMenu() {
        //implemented in rust
        __ui_close_menu(this.__pointer);
    }
}
export class UiResponse extends PointerWrapper {
    constructor() {
        super(...arguments);
        this.clicked = false;
        this.secondary_clicked = false;
        this.clicked_by = {
            primary: false,
            secondary: false,
            middle: false,
            extra1: false,
            extra2: false,
        };
        this.hovered = false;
        this.interact_rect = { height: 0, left: 0, top: 0, width: 0 };
        this.rect = { height: 0, left: 0, top: 0, width: 0 };
        this.id = 0;
    }
    __context_menu_rust(internal_pointer, add_contents_callback) { }
    contextMenu(addContents) {
        if (this.__pointer == 0)
            throw Error("DON'T USE UiResponse OBJECT OUTSIDE OF IT'S Render Frame, this mean is NEVER save and reuse any of UI elements, any of UI elements only valid it's current Render Frame, reuse of UI elements introduces critical UNDEFINED behaviour, it may cause program crash or other critical errors.");
        this.__context_menu_rust(this.__pointer, (ui_object_pointer) => {
            const ui = new Ui(ui_object_pointer);
            addContents(ui);
        });
    }
}
var WidgetType;
(function (WidgetType) {
    WidgetType[WidgetType["Null"] = 0] = "Null";
    WidgetType[WidgetType["Button"] = 1] = "Button";
    WidgetType[WidgetType["MenuButton"] = 2] = "MenuButton";
    WidgetType[WidgetType["Separator"] = 3] = "Separator";
    WidgetType[WidgetType["Layout"] = 4] = "Layout";
    WidgetType[WidgetType["Label"] = 5] = "Label";
    WidgetType[WidgetType["CheckBox"] = 6] = "CheckBox";
})(WidgetType || (WidgetType = {}));
export class Widget {
    constructor(__widgetType) {
        this.__widgetType = __widgetType;
        this.__WIDGET_TYPE = -1;
        this.__WIDGET_TYPE = __widgetType;
    }
    withResponse(response) {
        this.response = response;
        return this;
    }
    __on_response_from_rust(self_object, response_object) {
        const responseInstance = new UiResponse(response_object.__pointer);
        Object.assign(responseInstance, response_object); //this is necessary because some of native methods returned by rust and definitions in this type script file is diffferent, we need to merge the object from rust side into the object in typescript side.
        self_object.response?.call(self_object, responseInstance);
        return response_object;
    }
}
Widget.WIDGET_TYPE_BUTTON = 1;
export class ContainerWidget extends Widget {
    constructor(__widgetType, addContents) {
        super(__widgetType);
        this.addContents = addContents;
    }
    __add_content(self_object, ui_pointer) {
        self_object.addContents.call(self_object, new Ui(ui_pointer));
    }
}
/////////////////////////////////////////
export class Button extends Widget {
    constructor(text) {
        super(WidgetType.Button);
        this.text = text;
        this.text = text ?? "EMPTY";
    }
}
/////////////////////////////////////////
export class MenuButton extends ContainerWidget {
    constructor(title, addContents) {
        super(WidgetType.MenuButton, addContents);
        this.title = title;
        this.addContents = addContents;
    }
}
/////////////////////////////////////////
export var LayoutDirection;
(function (LayoutDirection) {
    LayoutDirection[LayoutDirection["TopToBottom"] = 1] = "TopToBottom";
    LayoutDirection[LayoutDirection["LeftToRight"] = 2] = "LeftToRight";
    LayoutDirection[LayoutDirection["BottomToTop"] = 3] = "BottomToTop";
    LayoutDirection[LayoutDirection["RightToLEft"] = 4] = "RightToLEft";
})(LayoutDirection || (LayoutDirection = {}));
export var Alignment;
(function (Alignment) {
    Alignment[Alignment["Top"] = 1] = "Top";
    Alignment[Alignment["Center"] = 2] = "Center";
    Alignment[Alignment["Bottom"] = 3] = "Bottom";
    Alignment[Alignment["Left"] = 1] = "Left";
    Alignment[Alignment["Right"] = 3] = "Right";
})(Alignment || (Alignment = {}));
export class Layout extends ContainerWidget {
    constructor(addContents, direction = LayoutDirection.TopToBottom, mainAxisAlignment = Alignment.Top, crossAxisAlignment = Alignment.Left, main_justify = false, cross_justify = false) {
        super(WidgetType.Layout, addContents);
        this.direction = direction;
        this.mainAxisAlignment = mainAxisAlignment;
        this.crossAxisAlignment = crossAxisAlignment;
        this.main_justify = main_justify;
        this.cross_justify = cross_justify;
    }
}
/////////////////////////////////////////
export class Separator extends Widget {
    constructor(spacing = 2) {
        super(WidgetType.Separator);
        this.spacing = spacing;
    }
}
/////////////////////////////////////////
export class Label extends Widget {
    constructor(text) {
        super(WidgetType.Label);
        this.text = text;
    }
}
export class CheckBox extends Widget {
    constructor(text, checked) {
        super(WidgetType.CheckBox);
        this.text = text;
        this.checked = checked;
    }
}
/////////////////////////////////////////
globalThis._ui_main = function (ui_object_pointer) {
    const ui = new Ui(ui_object_pointer);
    globalThis.ui_main(ui);
};
