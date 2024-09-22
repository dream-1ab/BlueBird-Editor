/// <reference path="../assets/js_modules/build/designer.d.ts" />

import {Ui, Button, MenuButton, Separator, Alignment, Layout, LayoutDirection, Label, CheckBox} from "designer";
// import { Alignment, Layout, LayoutDirection } from "./js_modules/build/designer";
// import {Version, UiResponse} from "designer/egui"

globalThis.ui_main = ui_main

let count = 0
let itemCount = 10
let enablePartOfUi = {
    value: true
}
function ui_main(ui) {
    if (enablePartOfUi.value) {
        ui.add(new Button(`Click me ${count}`).withResponse((response) => {
            if (response.clicked) {
                print("Hello world")
                count++
            }
            response.contextMenu((ui) => {
                ui.add(new MenuButton("My Context Menu", (ui) => {
                    ui.add(new Button("add new item ++").withResponse((response) => {
                        if (response.clicked) {
                            itemCount += 1;
                        }
                    }))
                    ui.add(new Button("add new item --").withResponse((response) => {
                        if (response.clicked) {
                            itemCount -= 1;
                        }
                    }))
                    for (let a = 0; a < itemCount; a++) {
                        if (a == 5) {
                            ui.add(new Separator())
                        }
                        ui.add(new Button(`Menu item ${a}`).withResponse((response) => {
                            if (response.hovered) {
                                // print(`menu item ${a} is hovered`)
                            }
                            if (response.clicked) {
                                print(`menu item ${a} is Clicked`)
                            }
                        }))
                    }
                    ui.add(new Separator())
                    ui.add(new Button("Menu item 3").withResponse((response) => {
                        if (response.clicked) {
                            ui.closeMenu()
                        }
                    }))
                }))
            })
            ui.add(new Label(`Hello world from text counter: ${count}`))
            ui.add(new Separator())
            ui.add(new Layout((ui) => {
                ui.add(new Button("Click me to ++").withResponse((response) => {
                    if (response.clicked) {
                        count += 2
                    }
                }))
                ui.add(new Separator())
                ui.add(new Button("Click me to --").withResponse((response) => {
                    if (response.clicked) {
                        count -= 2
                    }
                }))
            }, LayoutDirection.BottomToUp))
        }))
    }
    ui.add(new CheckBox("Click me to enable/disable part of your UI", enablePartOfUi))
}

print(globalThis)

// function X(ui) {
//     ui.add(new Button("click me").uiResponse((response) => {
//         response.contextMenu((ui) => {
//             ui.add(new Button("You are entered this area.").uiResponse((response) => {
//                 ui.closeMenu()
//             }))
//         })
//     }))

//     if (ui.add(new Button("click me")).clicked) {

//     }
// }
