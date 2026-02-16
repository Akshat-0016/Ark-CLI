//System functions overview

use sysinfo::{System, SystemExt, CpuExt};

pub async fn handle() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let used_mem = sys.used_memory() / 1024;

    format!("CPU: {:.2}% | RAM: {} MB", cpu_usage, used_mem)
}
