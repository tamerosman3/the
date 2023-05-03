use crossterm::event::{poll, read, Event, KeyCode};
use std::io;
use std::time::Duration;
use sysinfo::{ProcessExt, System, SystemExt};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Gauge, Row, Table},
    Terminal,
};
use std::thread::sleep;
use sysinfo::ProcessorExt;

fn get_process_data(system: &mut sysinfo::System) -> Vec<(i32, f32, f64, String, sysinfo::ProcessStatus)> {
    let mut rows = vec![];

    system.refresh_all();

    let num_processors = system.get_processors().len() as f32;


    for (pid, process) in system.get_processes() {
        let cpu_percent = process.cpu_usage() / num_processors;
        let mem_percent =
            (process.memory() as f64 / system.get_total_memory() as f64) * 100.0;
        let process_name = process.name().to_string();
        let status = process.status();

        rows.push((*pid,  cpu_percent, mem_percent, process_name, status));
    }

    rows
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut system = System::new_all();

    terminal.clear()?;

    loop {
        let mut rows = get_process_data(&mut system);
        rows.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let total_cpu_usage = system.get_global_processor_info().get_cpu_usage();
        let total_cpu_percentage = total_cpu_usage as f64;
        let total_mem_percentage =
    rows.iter().map(|row| row.2).sum::<f64>() / rows.len() as f64;


        terminal.draw(|f| {
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
                .split(f.size());

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
        })?;

        if poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = read()? {
                if key_event.code == KeyCode::Char('q') || key_event.code == KeyCode::Esc {
                    break;
                }
            }
        }

        sleep(Duration::from_secs(2));
    }

    terminal.clear()?;
    Ok(())
}

