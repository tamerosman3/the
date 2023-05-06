use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Gauge, Row, Table},
    Frame,
};


  
pub fn draw_ui(f: &mut Frame<CrosstermBackend<io::Stdout>>, prompt_text: &str, rows: &[(i32, f32, f64, String, sysinfo::ProcessStatus)], selected_row: usize, total_cpu_percentage: f64, total_mem_percentage: f64) {


    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), // Add an extra constraint for the prompt
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(f.size());

    
    let prompt = tui::widgets::Paragraph::new(prompt_text).block(Block::default().borders(Borders::ALL).title("Commands"));
    f.render_widget(prompt, chunks[0]);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(70),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let memory_gauge = Gauge::default()
        .block(
            Block::default()
                .title("Total Memory%")
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(Color::Green))
        .percent(total_mem_percentage.round() as u16);

    f.render_widget(memory_gauge, chunks[1]);

    let cpu_gauge = Gauge::default()
        .block(
            Block::default()
                .title("CPU%")
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(Color::Red))
        .percent(total_cpu_percentage.round() as u16);

    f.render_widget(cpu_gauge, chunks[2]);

    let rows = rows.iter()
    .map(|row| {
        Row::new(vec![
            Cell::from(format!("{:<10}", row.0)),
            // Format CPU% with only two decimal places
            Cell::from(format!("{:<15.2}", row.1)),
            Cell::from(format!("{:<15.2}", row.2)),
            Cell::from(format!("{:<25}", row.3)),
            Cell::from(format!("{:<10}", format!("{:?}", row.4))),
        ])
    })
    .collect::<Vec<_>>();

    let header = Row::new(vec![
        Cell::from("PID").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("CPU%").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Memory%").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Process Name").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Status").style(Style::default().add_modifier(Modifier::BOLD)),
    ]);

    let table = Table::new(std::iter::once(header).chain(rows))
    .block(Block::default().borders(Borders::ALL).title("Process Table"))
    .widths(&[
        Constraint::Percentage(10),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(45),
        Constraint::Percentage(15),
    ]);

    f.render_widget(table, chunks[0]);
}