mod process_data; // Import the process_data module
mod ui; // Import the ui module

use crossterm::event::{poll, read, Event, KeyCode};
use std::io;
use std::time::Duration;
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::thread::sleep;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode};
use crossterm::execute;
use process_data::get_process_data;
use sysinfo::System;
use sysinfo::SystemExt;
use sysinfo::ProcessorExt;
use sysinfo::ProcessExt;






fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout();

    // Switch to the alternate screen and enable raw mode
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut system = System::new_all();

    let mut command_buffer = String::new();

    let selected_row = 0;

    terminal.clear()?;

    loop {
        let mut rows = get_process_data(&mut system);
        rows.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let total_cpu_usage = system.get_global_processor_info().get_cpu_usage();
        let total_cpu_percentage = total_cpu_usage as f64;
        let used_memory = system.get_used_memory() as f64;
        let total_memory = system.get_total_memory() as f64;
        let total_mem_percentage = (used_memory / total_memory) * 100.0;
    
        let prompt_text = if command_buffer.starts_with('k') {
            format!("Enter PID to kill: {}", &command_buffer[1..])
        } else {
            format!("Press 'k' to kill a process. Press 'Esc' to quit. {}", command_buffer)
        };
        let selected_row = 0;
    

        terminal.draw(|f| ui::draw_ui(f, &prompt_text, &rows, selected_row, total_cpu_percentage, total_mem_percentage))?; // Call draw_ui here
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
            terminal.draw(|f| ui::draw_ui(f, &prompt_text, &rows, selected_row, total_cpu_percentage, total_mem_percentage))?; // Call draw_ui here

        }

    }

// Disable raw mode and leave the alternate screen
disable_raw_mode()?;
execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    terminal.clear()?;
    Ok(())
}

