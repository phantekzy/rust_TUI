use std::{
    alloc::System,
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode}, execute, terminal::{self, EnterAlternateScreen, enable_raw_mode}
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

    loop {
        let stats = system::get_stats(&mut sys);
        terminal.draw(|f| ui::render(f, &stats))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }

}
