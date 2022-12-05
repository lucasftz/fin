mod utils;
use crate::utils::term::{restore, setup};
use crate::utils::tree::{get_tree_items, StatefulTree};

use std::{env, io};

use crossterm::event;
use tui_tree_widget::Tree;

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
                event::KeyCode::Char('h') => tree.left(),
                event::KeyCode::Char('j') => tree.down(),
                event::KeyCode::Char('k') => tree.up(),
                event::KeyCode::Char('l') => tree.right(),
                _ => {}
            }
        };
    }

    restore(terminal)
}
