use crate::system::SysStats;
use ratatui::layout::{Constraint, Direction, Layout};
pub fn render(f: &mut Frame, stats: &SysStats) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            //CPU
            Constraint::Length(3),
            //RAM
            Constraint::Length(3),
            // PROCESSES AND NET
            Constraint::Min(10),
        ])
        .split(f.area());

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            // Processes
            Constraint::Percentage(50),
            // Network
            Constraint::Percentage(50),
        ])
        .split(main_chunks[2]);
}
