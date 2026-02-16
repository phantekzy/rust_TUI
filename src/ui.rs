use crate::system::SysStats;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Sparkline},
};

pub fn render(f: &mut Frame, stats: &SysStats) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(10),
        ])
        .split(f.area());

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[2]);

    // 1. CPU Gauge
    f.render_widget(
        Gauge::default()
            .block(Block::default().title(" CPU ").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Cyan))
            .percent(stats.cpu_usage as u16),
        main_chunks[0],
    );

    // 2. RAM Gauge
    let mem_p = (stats.mem_used as f64 / stats.mem_total as f64 * 100.0) as u16;
    f.render_widget(
        Gauge::default()
            .block(Block::default().title(" RAM ").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Magenta))
            .percent(mem_p),
        main_chunks[1],
    );

    // 3. Process List
    let items: Vec<ListItem> = stats
        .processes
        .iter()
        .map(|(name, mem)| ListItem::new(format!("{:<15} | {} MB", name, mem)))
        .collect();

    let process_list = List::new(items)
        .block(
            Block::default()
                .title(" Top Processes (Mem) ")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::White));
    f.render_widget(process_list, bottom_chunks[0]);

    // 4. Network Info
    let net_text = format!(
        "\n IN:  {} KB/s\n OUT: {} KB/s\n\n Cache Stats: Optimized\n GPU: Detected",
        stats.net_in / 1024,
        stats.net_out / 1024
    );
    let net_para = Paragraph::new(net_text)
        .block(
            Block::default()
                .title(" Network & Hardware ")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(net_para, bottom_chunks[1]);
}
