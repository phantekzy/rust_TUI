use crate::system::SysStats;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Gauge, Paragraph},
};

pub fn render(f: &mut Frame, stats: &SysStats) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            // CPU
            Constraint::Length(3),
            // RAM
            Constraint::Length(3),
            // INFO
            Constraint::Min(0),
        ])
        .split(f.size());
}
