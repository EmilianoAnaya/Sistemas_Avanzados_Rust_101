#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use sysinfo::{CpuExt, NetworkExt, ProcessExt, System, SystemExt};
use systemstat::{System as SysStat, Platform};
use std::io::{Read, Seek, Write};
use std::{fs::OpenOptions, thread};
use std::time::Duration;
use wmi::{COMLibrary, WMIConnection};
use chrono::Local;
use serde_json;

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
struct CPUStats {
    usage : Vec<f32>,
    frequency : f64
}

#[derive(Serialize, Deserialize)]
struct MemoryStats {
    physic : u64,
    swap : u64,
    cache : u64

}

#[derive(Serialize, Deserialize)]
struct DiskStats {
    read_mbps: f64,
    write_mbps: f64,
    iops_read : u64,
    iops_write : u64
}

#[derive(Serialize, Deserialize)]
struct NetworkStats {
    received : f64,
    transmitted : f64,
    active : u64
}

#[derive(Serialize, Deserialize)]
struct JSONData {
    Time : String,
    CPU : CPUStats,
    Memory : MemoryStats,
    Network : NetworkStats,
    Disk : DiskStats,
    Proccess : Vec<String>
}

fn get_cpu_stats(system : &System) -> CPUStats {
    CPUStats {
        usage       : system.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
        frequency   : system.cpus().get(0).map_or(0.0, |cpu| cpu.frequency() as f64 / 1000.0),
    }
}

fn get_memory_stats(system : &System) -> MemoryStats {
    MemoryStats { 
        physic: system.used_memory() / 1024,
        swap: system.used_swap() / 1024,
        cache:system.free_memory() / 1024
    }
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

fn get_processes(system : &System) -> Vec<(String, f32)> {
    let mut processes: Vec<(String, f32)> = system.processes()
        .iter()
        .map(|(_, process)| (process.name().to_string(), process.cpu_usage()))
        .collect();

    processes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    processes.truncate(5);
    processes
}

fn get_network_stats(system : &System, stats : &SysStat) -> NetworkStats{
    let mut receive_bytes_total: u64 = 0;
    let mut transmitted_bytes_total: u64 = 0;

    for (_, network) in system.networks() {
        receive_bytes_total += network.received();
        transmitted_bytes_total += network.transmitted();
    };

    let received_data: f64 = (receive_bytes_total as f64 * 8.0) / 1000.0;
    let transmitted_data: f64 = (transmitted_bytes_total as f64 * 8.0) / 1000.0;

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

fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let file_path = "metrics.json";

    let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;
    
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut system = System::new_all();
    let stats = SysStat::new();
    let con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(con)?;

    loop {
        system.refresh_all();
        let cpu_stats = get_cpu_stats(&system);
        let memory_stats = get_memory_stats(&system);
        let network_stats = get_network_stats(&system, &stats);
        let top_processes = get_processes(&system);
        let disk_stats = get_disk_stats(&wmi_con).unwrap_or(DiskStats { 
            read_mbps: 0.0, 
            write_mbps: 0.0, 
            iops_read: 0, 
            iops_write: 0 
        });

        let new_data = JSONData {
            Time : Local::now().to_rfc2822(),
            CPU : CPUStats { 
                usage: (cpu_stats.usage), 
                frequency: (cpu_stats.frequency) 
            },
            Memory : MemoryStats { 
                physic: (memory_stats.physic), 
                swap: (memory_stats.swap), 
                cache: (memory_stats.cache) 
            },
            Network : NetworkStats { 
                received: (network_stats.received), 
                transmitted: (network_stats.transmitted), 
                active: (network_stats.active) 
            },
            Disk : DiskStats { 
                read_mbps: (disk_stats.read_mbps), 
                write_mbps: (disk_stats.write_mbps), 
                iops_read: (disk_stats.iops_read), 
                iops_write: (disk_stats.iops_write) 
            },
            Proccess : top_processes
                        .iter()
                        .map(|(name, usage)| format!("{} - {:.2}%", name, usage))
                        .collect()
        };

        content.clear();
        file.seek(std::io::SeekFrom::Start(0))?;
        file.read_to_string(&mut content)?;

        let mut all_data: Vec<JSONData> = if content.trim().is_empty(){
            Vec::new()
        } else {
            serde_json::from_str(&content)?
        };

        all_data.push(new_data);

        let file_update = serde_json::to_string_pretty(&all_data)?;

        file.set_len(0)?;
        file.seek(std::io::SeekFrom::Start(0))?;
        file.write_all(file_update.as_bytes())?;

        println!("Log Guardado el {}", Local::now().to_rfc2822());
        thread::sleep(Duration::from_secs(300));
    }
}