use sysinfo::System;
use systemstat::{System as SysStat, Platform};
// use std::os::linux::raw::stat;
use std::thread;
use std::time::Duration;

struct CPUStats {
    usage : Vec<f32>,
    frequency : f64
}

struct MemoryStats {
    physic : u64,
    swap : u64,
    cache : u64

}

struct DiskStats {
    iops : usize,
    tat : usize
}

fn get_cpu_stats(system : &System) -> CPUStats {
    let cpu_usage: Vec<f32> = system.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
    let cpu_frequency: f64 = system.cpus().get(0).map_or(0.0, |cpu| cpu.frequency() as f64 / 1000.0);

    CPUStats { usage : cpu_usage, frequency : cpu_frequency }
}

fn get_memory_stats(system : &System) -> MemoryStats {
    let memory_used = system.used_memory() / 1024;
    let memory_swap = system.used_swap() / 1024;
    let memory_cache = system.free_memory() / 1024;

    MemoryStats { physic: memory_used, swap: memory_swap, cache: memory_cache }
}

fn get_disk_stats(stats : &SysStat) -> DiskStats {
    if let Ok(map) = stats.block_device_statistics() {
        for (_device, stats) in map {
            return DiskStats {
                iops : stats.read_ios + stats.write_ios,
                tat : stats.read_ticks + stats.write_ticks
            };
        }
    }
    DiskStats { iops: 0, tat: 0 }
}


fn main() {
    let mut system = System::new_all();
    let stats = SysStat::new();

    loop {
        system.refresh_all();
        let cpu_stats = get_cpu_stats(&system);
        let memory_stats = get_memory_stats(&system);
        let disk_stats = get_disk_stats(&stats);

        println!("CPU Data");
        println!("{:?} Cores\n{} Frequency\n", cpu_stats.usage, cpu_stats.frequency);
        println!("Memory Data");
        println!("{} Physic\n{} Swap\n{} Cache\n", memory_stats.physic, memory_stats.swap, memory_stats.cache);
        println!("Disk Data");
        println!("{} IOPS\n{} TAT", disk_stats.iops, disk_stats.tat);

        thread::sleep(Duration::from_secs(5));
    }
}
