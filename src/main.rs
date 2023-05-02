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
        let mem_con = proc.memory_percent().unwrap();
        let cpu_con = proc.cpu_percent().unwrap();
        let process_name = proc.name().unwrap();
        let status = proc.status().unwrap();

        // Add the data for the current process to the rows vector
        rows.push((pid, total_time, mem_con, cpu_con, process_name, status));
    }

    // Print the table header
    println!("{:<10} {:<15} {:<15} {:<10} {:<15} {:<10}", "PID", "CPU Time", "Memory%", "CPU%", "Process Name", "Status");

    // Print the table rows
    for row in rows {
        println!("{:<10} {:<15} {:<15.2} {:<10.2} {:<15} {:<10}", row.0, row.1, row.2, row.3, row.4, format!("{:?}", row.5));
    }
}
