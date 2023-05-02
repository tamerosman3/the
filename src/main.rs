// use psutil::process::Process;

// fn main() {
//     let processes = psutil::process::processes().unwrap();
//     let mut rows = vec![];

//     // Get the data for each process
//     for process in processes {
//         let pid = process.expect("Failed to get process").pid();
//         let mut proc = Process::new(pid).unwrap();
//         let cpu_time = proc.cpu_times().unwrap();
//         let total_time = cpu_time.user().as_nanos() + cpu_time.system().as_nanos() +
//             cpu_time.children_user().as_nanos() + cpu_time.children_system().as_nanos();
//         let mem_con = proc.memory_percent().unwrap();
//         let cpu_con = proc.cpu_percent().unwrap();
//         let process_name = proc.name().unwrap();
//         let status = proc.status().unwrap();

//         // Add the data for the current process to the rows vector
//         rows.push((pid, total_time, mem_con, cpu_con, process_name, status));
//     }

//     // Print the table header
//     println!("{:<10} {:<15} {:<15} {:<10} {:<15} {:<10}", "PID", "CPU Time", "Memory%", "CPU%", "Process Name", "Status");

//     // Print the table rows
//     for row in rows {
//         println!("{:<10} {:<15} {:<15.2} {:<10.2} {:<15} {:<10}", row.0, row.1, row.2, row.3, row.4, format!("{:?}", row.5));
//     }
// }



use crossterm::event::{poll, read, Event, KeyCode};
use std::io;
use std::time::Duration;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Terminal,
};
use psutil::process::Process;
use tui::widgets::Wrap;
use tui::text::Text;


fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let processes = psutil::process::processes().unwrap();
    let mut rows = vec![];

    // Get the data for each process
    for process in processes {
        let pid = process.expect("Failed to get process").pid();
        let mut proc = Process::new(pid).unwrap();
        let cpu_time = proc.cpu_times().unwrap();
        let total_time = cpu_time.user().as_nanos() + cpu_time.system().as_nanos() +
            cpu_time.children_user().as_nanos() + cpu_time.children_system().as_nanos();
        let mem_con = proc.memory_percent().unwrap();
        let cpu_con = proc.cpu_percent().unwrap();
        let process_name = proc.name().unwrap();
        let status = proc.status().unwrap();

        // Add the data for the current process to the rows vector
        rows.push((pid, total_time, mem_con, cpu_con, process_name, status));
    }

    let total_mem_percentage = rows.iter().map(|row| row.2).sum::<f32>() / rows.len() as f32;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            let gauge = Gauge::default()
                .block(
                    Block::default()
                        .title("Total Memory%")
                        .borders(Borders::ALL),
                )
                .gauge_style(Style::default().fg(Color::Green))
                .percent(total_mem_percentage.round() as u16);

            let gauge_chunk = Rect {
                x: chunks[0].width.saturating_sub(20),
                y: chunks[0].y,
                width: 20,
                height: chunks[0].height,
            };

            f.render_widget(gauge, gauge_chunk);

            let mut process_list = String::new();
            process_list.push_str(&format!("{:<10} {:<15} {:<15} {:<10} {:<15} {:<10}\n", "PID", "CPU Time", "Memory%", "CPU%", "Process Name", "Status"));
            for row in &rows {
                process_list.push_str(&format!("{:<10} {:<15} {:<15.2} {:<10.2} {:<15} {:<10}\n", row.0, row.1, row.2, row.3, row.4, format!("{:?}", row.5)));
            }
            
            let process_text = Text::from(process_list);
            let paragraph = Paragraph::new(process_text)
             .block(Block::default().borders(Borders::NONE))
             .alignment(Alignment::Left)
             .wrap(Wrap { trim: true });


            f.render_widget(paragraph, chunks[0]);
        
         })?;

        if poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = read()? {
                if key_event.code == KeyCode::Char('q') || key_event.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }
    
    terminal.clear()?;
    Ok(())
}