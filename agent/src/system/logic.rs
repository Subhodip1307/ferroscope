use crate::system::structures::__SysInfo;
use crate::system::structures::{__DiskData, __DiskIoStats, /*__Cprocesses,*/ __Memory};
use anyhow;
use procfs::Current;
use procfs::CurrentSI;
use procfs::KernelStats;
use procfs::Meminfo;
use procfs::Uptime;
use std::collections::HashMap;
use std::fs;
use sysinfo::System;
use tokio::time::{Duration, Instant, sleep};

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

pub async fn total_cpu_usage() -> anyhow::Result<f64> {
    // 1st sample
    let k1 = KernelStats::current()?;
    let total1 = cpu_total_ticks(&k1.total);
    let idle1 = k1.total.idle as u128 + k1.total.iowait.unwrap_or(0) as u128; // include iowait in idle if present

    sleep(Duration::from_millis(500)).await;

    // 2nd sample
    let k2 = KernelStats::current()?;
    let total2 = cpu_total_ticks(&k2.total);
    let idle2 = k2.total.idle as u128 + k2.total.iowait.unwrap_or(0) as u128;

    let total_delta = total2.saturating_sub(total1) as f64;
    let idle_delta = idle2.saturating_sub(idle1) as f64;

    if total_delta <= 0.0 {
        println!("Could not compute CPU usage (no change in counters)."); //need to fix this can't return 0.0 
        Ok(0.0)
    } else {
        let usage_frac = 1.0 - (idle_delta / total_delta);
        println!("CPU usage: {:.2}%", usage_frac * 100.0);
        Ok(usage_frac)
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

fn read_diskstats() -> Vec<__DiskData> {
    let content = fs::read_to_string("/proc/diskstats").expect("Cannot read /proc/diskstats");

    content
        .lines()
        .filter_map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() < 14 {
                return None;
            }
            let name = fields[2].to_string();

            // Filter: keep only whole disks (sda, nvme0n1, vda, hda, xvda)
            let is_disk = name.starts_with("nvme") && name.contains("n") && !name.contains("p")
                || (name.starts_with("sd")
                    || name.starts_with("vd")
                    || name.starts_with("hd")
                    || name.starts_with("xvd"))
                    && !name.chars().last().unwrap_or('x').is_ascii_digit();

            if !is_disk {
                return None;
            }

            Some(__DiskData {
                name,
                // reads_completed: fields[3].parse().unwrap_or(0),
                sectors_read: fields[5].parse().unwrap_or(0),
                // writes_completed: fields[7].parse().unwrap_or(0),
                sectors_written: fields[9].parse().unwrap_or(0),
            })
        })
        .collect()
}

fn calculate_speed(
    before: &[__DiskData],
    after: &[__DiskData],
    elapsed_secs: f64,
) -> Vec<(String, f64, f64)> {
    let before_map: HashMap<&str, &__DiskData> =
        before.iter().map(|d| (d.name.as_str(), d)).collect();

    after
        .iter()
        .filter_map(|after_disk| {
            let before_disk = before_map.get(after_disk.name.as_str())?;

            // Linux sector = 512 bytes
            let sector_size = 512.0;
            let read_mb = (after_disk.sectors_read - before_disk.sectors_read) as f64 * sector_size
                / 1_048_576.0
                / elapsed_secs;
            let write_mb = (after_disk.sectors_written - before_disk.sectors_written) as f64
                * sector_size
                / 1_048_576.0
                / elapsed_secs;

            Some((after_disk.name.clone(), read_mb, write_mb))
        })
        .collect()
}

pub async fn get_disk_io() -> Vec<__DiskIoStats> {
    // returns [(Disk_name,read_MB,write_MB)]
    // Snapshot 1
    let snapshot1 = read_diskstats();
    // let disk_count = snapshot1.len();
    let t0 = Instant::now();
    tokio::time::sleep(Duration::from_secs(1)).await;
    let elapsed = t0.elapsed().as_secs_f64();

    // Snapshot 2
    let snapshot2 = read_diskstats();
    let data = calculate_speed(&snapshot1, &snapshot2, elapsed)
        .iter()
        .map(|row| __DiskIoStats {
            name: row.0.clone(),
            read: row.1,
            write: row.2,
        })
        .collect();
    println!("sending disk data");
    data
}
