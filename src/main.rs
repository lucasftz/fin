mod utils;
use crate::utils::{restore, setup};

use std::{env, fs, io, path, thread, time};

use tui::widgets::{List, ListItem};

fn main() -> Result<(), io::Error> {
    let mut terminal = setup().unwrap();

    let paths: Vec<path::PathBuf> = fs::read_dir(env::current_dir().unwrap())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();

    let (mut dirs, mut files) = (Vec::new(), Vec::new());

    for path in paths.iter() {
        let item = ListItem::new(path.to_str().unwrap());

        if path.is_file() {
            files.push(item);
        } else {
            dirs.push(item);
        }
    }

    dirs.append(&mut files);

    terminal.draw(|frame| {
        let size = frame.size();
        let items = List::new(dirs);
        frame.render_widget(items, size);
    })?;

    thread::sleep(time::Duration::from_millis(2000));

    restore(terminal)
}
