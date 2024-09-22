/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-10 03:11:55
 * @modify date 2024-09-10 03:11:55
 * @desc [description]
*/

import {
    __version,
    __ui_add,
    __ui_close_menu,
    //@ts-ignore
} from "designer/egui"

export class PointerWrapper {
    constructor(protected __pointer: number) {
    }
}

export class Ui extends PointerWrapper {
    constructor(__pointer: number) {
        super(__pointer)
    }

    add(widget: Widget) {
        __ui_add(this.__pointer, widget)
    }

    closeMenu(): void {
        //implemented in rust
        __ui_close_menu(this.__pointer)
    }
}

export class UiResponse extends PointerWrapper {
    clicked: boolean = false
    secondary_clicked = false
    clicked_by: {
        primary: boolean,
        secondary: boolean,
        middle: boolean,
        extra1: boolean,
        extra2: boolean,
    } = {
        primary: false,
        secondary: false,
        middle: false,
        extra1: false,
        extra2: false,
    }
    hovered: boolean = false;
    interact_rect: {left: number, top: number, width: number, height: number} = {height: 0, left: 0, top: 0, width: 0}
    rect: {left: number, top: number, width: number, height: number} = {height: 0, left: 0, top: 0, width: 0}
    id: number = 0
    private __context_menu_rust(internal_pointer: number, add_contents_callback: (ui_pointer: number) => void) {}
    contextMenu(addContents: (ui: Ui) => void) {
        if (this.__pointer == 0) throw Error("DON'T USE UiResponse OBJECT OUTSIDE OF IT'S Render Frame, this mean is NEVER save and reuse any of UI elements, any of UI elements only valid it's current Render Frame, reuse of UI elements introduces critical UNDEFINED behaviour, it may cause program crash or other critical errors.")
        this.__context_menu_rust(this.__pointer, (ui_object_pointer) => {
            const ui = new Ui(ui_object_pointer)
            addContents(ui)
        })
    }
}

enum WidgetType {
    Null = 0,
    Button = 1,
    MenuButton = 2,
    Separator = 3,
    Layout = 4,
    Label = 5,
    CheckBox = 6,
}

export class Widget {
    __WIDGET_TYPE = -1
    public response?: (response: UiResponse) => void
    constructor(protected __widgetType: WidgetType) {
        this.__WIDGET_TYPE = __widgetType
    }
    static WIDGET_TYPE_BUTTON = 1;

    public withResponse(response: (response: UiResponse) => void) {
        this.response = response
        return this
    }

    protected __on_response_from_rust(self_object: any, response_object: any) {
        const responseInstance = new UiResponse(response_object.__pointer)
        Object.assign(responseInstance, response_object) //this is necessary because some of native methods returned by rust and definitions in this type script file is diffferent, we need to merge the object from rust side into the object in typescript side.
        self_object.response?.call(self_object, responseInstance)
        return response_object
    }
}

export class ContainerWidget extends Widget {
    constructor(__widgetType: WidgetType, public addContents: (ui: Ui) => void) {
        super(__widgetType)
    }

    protected __add_content(self_object: MenuButton, ui_pointer: number) {
        self_object.addContents.call(self_object, new Ui(ui_pointer))
    }
}

/////////////////////////////////////////

export class Button extends Widget {
    constructor(public text: string) {
        super(WidgetType.Button)
        
        this.text = text ?? "EMPTY"
    }
}

/////////////////////////////////////////
export class MenuButton extends ContainerWidget {
    constructor(public title: string, public addContents: (ui: Ui) => void) {
        super(WidgetType.MenuButton, addContents)
    }
}
/////////////////////////////////////////
export enum LayoutDirection {
    TopToBottom = 1,
    LeftToRight = 2,
    BottomToTop = 3,
    RightToLEft = 4
}
export enum Alignment {
    Top = 1,
    Center = 2,
    Bottom = 3,
    Left = 1,
    Right = 3,
}

export class Layout extends ContainerWidget {
    public constructor(addContents: (ui: Ui) => void, public direction: LayoutDirection = LayoutDirection.TopToBottom, public mainAxisAlignment: Alignment = Alignment.Top, public crossAxisAlignment: Alignment = Alignment.Left, public main_justify: boolean = false, public cross_justify: boolean = false) {
        super(WidgetType.Layout, addContents)
    }
}
/////////////////////////////////////////
export class Separator extends Widget {
    constructor(public spacing: number = 2) {
        super(WidgetType.Separator)
    }
}
/////////////////////////////////////////
export class Label extends Widget {
    constructor(public text: string) {
        super(WidgetType.Label)
    }
}
/////////////////////////////////////////
export interface BoolValue {
    value: boolean
}

export class CheckBox extends Widget {
    constructor(public text: string, public checked: BoolValue) {
        super(WidgetType.CheckBox)
    }
}

/////////////////////////////////////////
(globalThis as any)._ui_main = function(ui_object_pointer: number) {
    const ui = new Ui(ui_object_pointer);
    (globalThis as any).ui_main(ui);
}

