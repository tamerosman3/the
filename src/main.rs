
// use crossterm::event::{poll, read, Event, KeyCode};
// use std::io;
// use std::time::Duration;
// use tui::{
//     backend::CrosstermBackend,
//     layout::{Alignment, Constraint, Direction, Layout, Rect},
//     style::{Color, Modifier, Style},
//     widgets::{Block, Borders, Gauge, Paragraph},
//     Terminal,
// };
// use psutil::process::Process;
// use tui::widgets::Wrap;
// use tui::text::Text;
// use tui::widgets::{Cell, Row, Table};
// use std::thread::sleep;

// Add this function to your code
// fn get_process_data() -> Vec<(u32, u128, f32, f32, String, psutil::process::Status)> {
//     let processes = psutil::process::processes().unwrap();
//     let mut rows = vec![];

//     Get the data for each process
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

//         Add the data for the current process to the rows vector
//         rows.push((pid, total_time, mem_con, cpu_con, process_name, status));
//     }

//     rows
// }


// fn main() -> Result<(), io::Error> {
//     let stdout = io::stdout();
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;

//     terminal.clear()?;



//     loop {
        
//             let rows = get_process_data();
//             let total_mem_percentage = rows.iter().map(|row| row.2).sum::<f32>() / rows.len() as f32;    
    

//             terminal.draw(|f| {
//                 let chunks = Layout::default()
//                     .direction(Direction::Horizontal)
//                     .margin(2)
//                     .constraints(
//                         [
//                             Constraint::Percentage(70),
//                             Constraint::Percentage(15),
//                             Constraint::Percentage(15),
//                         ]
//                         .as_ref(),
//                     )
//                     .split(f.size());
            
//                 let memory_gauge = Gauge::default()
//                     .block(
//                         Block::default()
//                             .title("Total Memory%")
//                             .borders(Borders::ALL),
//                     )
//                     .gauge_style(Style::default().fg(Color::Green))
//                     .percent(total_mem_percentage.round() as u16);
            
//                 f.render_widget(memory_gauge, chunks[1]);
            
//                 let cpu_percentage = rows.iter().map(|row| row.3).sum::<f32>() / rows.len() as f32;
            
//                 let cpu_gauge = Gauge::default()
//                     .block(
//                         Block::default()
//                             .title("CPU%")
//                             .borders(Borders::ALL),
//                     )
//                     .gauge_style(Style::default().fg(Color::Red))
//                     .percent(cpu_percentage.round() as u16);
            
//                 f.render_widget(cpu_gauge, chunks[2]);
            
//                 ... rest of the code
            


//             let mut process_list = String::new();
//             process_list.push_str(&format!("{:<10} {:<15} {:<15} {:<10} {:<15} {:<10}\n", "PID", "CPU Time", "Memory%", "CPU%", "Process Name", "Status"));
//             for row in &rows {
//                 process_list.push_str(&format!("{:<10} {:<15} {:<15.2} {:<10.2} {:<15} {:<10}\n", row.0, row.1, row.2, row.3, row.4, format!("{:?}", row.5)));
//             }
            
//             let rows = rows
//             .iter()
//             .map(|row| {
//                 Row::new(vec![
//                     Cell::from(format!("{:<10}", row.0)),
//                     Cell::from(format!("{:<15}", row.1)),
//                     Cell::from(format!("{:<15.2}", row.2)),
//                     Cell::from(format!("{:<10.2}", row.3)),
//                     Cell::from(format!("{:<15}", row.4)),
//                     Cell::from(format!("{:<10}", format!("{:?}", row.5))),
//                 ])
//             })
//             .collect::<Vec<_>>();
        
//         let header = Row::new(vec![
//             Cell::from("PID").style(Style::default().add_modifier(Modifier::BOLD)),
//             Cell::from("CPU Time").style(Style::default().add_modifier(Modifier::BOLD)),
//             Cell::from("Memory%").style(Style::default().add_modifier(Modifier::BOLD)),
//             Cell::from("CPU%").style(Style::default().add_modifier(Modifier::BOLD)),
//             Cell::from("Process Name").style(Style::default().add_modifier(Modifier::BOLD)),
//             Cell::from("Status").style(Style::default().add_modifier(Modifier::BOLD)),
//         ]);
        
//         let table = Table::new(std::iter::once(header).chain(rows))
//             .block(Block::default().borders(Borders::ALL).title("Process Table"))
//             .widths(&[
//                 Constraint::Percentage(10),
//                 Constraint::Percentage(20),
//                 Constraint::Percentage(15),
//                 Constraint::Percentage(10),
//                 Constraint::Percentage(25),
//                 Constraint::Percentage(20),
//             ]);
        
//         f.render_widget(table, chunks[0]);
        
//          })?;

//         if poll(Duration::from_millis(100))? {
//             if let Event::Key(key_event) = read()? {
//                 if key_event.code == KeyCode::Char('q') || key_event.code == KeyCode::Esc {
//                     break;
//                 }
//             }
//         }

//         sleep(Duration::from_secs(2));

//     }
    
//     terminal.clear()?;
//     Ok(())
// }




use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    // Print table header
    println!("{:<8} {:<20} {:<15} {:<15} {:<10}", "PID", "Name", "CPU%", "Memory%", "Status");

    // Print table rows
    for (pid, process) in system.get_processes() {
        let cpu_percent = process.cpu_usage();
        let mem_percent = (process.memory() as f64 / system.get_total_memory() as f64) * 100.0;
        let status = match process.status() {
            sysinfo::ProcessStatus::Idle => "Idle",
            sysinfo::ProcessStatus::Run => "Run",
            sysinfo::ProcessStatus::Sleep => "Sleep",
            sysinfo::ProcessStatus::Stop => "Stop",
            sysinfo::ProcessStatus::Zombie => "Zombie",
            _ => "Unhandled status",

        };

        println!(
            "{:<8} {:<20} {:<15.2} {:<15.2} {:<10}",
            pid,
            process.name(),
            cpu_percent,
            mem_percent,
            status
        );
    }
}



