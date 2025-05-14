use crate::models::NetworkStats;
use win_toast_notify::WinToastNotify;


pub fn detect_ddos_attack(network_stats : &NetworkStats, threshold_received: f64, threshold_active : u64) {
    let active_connections : u64 = network_stats.active;
    let data_received : f64 = network_stats.received;

    if network_stats.received > threshold_received || network_stats.active > threshold_active {
        let message = format!(
            "{:.2} Mgps recibidos - {} conexiones activas",
            data_received, active_connections
        );

        if let Err(e) = WinToastNotify::new()
            .set_title("Posible DDoS detectado!")
            .set_messages(vec![&message])
            .show()
        {
            eprintln!("Error al mostrar notificación: {:?}", e);
        }
    }
}