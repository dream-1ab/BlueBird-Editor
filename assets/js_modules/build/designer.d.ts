/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2024-09-10 03:11:55
 * @modify date 2024-09-10 03:11:55
 * @desc [description]
*/
export declare class PointerWrapper {
    protected __pointer: number;
    constructor(__pointer: number);
}
export declare class Ui extends PointerWrapper {
    constructor(__pointer: number);
    add(widget: Widget): void;
    closeMenu(): void;
}
export declare class UiResponse extends PointerWrapper {
    clicked: boolean;
    secondary_clicked: boolean;
    clicked_by: {
        primary: boolean;
        secondary: boolean;
        middle: boolean;
        extra1: boolean;
        extra2: boolean;
    };
    hovered: boolean;
    interact_rect: {
        left: number;
        top: number;
        width: number;
        height: number;
    };
    rect: {
        left: number;
        top: number;
        width: number;
        height: number;
    };
    id: number;
    private __context_menu_rust;
    contextMenu(addContents: (ui: Ui) => void): void;
}
declare enum WidgetType {
    Null = 0,
    Button = 1,
    MenuButton = 2,
    Separator = 3,
    Layout = 4,
    Label = 5,
    CheckBox = 6
}
export declare class Widget {
    protected __widgetType: WidgetType;
    __WIDGET_TYPE: number;
    response?: (response: UiResponse) => void;
    constructor(__widgetType: WidgetType);
    static WIDGET_TYPE_BUTTON: number;
    withResponse(response: (response: UiResponse) => void): this;
    protected __on_response_from_rust(self_object: any, response_object: any): any;
}
export declare class ContainerWidget extends Widget {
    addContents: (ui: Ui) => void;
    constructor(__widgetType: WidgetType, addContents: (ui: Ui) => void);
    protected __add_content(self_object: MenuButton, ui_pointer: number): void;
}
export declare class Button extends Widget {
    text: string;
    constructor(text: string);
}
export declare class MenuButton extends ContainerWidget {
    title: string;
    addContents: (ui: Ui) => void;
    constructor(title: string, addContents: (ui: Ui) => void);
}
export declare enum LayoutDirection {
    TopToBottom = 1,
    LeftToRight = 2,
    BottomToTop = 3,
    RightToLEft = 4
}
export declare enum Alignment {
    Top = 1,
    Center = 2,
    Bottom = 3,
    Left = 1,
    Right = 3
}
export declare class Layout extends ContainerWidget {
    direction: LayoutDirection;
    mainAxisAlignment: Alignment;
    crossAxisAlignment: Alignment;
    main_justify: boolean;
    cross_justify: boolean;
    constructor(addContents: (ui: Ui) => void, direction?: LayoutDirection, mainAxisAlignment?: Alignment, crossAxisAlignment?: Alignment, main_justify?: boolean, cross_justify?: boolean);
}
export declare class Separator extends Widget {
    spacing: number;
    constructor(spacing?: number);
}
export declare class Label extends Widget {
    text: string;
    constructor(text: string);
}
export interface BoolValue {
    value: boolean;
}
export declare class CheckBox extends Widget {
    text: string;
    checked: BoolValue;
    constructor(text: string, checked: BoolValue);
}
export {};
//# sourceMappingURL=designer.d.ts.map