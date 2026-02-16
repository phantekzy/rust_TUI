use crate::system::SysStats;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Gauge, Paragraph, Row, Table},
};

pub fn render(f: &mut Frame, stats: &SysStats) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(4),
            Constraint::Min(10),
        ])
        .split(f.area());

    //  Header
    let header_text = format!(" Uptime: {}s | Load Avg: {} ", stats.uptime, stats.load_avg);
    f.render_widget(
        Paragraph::new(header_text).block(Block::default().borders(Borders::ALL)),
        main_chunks[0],
    );

    // Gauges
    let gauge_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(main_chunks[1]);

    f.render_widget(
        Gauge::default()
            .block(Block::default().title(" CPU ").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Cyan))
            .percent(stats.cpu_usage as u16),
        gauge_chunks[0],
    );

    // RAM percentage
    let mem_p = if stats.mem_total > 0 {
        (stats.mem_used as f64 / stats.mem_total as f64 * 100.0) as u16
    } else {
        0
    };
    f.render_widget(
        Gauge::default()
            .block(Block::default().title(" RAM ").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Magenta))
            .percent(mem_p),
        gauge_chunks[1],
    );

    //   Swap percentage
    let swap_p = if stats.swap_total > 0 {
        (stats.swap_used as f64 / stats.swap_total as f64 * 100.0) as u16
    } else {
        0
    };
    f.render_widget(
        Gauge::default()
            .block(Block::default().title(" SWAP ").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Yellow))
            .percent(swap_p),
        gauge_chunks[2],
    );

    //  Network
    let rx_kbs = (stats.net_in * 2) / 1024;
    let tx_kbs = (stats.net_out * 2) / 1024;
    let net_text = format!(" ▼ RX: {:>6} KB/s | ▲ TX: {:>6} KB/s ", rx_kbs, tx_kbs);

    f.render_widget(
        Paragraph::new(net_text).block(Block::default().title(" Network ").borders(Borders::ALL)),
        main_chunks[2],
    );

    // Table
    let header = Row::new(vec!["PID", "Process Name", "CPU %", "MEM (MB)"])
        .style(Style::default().fg(Color::Yellow).bg(Color::DarkGray));

    let rows: Vec<Row> = stats
        .processes
        .iter()
        .map(|p| {
            Row::new(vec![
                Cell::from(p.pid.clone()),
                Cell::from(p.name.clone()),
                Cell::from(format!("{:.1}%", p.cpu)),
                Cell::from(format!("{}", p.mem_mb)),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(8),
            Constraint::Min(20),
            Constraint::Length(8),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .block(Block::default().title(" Processes ").borders(Borders::ALL));

    f.render_widget(table, main_chunks[3]);
}
