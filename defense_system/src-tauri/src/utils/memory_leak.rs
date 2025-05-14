use crate::models::ProcessStat;
use win_toast_notify::WinToastNotify;

pub fn detect_memory_leaks(processes: &[ProcessStat], threshold_gb: f32) {
    for p in processes {
        println!("{} → {:.4} GB", p.name, p.memory);
    }

    let spikes: Vec<&ProcessStat> = processes
        .iter()
        .filter(|p| p.memory >= threshold_gb)
        .collect();

    if let Some(first_spike) = spikes.first() {
        let message = format!(
            "{} está usando {:.2} GB de memoria",
            first_spike.name,
            first_spike.memory
        );

        if let Err(e) = WinToastNotify::new()
            .set_title("Alerta de memoria")
            .set_messages(vec![&message])
            .show()
        {
            eprintln!("Error al mostrar notificación: {:?}", e);
        }
    }
}