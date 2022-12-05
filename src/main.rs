mod stateful_tree;
mod utils;

use crate::stateful_tree::StatefulTree;
use crate::utils::{restore, setup};
use crossterm::event;
use std::{env, fs, io};
use tui_tree_widget::{Tree, TreeItem};

fn get_tree_items(dir: &std::path::PathBuf) -> Vec<TreeItem<'static>> {
    let mut tree_items: Vec<TreeItem> = Vec::new();
    let mut files: Vec<TreeItem> = Vec::new();

    for path in fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
    {
        if path.is_file() {
            files.push(TreeItem::new_leaf(
                path.file_name()
                    .unwrap()
                    .clone()
                    .to_str()
                    .unwrap()
                    .to_owned(),
            ));
        } else {
            tree_items.push(TreeItem::new(
                path.file_name()
                    .unwrap()
                    .clone()
                    .to_str()
                    .unwrap()
                    .to_owned(),
                get_tree_items(&path),
            ));
        }
    }

    tree_items.append(&mut files);

    return tree_items;
}

fn main() -> Result<(), io::Error> {
    let mut terminal = setup().unwrap();

    let mut tree = StatefulTree::with_items(get_tree_items(&env::current_dir().unwrap()));

    loop {
        terminal.draw(|frame| {
            let size = frame.size();
            let items = Tree::new(tree.items.clone());
            frame.render_stateful_widget(items, size, &mut tree.state);
        })?;

        if let event::Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Char('q') => break,
                _ => {}
            }
        };
    }

    restore(terminal)
}
