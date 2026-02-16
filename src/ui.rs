use crate::system::SysStats;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge},
};

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

    // CPU USAGE
    f.render_widget(
        Gauge::default()
            .block(Block::default().title(" CPU ").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Cyan))
            .percent(stats.cpu_usage as u16),
        main_chunks[0],
    );
}
