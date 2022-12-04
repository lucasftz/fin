mod utils;
use crate::utils::utils::{restore, setup};

use std::{io, thread, time::Duration};

use tui::widgets::{Block, Borders};

fn main() -> Result<(), io::Error> {
    let mut terminal = setup().unwrap();

    terminal.draw(|frame| {
        let size = frame.size();
        let block = Block::default().title("Block").borders(Borders::ALL);
        frame.render_widget(block, size);
    })?;

    thread::sleep(Duration::from_millis(5000));

    restore(terminal)
}
