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
use std::io::Write;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode};
use crossterm::execute;
use tui::Frame;
mod process_data; // Import the process_data module
use process_data::get_process_data;



fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout();

    // Switch to the alternate screen and enable raw mode
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut system = System::new_all();

    let mut command_buffer = String::new();


    terminal.clear()?;

    loop {
        let mut rows = get_process_data(&mut system);
        rows.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let total_cpu_usage = system.get_global_processor_info().get_cpu_usage();
        let total_cpu_percentage = total_cpu_usage as f64;
        let used_memory = system.get_used_memory() as f64;
        let total_memory = system.get_total_memory() as f64;
        let total_mem_percentage = (used_memory / total_memory) * 100.0;
        
        
        let draw_ui = |f: &mut Frame<CrosstermBackend<io::Stdout>>, prompt_text: &str| {


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
        };




        let prompt_text = if command_buffer.starts_with('k') {
            format!("Enter PID to kill: {}", &command_buffer[1..])
        } else {
            format!("Press 'k' to kill a process. Press 'Esc' to quit. {}", command_buffer)
        };
        terminal.draw(|f| draw_ui(f, &prompt_text))?; // Call draw_ui here
        let mut updated = false;


        if poll(Duration::from_millis(10))? {
            if let Event::Key(key_event) = read()? {
                updated = true;
                match key_event.code {
                    KeyCode::Char('k') => {
                        command_buffer.push('k');
                    }
                    KeyCode::Char('q') => {
                        command_buffer.push('q');
                    }
                    KeyCode::Char(c) if command_buffer.starts_with('k') => {
                        command_buffer.push(c);
                    }                    
                    KeyCode::Enter => {
                        if command_buffer.starts_with('k') {
                            // Add the process killing logic here
                            if let Ok(pid) = command_buffer[1..].parse::<sysinfo::Pid>() {
                                if let Some(process) = system.get_process(pid) {
                                    process.kill(sysinfo::Signal::Kill);
                                    command_buffer.clear();
                                } else {
                                    command_buffer = format!("Failed to kill process with PID: {}", pid);
                                }
                            } else {
                                command_buffer = format!("Invalid PID: {}", &command_buffer[1..]);
                            }
                        } else if command_buffer.starts_with('q') {
                            break;
                        } else {
                            command_buffer.clear();
                        }
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Backspace if !command_buffer.is_empty() => {
                        command_buffer.pop();
                    }
                    _ => {}
                }
            }
        }
        if !updated {
            system.refresh_all();
            sleep(Duration::from_millis(1000));
        } else {
            // 3. Call draw_ui after handling input events
            let prompt_text = if command_buffer.starts_with('k') {
                "Enter PID to kill.".to_string()
            } else {
                format!("Press 'k' to kill a process. Press 'Esc' to quit. {}", command_buffer)
            };
            terminal.draw(|f| draw_ui(f, &prompt_text))?; // Call draw_ui here
        }

    }

// Disable raw mode and leave the alternate screen
disable_raw_mode()?;
execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    terminal.clear()?;
    Ok(())
}

