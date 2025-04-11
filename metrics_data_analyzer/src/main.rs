use std::{collections::HashMap, fs, vec};
use serde::{Deserialize, Serialize};
use serde_json;
use plotters::prelude::*;

#[derive(Serialize, Deserialize)]
struct JSONData {
    #[serde(rename = "Time")]
    time : String,
    #[serde(rename = "CPU")]
    cpu : CPUStats,
    #[serde(rename = "Memory")]
    memory : MemoryStats,
    #[serde(rename = "Network")]
    network : NetworkStats,
    #[serde(rename = "Disk")]
    disk : DiskStats,
    #[serde(rename = "Proccess")]
    proccess : Vec<String>
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

static PLOTS_ROUTE: &str = "plots/";

fn plot_usage(times: &Vec<String>, usage_average: Vec<f32>) -> Result<(), Box<dyn std::error::Error>> {
    static USAGE: &str = "cores_avg_usage.png";

    let path = format!("{}{}", PLOTS_ROUTE, USAGE);
    let root = BitMapBackend::new(&path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_y = usage_average.iter().cloned().fold(0./0., f32::max).ceil() as i32 + 10;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .caption("Uso Promedio del CPU (%)", ("sans-serif", 40))
        .margin(20)
        .build_cartesian_2d(0..times.len(), 0..max_y)?;

    chart
        .configure_mesh()
        .x_labels((times.len() / 30).max(2)) // máximo 10 etiquetas o mínimo 2
        .x_label_formatter(&|idx| times.get(*idx).cloned().unwrap_or_default())        
        .y_desc("Uso de CPU (%)")
        .x_desc("Fecha/Hora")
        .axis_desc_style(("sans-serif", 20))
        .label_style(("sans-serif", 9))
        .draw()?;

    chart.draw_series(
        usage_average.iter().enumerate().map(|(i, &val)| {
            Rectangle::new(
                [(i, 0), (i + 1, val.round() as i32)],
                RED.mix(0.5).filled(),
            )
        })
    )?;

    Ok(())
}

fn plot_frequency(times: &Vec<String>, metrics: &Vec<JSONData>) -> Result<(), Box<dyn std::error::Error>> {
    static FREQUENCY: &str = "cpu_frequency.png";
    let path = format!("{}{}", PLOTS_ROUTE, FREQUENCY);

    let root = BitMapBackend::new(&path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let max_y = metrics.iter()
        .map(|data| data.cpu.frequency) // Obtener frequency de cada CPUStats
        .fold(f64::NAN, f64::max)       // Encontrar el máximo
        .ceil() as i32 + 100;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .caption("Frecuencia del CPU (MHz)", ("sans-serif", 40))
        .margin(20)
        .build_cartesian_2d(0..times.len(), 0..max_y)?;

    chart
        .configure_mesh()
        .x_labels((times.len() / 30).max(2)) // máximo 10 etiquetas o mínimo 2
        .x_label_formatter(&|idx| times.get(*idx).cloned().unwrap_or_default())
        .y_desc("Frecuencia (MHz)")
        .x_desc("Fecha/Hora")
        .axis_desc_style(("sans-serif", 20))
        .label_style(("sans-serif", 9))
        .draw()?;

    chart.draw_series(LineSeries::new(
        metrics.iter().enumerate().map(|(i, data)| (i, data.cpu.frequency.round() as i32)),
        &BLUE,
    ))?
    .label("Frecuencia CPU")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Dibuja leyenda
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .label_font(("sans-serif", 15))
        .draw()?;

    Ok(())
}

fn plot_memory(times: &Vec<String>, metrics: &Vec<JSONData>) -> Result<(), Box<dyn std::error::Error>> {
    static MEMORY: &str = "memory.png";
    let path = format!("{}{}", PLOTS_ROUTE, MEMORY);

    let root = BitMapBackend::new(&path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let max_y = (metrics.iter()
        .flat_map(|data| vec![data.memory.physic, data.memory.swap, data.memory.cache])
        .max()
        .unwrap_or(0) as f64)
        .ceil() as i32 + 1000;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .caption("Uso de Memoria (KB)", ("sans-serif", 40))
        .margin(20)
        .build_cartesian_2d(0..times.len(), 0..max_y)?;

    chart
        .configure_mesh()
        .x_labels((times.len() / 30).max(2)) // máximo 10 etiquetas o mínimo 2
        .x_label_formatter(&|idx| times.get(*idx).cloned().unwrap_or_default())
        .y_desc("Memoria (KB)")
        .x_desc("Fecha/Hora")
        .axis_desc_style(("sans-serif", 20))
        .label_style(("sans-serif", 9))
        .draw()?;

    chart.draw_series(LineSeries::new(
        metrics.iter().enumerate().map(|(i, data)| (i, data.memory.physic as i32)),
        &RED,
    ))?
    .label("Memoria Física")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.draw_series(LineSeries::new(
        metrics.iter().enumerate().map(|(i, data)| (i, data.memory.swap as i32)),
        &BLUE,
    ))?
    .label("Memoria Swap")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart.draw_series(LineSeries::new(
        metrics.iter().enumerate().map(|(i, data)| (i, data.memory.cache as i32)),
        &GREEN,
    ))?
    .label("Memoria Cache")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    // Dibuja leyenda
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .label_font(("sans-serif", 15))
        .draw()?;

    Ok(())
}

fn plot_disk_mbps(times: &Vec<String>, metrics: &Vec<JSONData>) -> Result<(), Box<dyn std::error::Error>> {
    static DISK_MBPS: &str = "read_write_speeds.png";
    let path = format!("{}{}", PLOTS_ROUTE, DISK_MBPS);

    let root = BitMapBackend::new(&path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_y = metrics.iter()
        .flat_map(|data| vec![data.disk.read_mbps, data.disk.write_mbps])
        .fold(f64::NAN, f64::max)
        .ceil() as i32 + 5;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .caption("Lectura/Escritura Disco (Mbps)", ("sans-serif", 40))
        .margin(20)
        .build_cartesian_2d(0..times.len(), 0..max_y)?;

    chart
        .configure_mesh()
        .x_labels((times.len() / 30).max(2)) // máximo 10 etiquetas o mínimo 2
        .x_label_formatter(&|idx| times.get(*idx).cloned().unwrap_or_default())
        .y_desc("Velocidades (Mbps)")
        .x_desc("Fecha/Hora")
        .axis_desc_style(("sans-serif", 20))
        .label_style(("sans-serif", 9))
        .draw()?;

    chart.draw_series(LineSeries::new(
        metrics.iter().enumerate().map(|(i, data)| (i, data.disk.read_mbps as i32)),
        &RED,
    ))?
    .label("Lectura (Mbps)")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.draw_series(LineSeries::new(
        metrics.iter().enumerate().map(|(i, data)| (i, data.disk.write_mbps as i32)),
        &BLUE,
    ))?
    .label("Escritura (Mbps)")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Dibuja leyenda
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .label_font(("sans-serif", 15))
        .draw()?;

    Ok(())
}

fn plot_disk_iops(times: &Vec<String>, metrics: &Vec<JSONData>) -> Result<(), Box<dyn std::error::Error>>{
    static DISK_IOPS: &str = "read_write_iops.png";
    let path = format!("{}{}", PLOTS_ROUTE, DISK_IOPS);

    let root = BitMapBackend::new(&path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_y = metrics.iter()
        .flat_map(|data| vec![data.disk.iops_read, data.disk.iops_write])
        .max()
        .unwrap_or(0) as i32 + 50;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .caption("Operaciones E/S (IOPS)", ("sans-serif", 40))
        .margin(20)
        .build_cartesian_2d(0..times.len(), 0..max_y)?;

    chart
        .configure_mesh()
        .x_labels((times.len() / 30).max(2)) // máximo 10 etiquetas o mínimo 2
        .x_label_formatter(&|idx| times.get(*idx).cloned().unwrap_or_default())
        .y_desc("Operaciones E/S (IOPS)")
        .x_desc("Fecha/Hora")
        .axis_desc_style(("sans-serif", 20))
        .label_style(("sans-serif", 9))
        .draw()?;

    chart.draw_series(LineSeries::new(
        metrics.iter().enumerate().map(|(i, data)| (i, data.disk.iops_read as i32)),
        &RED,
    ))?
    .label("Lectura (IOPS)")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.draw_series(LineSeries::new(
        metrics.iter().enumerate().map(|(i, data)| (i, data.disk.iops_write as i32)),
        &BLUE,
    ))?
    .label("Escritura (IOPS)")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Dibuja leyenda
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .label_font(("sans-serif", 15))
        .draw()?;

    Ok(())
}

fn plot_network(times: &Vec<String>, metrics: &Vec<JSONData>) -> Result<(), Box<dyn std::error::Error>>{
    static NETWORK: &str = "network.png";
    let path = format!("{}{}", PLOTS_ROUTE, NETWORK);

    let root = BitMapBackend::new(&path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_y = metrics.iter()
        .flat_map(|data| vec![data.network.received, data.network.transmitted])
        .fold(f64::NAN, f64::max)
        .ceil() as i32 + 100;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .caption("Tráfico de Red (Kb)", ("sans-serif", 40))
        .margin(20)
        .build_cartesian_2d(0..times.len(), 0..max_y)?;

    chart
        .configure_mesh()
        .x_labels((times.len() / 30).max(2)) // máximo 10 etiquetas o mínimo 2
        .x_label_formatter(&|idx| times.get(*idx).cloned().unwrap_or_default())
        .y_desc("Tráfico (Kb)")
        .x_desc("Fecha/Hora")
        .axis_desc_style(("sans-serif", 20))
        .label_style(("sans-serif", 9))
        .draw()?;

    chart.draw_series(LineSeries::new(
        metrics.iter().enumerate().map(|(i, data)| (i, data.network.received as i32)),
        &RED,
    ))?
    .label("Recibido (Kb)")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.draw_series(LineSeries::new(
        metrics.iter().enumerate().map(|(i, data)| (i, data.network.transmitted as i32)),
        &BLUE,
    ))?
    .label("Transmitido (Kb)")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Dibuja leyenda
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .label_font(("sans-serif", 15))
        .draw()?;
    
    Ok(())
}

fn plot_connections(times: &Vec<String>, metrics: &Vec<JSONData>) -> Result<(), Box<dyn std::error::Error>> {
    static CONNECTIONS: &str = "connections.png";
    let path = format!("{}{}", PLOTS_ROUTE, CONNECTIONS);

    let root = BitMapBackend::new(&path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_y = metrics.iter()
                    .flat_map(|data| vec![data.network.active])
                    .max()
                    .unwrap_or(0) as u32 + 10;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .caption("Conexiones Activas de Red", ("sans-serif", 40))
        .margin(20)
        .build_cartesian_2d(0..times.len(), 0..max_y)?;

    chart
        .configure_mesh()
        .x_labels((times.len() / 30).max(2)) // máximo 10 etiquetas o mínimo 2
        .x_label_formatter(&|idx| times.get(*idx).cloned().unwrap_or_default())        
        .y_desc("Conexiones Activas")
        .x_desc("Fecha/Hora")
        .axis_desc_style(("sans-serif", 20))
        .label_style(("sans-serif", 9))
        .draw()?;

    
    chart.draw_series(
        metrics.iter().enumerate().map(|(i, data)| {
            Rectangle::new(
                [(i, 0), (i + 1, data.network.active as u32)],
                RED.mix(0.5).filled(),
            )
        })
    )?;
    
    Ok(())
}

fn plot_proccess(metrics: &Vec<JSONData>) -> Result<(), Box<dyn std::error::Error>> {
    static PROCCESS: &str = "process.png";
    let path = format!("{}{}", PLOTS_ROUTE, PROCCESS);

    let root = BitMapBackend::new(&path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut unique_process = HashMap::new();
    for data in metrics {
        for process in &data.proccess {
            let process_name = process.split(" - ")
                                                .next()
                                                .unwrap_or(process)
                                                .to_string();
            *unique_process.entry(process_name).or_insert(0) += 1;
        }
    }

    let mut process_frequency: Vec<(String, u32)> = unique_process.into_iter().collect();
    process_frequency.sort_by(|a, b|b.1.cmp(&a.1));
    

    let max_x = process_frequency.iter().map(|(_, count)| *count).max().unwrap_or(0) + 1;

    // Crear el gráfico con índices en el eje X
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(60)
        .y_label_area_size(150) // Más espacio para nombres largos en el eje Y
        .caption("Frecuencia de Procesos", ("sans-serif", 40))
        .margin(20)
        .build_cartesian_2d(0..max_x, 0..process_frequency.len())?;

    chart
        .configure_mesh()
        .y_labels(process_frequency.len().min(50)) // Limitar etiquetas si hay muchas
        .y_label_formatter(&|idx| {
            process_frequency.get(*idx as usize)
                .map(|(name, _)| name.clone())
                .unwrap_or_default()
        })
        .y_label_style(("sans-serif", 10))
        .x_desc("Frecuencia de Aparición")
        .y_desc("Procesos")
        .axis_desc_style(("sans-serif", 20))
        .label_style(("sans-serif", 9))
        .draw()?;

    // Dibujar barras horizontales
    chart.draw_series(
        process_frequency.iter().enumerate().map(|(i, (_, count))| {
            Rectangle::new(
                [(0, i), (*count, i + 1)], // Barra horizontal: desde (0, i) hasta (count, i + 1)
                GREEN.mix(0.5).filled(),
            )
        })
    )?;


    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    static FILE_PATH: &str = "metrics.json";


    let json_data = fs::read_to_string(FILE_PATH)?;
    let metrics: Vec<JSONData> = serde_json::from_str(&json_data)?;

    // Data to work for plotting
    let mut times: Vec<String> = Vec::new();
    let mut usage_average: Vec<f32> = Vec::new();

    for data in &metrics {
        let trimmed_time = data.time[..data.time.len() - 6].trim().to_string();
        times.push(trimmed_time);
        
        let average: f32 = if !data.cpu.usage.is_empty(){
            let sum: f32 = data.cpu.usage.iter().sum();
            sum / data.cpu.usage.len() as f32
        } else {
            0.0
        };

        usage_average.push(average);
    }

    plot_usage(&times, usage_average)?;
    plot_frequency(&times, &metrics)?;
    plot_memory(&times, &metrics)?;
    plot_disk_mbps(&times, &metrics)?;
    plot_disk_iops(&times, &metrics)?;
    plot_network(&times, &metrics)?;
    plot_connections(&times, &metrics)?;
    plot_proccess(&metrics)?;

    Ok(())
}
