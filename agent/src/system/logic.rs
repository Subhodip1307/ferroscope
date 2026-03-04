use crate::system::structures::__SysInfo;
use crate::system::structures::{/*__Cprocesses,*/ __Memory};
use anyhow;
use procfs::Current;
use procfs::CurrentSI;
use procfs::KernelStats;
use procfs::Meminfo;
use procfs::Uptime;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::{/*Disks,*/ System};

fn cpu_total_ticks(ct: &procfs::CpuTime) -> u128 {
    let mut total = ct.user as u128 + ct.nice as u128 + ct.system as u128 + ct.idle as u128;
    if let Some(v) = ct.iowait {
        total += v as u128;
    }
    if let Some(v) = ct.irq {
        total += v as u128;
    }
    if let Some(v) = ct.softirq {
        total += v as u128;
    }
    if let Some(v) = ct.steal {
        total += v as u128;
    }
    if let Some(v) = ct.guest {
        total += v as u128;
    }
    if let Some(v) = ct.guest_nice {
        total += v as u128;
    }
    total
}

pub fn total_cpu_usage() -> anyhow::Result<f64> {
    // 1st sample
    let k1 = KernelStats::current()?;
    let total1 = cpu_total_ticks(&k1.total);
    let idle1 = k1.total.idle as u128 + k1.total.iowait.unwrap_or(0) as u128; // include iowait in idle if present

    sleep(Duration::from_millis(500));

    // 2nd sample
    let k2 = KernelStats::current()?;
    let total2 = cpu_total_ticks(&k2.total);
    let idle2 = k2.total.idle as u128 + k2.total.iowait.unwrap_or(0) as u128;

    let total_delta = total2.saturating_sub(total1) as f64;
    let idle_delta = idle2.saturating_sub(idle1) as f64;

    if total_delta <= 0.0 {
        println!("Could not compute CPU usage (no change in counters).");//need to fix this can't return 0.0 
        return Ok(0.0);
    } else {
        let usage_frac = 1.0 - (idle_delta / total_delta);
        println!("CPU usage: {:.2}%", usage_frac * 100.0);
        return Ok(usage_frac);
    }
}

pub fn systeminfo() -> __SysInfo {
    let mut sys = System::new_all();
    sys.refresh_all();
    // let name
    __SysInfo {
        system_name: System::name(),
        kernel_version: System::kernel_version(),
        os_version: System::os_version(),
        uptime: System::uptime(),
        cpu_threads: sys.cpus().len(),
        cpu_vendor: sys.cpus()[0].brand().to_string(),
    }
}



pub fn memory_usage() -> anyhow::Result<__Memory> {
    let data = Meminfo::current()?;
    Ok(__Memory::new(data.mem_total, data.mem_available))
}

pub fn get_uptime() -> anyhow::Result<u64> {
    // returns uptime in sec
    let uptime = Uptime::current()?;
    Ok(uptime.uptime_duration().as_secs())
}

// pub fn current_process() -> anyhow::Result<__Cprocesses> {
//     let kernel = KernelStats::current()?;
//     Ok(__Cprocesses::new(
//         kernel.procs_running,
//         kernel.procs_blocked,
//         kernel.processes,
//     ))
// }
