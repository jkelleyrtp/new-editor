use std::{fs::DirEntry, path::PathBuf, rc::Rc};

use dioxus::prelude::{Element, UnboundedReceiver};
use fermi::{Atom, AtomRoot, Readable};
use futures::{
    future::{BoxFuture, FutureExt},
    StreamExt,
};

pub static Files: Atom<FileTree> = |_| FileTree::default();

pub static CurrentFileContents: Atom<Option<String>> = |_| None;

pub enum Action {
    OpenFile(PathBuf),
}

// The main event loop for the editor
// This essentially holds all the state of the app and drives UI changes through Fermi
pub async fn main_event_loop(
    cx: Rc<AtomRoot>,
    mut rx: UnboundedReceiver<Action>,
    init_dir: Option<PathBuf>,
) {
    // try to load the directory up when the editor is initialized
    if let Some(init) = init_dir {
        match try_load_dir(init).await {
            Ok(files) => cx.set(Files.unique_id(), files),
            Err(ee) => {
                // send a message to the user that the directory failed to load using the toast/modal system
                eprintln!("failed to load directory")
            }
        }
    }

    // wait for keyboard input, ongoing tasks, file system events, etc

    loop {
        let msg = rx.next().await.unwrap();

        match msg {
            Action::OpenFile(f) => {
                // open the file and set the current file contents
                if let Ok(contents) = std::fs::read_to_string(&f) {
                    cx.set(CurrentFileContents.unique_id(), Some(contents));
                }
            }
        }

        // tokio::select! {
        //     Some(action) = rx.next() => {
        //         match action {
        //             Action::OpenFile(path) => {
        //                 // open the file in a new tab
        //             }
        //         }
        //     }
        // }
    }
}

#[derive(Default)]
pub struct FileTree {
    pub files: Vec<FileEntry>,
}

pub enum FileEntry {
    File {
        name: String,
        path: PathBuf,
    },
    Directory {
        name: String,
        path: PathBuf,
        files: Vec<FileEntry>,
    },
}

impl FileEntry {
    pub fn name(&self) -> &str {
        match self {
            FileEntry::File { name, .. } => name,
            FileEntry::Directory { name, .. } => name,
        }
    }
    pub fn is_dir(&self) -> bool {
        match self {
            FileEntry::File { .. } => false,
            FileEntry::Directory { .. } => true,
        }
    }
}

async fn try_load_dir(dir: PathBuf) -> anyhow::Result<FileTree> {
    let files = tokio::task::spawn_blocking(move || -> anyhow::Result<FileTree> {
        let mut files = FileTree::default();

        // todo: parallelize this
        fn gather_files(entry: DirEntry) -> anyhow::Result<FileEntry> {
            if entry.file_type()?.is_dir() {
                let files = std::fs::read_dir(entry.path())?;

                let mut entries = Vec::new();
                for file in files {
                    entries.push(gather_files(file?)?);
                }

                entries.sort_by(|a, b| a.name().cmp(b.name()));

                Ok(FileEntry::Directory {
                    name: entry.file_name().to_string_lossy().into_owned(),
                    path: entry.path(),
                    files: entries,
                })
            } else {
                Ok(FileEntry::File {
                    name: entry.file_name().to_string_lossy().into_owned(),
                    path: entry.path(),
                })
            }
        }

        // todo: properly paprallelize this
        for entry in std::fs::read_dir(dir)? {
            files.files.push(gather_files(entry?)?);
        }

        files.files.sort_by(|a, b| a.name().cmp(b.name()));

        Ok(files)
    })
    .await??;

    Ok(files)
}
