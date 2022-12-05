mod utils;
use crate::utils::utils::{restore, setup};

use std::{env, fs, io, path, thread, time};

use tui::widgets::{List, ListItem};

fn main() -> Result<(), io::Error> {
    let mut terminal = setup().unwrap();

    let paths: Vec<path::PathBuf> = fs::read_dir(env::current_dir().unwrap())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();

    let items: Vec<ListItem> = paths
        .iter()
        .map(|path| ListItem::new(path.to_str().unwrap()))
        .collect();

    terminal.draw(|frame| {
        let size = frame.size();
        let list = List::new(items);
        frame.render_widget(list, size);
    })?;

    thread::sleep(time::Duration::from_millis(2000));

    restore(terminal)
}
