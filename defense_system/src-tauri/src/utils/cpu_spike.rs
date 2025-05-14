use crate::models::ProcessStat;
use win_toast_notify::WinToastNotify;

pub fn detect_cpu_spikes(processes: &[ProcessStat], threshold: f32) {
    let spikes: Vec<&ProcessStat> = processes
        .iter()
        .filter(|p| p.cpu >= threshold)
        .collect();

    if let Some(first_spike) = spikes.first() {
        let message = format!(
            "{} está usando {:.1}% de CPU",
            first_spike.name, first_spike.cpu
        );

        if let Err(e) = WinToastNotify::new()
            .set_title("CPU Spike Detectado")
            .set_messages(vec![&message])
            .show()
        {
            eprintln!("Error al mostrar notificación: {:?}", e);
        }
    }
}
