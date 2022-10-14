use dioxus::{events::MouseEvent, prelude::*};
use fermi::*;

pub static SidebarItems: Atom<Vec<SidebarItem>> = |_| {
    vec![
        SidebarItem {
            name: "Explorer",
            icon: "ex",
            hover_text: "asd",
            component: FileTree,
        },
        SidebarItem {
            name: "Search",
            icon: "sr",
            hover_text: "asd",
            component: Search,
        },
    ]
};

// The entire sidebar including buttons and pane
pub fn Sidebar(cx: Scope) -> Element {
    let sidebar_items = use_read(&cx, SidebarItems);
    let selected_item = use_state(&cx, || 0);

    let item: &SidebarItem = sidebar_items.get(**selected_item)?;

    render!(
        div { class: "flex flex-row text-sm",
            // the left widget area
            div { class: "w-12 h-full bg-blue-300 flex flex-col justify-between",
                div { class: "flex flex-col",
                    sidebar_items.iter().enumerate().map(|(id, item)| rsx! {
                        button { onclick: move |_| selected_item.set(id),
                            "{item.icon}"
                        }
                    })
                }
            }

            // the right renderer area
            div { class: "bg-green-100", resize: "horizontal", overflow: "hidden",
                NodeFactory::new(&cx).component(item.component, (), None, item.name)
            }
        }
    )
}

pub struct SidebarItem {
    name: &'static str,

    // todo use actual icons
    icon: &'static str,

    hover_text: &'static str,

    // todo: global shortcut
    component: Component,
}

fn Search(cx: Scope) -> Element {
    render!(div {})
}

fn FileTree(cx: Scope) -> Element {
    let files = use_read(&cx, crate::event_loops::core::Files);
    render!(
        div { class: "font-mono",
            "File Tree"

            ul {
                files.files.iter().filter(|f| f.is_dir()).map(|file| rsx! {
                    FileRow { entry: file, indent: 0 }
                })
                files.files.iter().filter(|f| !f.is_dir()).map(|file| rsx! {
                    FileRow { entry: file, indent: 0 }
                })
            }
        }
    )
}

use crate::event_loops::core::*;

#[inline_props]
fn FileRow<'a>(cx: Scope<'a>, entry: &'a FileEntry, indent: usize) -> Element {
    let handler = use_coroutine_handle::<Action>(&cx)?;

    match entry {
        FileEntry::File { name, path } => render!(Container {
            indent: *indent,
            title: "• {name}",
            onclick: move |_| handler.send(Action::OpenFile(path.clone()))
        }),
        FileEntry::Directory { name, path, files } => render!(DirRow {
            files: files,
            name: name,
            indent: *indent
        }),
    }
}

#[inline_props]
fn Container<'a>(
    cx: Scope<'a>,
    title: &'a str,
    children: Element<'a>,
    onclick: EventHandler<'a, MouseEvent>,
    indent: usize,
) -> Element {
    let depth = indent * 8;

    render!(
        div { class: "",
            margin_left: "{depth}px",
            div { class: "hover:bg-gray-200 w-full py-px pl-4",
                onclick: move |evt| onclick.call(evt),
                "{title}",
            }

            children
        }
    )
}

#[inline_props]
fn DirRow<'a>(
    cx: Scope<'a>,
    files: &'a Vec<FileEntry>,
    indent: usize,
    name: &'a String,
) -> Element {
    let show_children = use_state(&cx, || false);

    let open_icon = if **show_children { "▼" } else { "▶" };

    render!(
        Container { indent: *indent,
            onclick: move |_| show_children.set(!**show_children),
            title: "{open_icon} {name}",
            if **show_children {
                render! {
                    files.iter().filter(|f| f.is_dir()).map(|f| render! {
                        FileRow { entry: f, indent: indent + 1 }
                    })
                    files.iter().filter(|f| !f.is_dir()).map(|f| render! {
                        FileRow { entry: f, indent: indent + 1 }
                    })
                }
            } else {
                render! {""}
            }
        }
    )
}
