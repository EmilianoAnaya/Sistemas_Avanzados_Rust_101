// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::State;
use sysinfo::{System, SystemExt, CpuExt};
use std::sync::Mutex;

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
        .invoke_handler(tauri::generate_handler![get_cpu_usage])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
