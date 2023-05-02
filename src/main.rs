use psutil::process::Process;

fn main() {
    let processes = psutil::process::processes().unwrap();
    let mut rows = vec![];

    // Get the data for each process
    for process in processes {
        let pid = process.expect("Failed to get process").pid();
        let mut proc = Process::new(pid).unwrap();
        let cpu_time = proc.cpu_times().unwrap();
        let total_time = cpu_time.user().as_nanos() + cpu_time.system().as_nanos() + 
            cpu_time.children_user().as_nanos() + cpu_time.children_system().as_nanos();
        let mem_con = proc.memory_percent().unwrap() * 100.0;
        let cpu_con = proc.cpu_percent().unwrap();

        // Add the data for the current process to the rows vector
        rows.push((pid, total_time, mem_con, cpu_con));
    }

    // Print the table header
    println!("{:<10} {:<10} {:<10} {:<10}", "PID", "CPU Time", "Memory Consumption", "CPU Consumption");

    // Print the table rows
    for row in rows {
        println!("{:<10} {:<10} {:<10} {:<10}", row.0, row.1, row.2, row.3);
    }
}
