// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::sync::Mutex;
use tauri::State;
use serde::{Serialize, Deserialize};
// use wmi::{COMLibrary, WMIConnection};
use sysinfo::{CpuExt, NetworkExt, System, SystemExt};
use systemstat::{System as SysStat, Platform};


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
struct MemoryStats {
    physic : u64,
    swap : u64,
    cache : u64
}

#[tauri::command]
fn get_network_stats() -> NetworkStats{
    let mut system = sysinfo::System::new_all();
    system.refresh_networks();

    let stats = SysStat::new();
    
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

#[tauri::command]
fn get_memory_stats(state: State<'_, Mutex<System>>) -> MemoryStats {
    let mut system = state.lock().unwrap();
    system.refresh_memory();
    MemoryStats { 
        physic: system.used_memory() / (1024 * 1024),
        swap: system.used_swap() / (1024 * 1024),
        cache: system.free_memory() / (1024 * 1024)
    }
}

#[tauri::command]
fn get_cpu_usage(state: State<'_, Mutex<System>>) -> f32 {
    let mut sys = state.lock().unwrap();
    sys.refresh_cpu();
    sys.global_cpu_info().cpu_usage()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let system = Mutex::new(System::new_all());

    tauri::Builder::default()
        .manage(system)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_cpu_usage, 
                                                 get_memory_stats,
                                                 get_network_stats])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
