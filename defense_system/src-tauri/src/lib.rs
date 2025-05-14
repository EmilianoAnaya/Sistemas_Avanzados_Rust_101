// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#![allow(non_snake_case)]

mod utils;
mod models; // <--- Agrega esta línea

use crate::utils::cpu_spike::detect_cpu_spikes;
use crate::utils::memory_leak::detect_memory_leaks;
use crate::models::{ProcessStat, ProcessStatMem, NetworkStats};

use serde::{Serialize, Deserialize};
use utils::ddos_defense::detect_ddos_attack;
use wmi::{COMLibrary, WMIConnection};
use sysinfo::{CpuExt, NetworkExt, System, SystemExt, ProcessExt};
use systemstat::{System as SysStat, Platform};
use core::str;
use std::thread;
use std::time::Duration;


#[derive(Deserialize)]
struct DiskDeserialize {
    DiskReadBytesPerSec: u64,
    DiskWriteBytesPerSec: u64,
    DiskReadsPerSec: u32,
    DiskWritesPerSec: u32,
    Timestamp_PerfTime: u64,
    Frequency_PerfTime: u64,
}

#[derive(Serialize, Deserialize)]
struct DiskStats {
    read_mbps: f64,
    write_mbps: f64,
    iops_read : u64,
    iops_write : u64
}

#[derive(Serialize, Deserialize)]
struct MemoryStats {
    physic : u64,
    swap : u64,
    cache : u64
}

#[derive(Serialize, Deserialize)]
struct CPUStats {
    global : f32,
    per_core : Vec<f32>
}

#[derive(Serialize, Deserialize)]
struct MonitorData {
    CPU : CPUStats,
    Memory : MemoryStats,
    Network : NetworkStats,
    Disk : DiskStats,
    Proccess : Vec<ProcessStat>
}

fn get_processes_mem(system : &System) -> Vec<ProcessStatMem> {
    let mut processes: Vec<ProcessStatMem> = system
        .processes()
        .iter()
        .map(|(_, process)| ProcessStatMem {
            name: process.name().to_string(),
            memory: process.memory() as f32 / (1024.0 * 1024.0 * 1024.0)
        })
        // .filter(|p| p.cpu > 0.0) 
        .collect();

    processes.sort_by(|a, b| {
        b.memory
            .partial_cmp(&a.memory)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    processes.truncate(10);
    processes
}


fn get_processes_cpu(system : &System) -> Vec<ProcessStat> {
    let num_cores = system.cpus().len() as f32;
    let mut processes: Vec<ProcessStat> = system
        .processes()
        .iter()
        .map(|(_, process)| ProcessStat {
            name: process.name().to_string(),
            cpu: process.cpu_usage() / num_cores
        })
        // .filter(|p| p.cpu > 0.0) 
        .collect();

    processes.sort_by(|a, b| {
        b.cpu
            .partial_cmp(&a.cpu)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    processes.truncate(10);
    processes
}

fn get_disk_stats(wmi_con: &WMIConnection) -> Option<DiskStats> {
    let sample_1: Vec<DiskDeserialize> = wmi_con
        .raw_query("SELECT * FROM Win32_PerfRawData_PerfDisk_PhysicalDisk WHERE Name='_Total'")
        .ok()?;
    let first = sample_1.first()?;

    thread::sleep(Duration::from_secs(1));

    let sample_2: Vec<DiskDeserialize> = wmi_con
        .raw_query("SELECT * FROM Win32_PerfRawData_PerfDisk_PhysicalDisk WHERE Name='_Total'")
        .ok()?;
    let second = sample_2.first()?;

    let time_delta = (second.Timestamp_PerfTime - first.Timestamp_PerfTime) as f64 / second.Frequency_PerfTime as f64;
    if time_delta <= 0.0 {
        return None;
    }

    let read_bytes_delta = second.DiskReadBytesPerSec.saturating_sub(first.DiskReadBytesPerSec);
    let write_bytes_delta = second.DiskWriteBytesPerSec.saturating_sub(first.DiskWriteBytesPerSec);
    let read_iops_delta = second.DiskReadsPerSec.saturating_sub(first.DiskReadsPerSec);
    let write_iops_delta = second.DiskWritesPerSec.saturating_sub(first.DiskWritesPerSec);

    Some(DiskStats {
        read_mbps: (read_bytes_delta as f64 / time_delta) / (1024.0 * 1024.0),
        write_mbps: (write_bytes_delta as f64 / time_delta) / (1024.0 * 1024.0),
        iops_read: (read_iops_delta as f64 / time_delta) as u64,
        iops_write: (write_iops_delta as f64 / time_delta) as u64,
    })
}

fn get_network_stats(system : &System, stats : &SysStat) -> NetworkStats{
    let mut receive_bytes_total: u64 = 0;
    let mut transmitted_bytes_total: u64 = 0;

    for (_, network) in system.networks() {
        receive_bytes_total += network.received();
        transmitted_bytes_total += network.transmitted();
    };

    let received_data: f64 = (receive_bytes_total as f64 * 8.0) / 1_000_000.0;
    let transmitted_data: f64 = (transmitted_bytes_total as f64 * 8.0) / 1_000_000.0;

    let active_connections = match stats.socket_stats() {
        Ok(stats) => stats.tcp_sockets_in_use as u64,
        Err(_) => 0
    };

    NetworkStats { 
        received: received_data, 
        transmitted: transmitted_data, 
        active : active_connections
    }
}

fn get_memory_stats(system : &System) -> MemoryStats {
    MemoryStats { 
        physic: system.used_memory() / (1024 * 1024),
        swap: system.used_swap() / (1024 * 1024),
        cache: system.free_memory() / (1024 * 1024)
    }
}

fn get_cpu_usage(system : &System) -> CPUStats {
    CPUStats{
        global : system.global_cpu_info().cpu_usage(),
        per_core : system.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
    }
}

#[tauri::command]
// async fn start_monitoring(memThreshold : f32, cpuThreshold : f32, netThreshold : f32) -> Result<MonitorData, String> {
async fn start_monitoring() -> Result<MonitorData, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let mut system = System::new_all();
        let stats = SysStat::new();
        let con = COMLibrary::new().map_err(|e| format!("COM init error: {:?}", e))?;
        let wmi_con = WMIConnection::new(con).map_err(|e| format!("WMI error: {:?}", e))?;

        system.refresh_all();
        let cpu_stats = get_cpu_usage(&system);
        let memory_stats = get_memory_stats(&system);
        let network_stats = get_network_stats(&system, &stats);
        let top_processes_cpu = get_processes_cpu(&system);
        let top_process_mem = get_processes_mem(&system);
        let disk_stats = get_disk_stats(&wmi_con).unwrap_or(DiskStats { 
            read_mbps: 0.0, 
            write_mbps: 0.0, 
            iops_read: 0, 
            iops_write: 0 
        });

        detect_cpu_spikes(&top_processes_cpu, 80.0);
        detect_memory_leaks(&top_process_mem, 5.0);
        detect_ddos_attack(&network_stats, 100.0, 100);

        Ok(MonitorData { 
            CPU: cpu_stats, 
            Memory: memory_stats, 
            Network: network_stats, 
            Disk: disk_stats, 
            Proccess: top_processes_cpu
        })
    })
    .await
    .map_err(|e| format!("Thread panic: {:?}", e))?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_monitoring]) 
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
