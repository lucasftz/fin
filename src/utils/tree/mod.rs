use std::fs;

use tui_tree_widget::{TreeItem, TreeState};

pub fn get_tree_items(dir: &std::path::PathBuf) -> Vec<TreeItem<'static>> {
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
    tree_items
}

pub struct StatefulTree<'a> {
    pub state: TreeState,
    pub items: Vec<TreeItem<'a>>,
}

impl<'a> StatefulTree<'a> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            state: TreeState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<TreeItem<'a>>) -> Self {
        Self {
            state: TreeState::default(),
            items,
        }
    }

    pub fn first(&mut self) {
        self.state.select_first();
    }

    pub fn last(&mut self) {
        self.state.select_last(&self.items);
    }

    pub fn down(&mut self) {
        self.state.key_down(&self.items);
    }

    pub fn up(&mut self) {
        self.state.key_up(&self.items);
    }

    pub fn left(&mut self) {
        self.state.key_left();
    }

    pub fn right(&mut self) {
        self.state.key_right();
    }

    pub fn toggle(&mut self) {
        self.state.toggle_selected();
    }
}
