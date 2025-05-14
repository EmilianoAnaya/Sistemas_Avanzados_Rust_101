use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ProcessStat {
    pub name : String,
    pub cpu : f32,
}

#[derive(Serialize, Deserialize)]
pub struct ProcessStatMem {
    pub name : String,
    pub memory : f32
}

#[derive(Serialize, Deserialize)]
pub struct NetworkStats {
    pub received : f64,
    pub transmitted : f64,
    pub active : u64
}