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
use std::process::Command;
use std::env;


fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "tree" {
        // Run another program
        let output = Command::new("./tes").output()?;
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
    
    
    
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
    let mut f = 1;
    loop {
        let mut rows = get_process_data(&mut system);
        
        if f == 1{
            rows.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        } else if f == 0{
            rows.sort_unstable_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        }else if f == 2 {
            rows.sort_unstable_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        }else if f == 3{ 
            rows.sort_unstable_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));
        } else{
            rows.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        }
        let total_cpu_usage = system.get_global_processor_info().get_cpu_usage();
        let total_cpu_percentage = total_cpu_usage as f64;
        let used_memory = system.get_used_memory() as f64;
        let total_memory = system.get_total_memory() as f64;
        let total_mem_percentage = (used_memory / total_memory) * 100.0;
    
        let prompt_text = if command_buffer.starts_with('k') {
            format!("Enter PID to kill: {}", &command_buffer[1..])
        } else if command_buffer.starts_with('s') {
            format!("Enter the initial to sort by: {}", &command_buffer[1..])
        } else {
            format!("  Press 'k' to kill a process. Press 'Esc' to quit. Press 's' to sort the table by a column.\n  Use 'sp' to sort acoording to PID, or 'sm' to sort according to memory usage, sc to sort according to cpu usage, or sn to sort according to the process name.\n  {}", command_buffer)
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
                    KeyCode::Char('c') if command_buffer.starts_with('k') => {
                        command_buffer.push('c');
                    }
                    KeyCode::Char('s') => {
                        command_buffer.push('s');
                    }

                    KeyCode::Char('c') if command_buffer.starts_with('s') => {
                        command_buffer.push('c');  }
                    KeyCode::Char('p') if command_buffer.starts_with('s') => {
                        command_buffer.push('p');  }
                    KeyCode::Char('m') if command_buffer.starts_with('s') => {
                        command_buffer.push('m');  }
                    KeyCode::Char('n') if command_buffer.starts_with('s') => {
                        command_buffer.push('n');  }                              
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
                        } else if command_buffer.len() >= 2 && &command_buffer[0..2] == "sp" {
                            f = 0;
                        } else if command_buffer.len() >= 2 && &command_buffer[0..2] == "sc" {
                            f=1;
                        } else if command_buffer.len() >= 2 && &command_buffer[0..2] == "sm"  {
                            f=2;
                        } else if command_buffer.len() >= 2 && &command_buffer[0..2] == "sn"  {
                            f=3;
                        }
                        else {
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
}
    Ok(())
}