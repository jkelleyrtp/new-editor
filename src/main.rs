use std::path::PathBuf;

mod schemas {
    mod text;
    mod workbench;
}

mod config {
    mod workbench;
}

mod components {
    pub mod sidebar;
}

pub mod event_loops {
    pub mod core;
}

use components::sidebar::Sidebar;

use clap::Parser;
use dioxus::{core::to_owned, prelude::*};
use event_loops::core::main_event_loop;
use fermi::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    dir: PathBuf,
}

fn main() {
    let args = Args::parse();

    dioxus_desktop::launch_with_props(
        app,
        appProps { dir: args.dir },
        Config::new()
            .with_custom_head(include_str!("../index.html").into())
            .with_window(
                WindowBuilder::new()
                    .with_maximized(true)
                    .with_titlebar_transparent(true)
                    .with_titlebar_buttons_hidden(false)
                    .with_title_hidden(true)
                    .with_menu({
                        let mut menu = MenuBar::new();
                        menu.add_submenu("Main", true, {
                            let mut m = MenuBar::new();

                            m
                        });
                        menu.add_submenu("File", true, {
                            let mut m = MenuBar::new();
                            m.add_item(MenuItemAttributes::new("Save"));

                            m
                        });
                        menu.add_submenu("Edit", true, {
                            let mut m = MenuBar::new();
                            m.add_native_item(MenuItem::Copy);
                            m.add_native_item(MenuItem::Cut);
                            m.add_native_item(MenuItem::Paste);
                            m.add_native_item(MenuItem::Hide);
                            m.add_native_item(MenuItem::HideOthers);

                            m
                        });
                        menu
                    })
                    .with_decorations(false),
            ),
    );
}

use dioxus::prelude::*;
use dioxus_desktop::{
    tao::{
        menu::{MenuBar, MenuItem, MenuItemAttributes},
        platform::macos::WindowBuilderExtMacOS,
    },
    Config, WindowBuilder,
};

#[inline_props]
fn app(cx: Scope, dir: PathBuf) -> Element {
    let fermi_cx = fermi::use_atom_root(&cx).clone();

    use_coroutine(&cx, |rx: UnboundedReceiver<_>| {
        to_owned![fermi_cx, dir];

        main_event_loop(fermi_cx, rx, Some(dir))
    });

    render!(
        div { class: "w-full h-full bg-red-500 flex flex-col",
            onkeydown: move |evt| {
                // println!("key down!")
            },
            FixedTitle {}
            div { class: "bg-green-300 grow flex flex-row overflow-hidden",
                Sidebar {}
                WindowSplitter {}
            }
            PowerBottom {}
        }
    )
}

fn WindowSplitter(cx: Scope) -> Element {
    // div { class: "w-48 h-48 mx-auto",
    //     textarea {
    //         class: "bg-yellow-300 w-full h-full",
    //         // contenteditable: "true",
    //         "Hello World"
    //     }
    // }

    render!(div { class: "grow overflow-hidden", EditorContent {} })
}

fn EditorContent(cx: Scope) -> Element {
    let file_contents = use_read(&cx, crate::event_loops::core::CurrentFileContents);

    let contents = match file_contents.as_ref() {
        Some(contents) => contents.as_str(),
        None => "",
    };

    render! {
        div {
            class: "h-full bg-gray-800 mx-auto text-white font-mono whitespace-pre-wrap p-8 overflow-scroll",
            contenteditable: "true",
            contents
        }
    }
}

static WindowTitle: Atom<String> = |_| "new-ide".into();

fn FixedTitle(cx: Scope) -> Element {
    let title = use_read(&cx, WindowTitle);
    let window = dioxus_desktop::use_window(&cx);

    render!(
        div { class: "w-full flex flex-row justify-between", onmousedown: move |_| window.drag(),
            // stoplight
            div { class: "pt-2",
                button { class: "rounded bg-red-900 h-4 w-4", "X" }
                button { class: "rounded bg-yellow-900 h-4 w-4", "X" }
                button { class: "rounded bg-green-900 h-4 w-4", "X" }
            }

            // center info pannel
            div { class: "pt-1",
                div {}
                div { "{title}" }
                div {}
            }

            // right split helpers
            div { class: "bg-black-100 w-24 h-4 pt-2", "split helper" }
        }
    )
}

fn Explorer(cx: Scope) -> Element {
    render!("")
}

fn PowerMenu(cx: Scope) -> Element {
    // warnings, terminal, problems, debug, etc
    render!("")
}

fn PowerBottom(cx: Scope) -> Element {
    render!(
        div { class: "w-full flex flex-row justify-between",
            // left helpers
            div { class: "w-30 bg-blue-400" }

            // right helpers
            div { class: "bg-black-100 w-24", "split helper" }
        }
    )
}

fn Terminal(cx: Scope) -> Element {
    render!("")
}
