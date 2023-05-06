use sysinfo::{ProcessExt, SystemExt};


pub fn get_process_data(system: &mut sysinfo::System) -> Vec<(i32, f32, f64, String, sysinfo::ProcessStatus)> {
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

