use std::{
    alloc::System,
    io,
    time::{Duration, Instant},
};

use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, enable_raw_mode},
};
use ratatui::{Terminal, backend, prelude::CrosstermBackend};
use sysinfo::System;

mod system;
mod ui;

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut sys = System::new_all();
    let tick_rate = Duration::from_millis(500);
    let mut last_tick = Instant::now();
}
