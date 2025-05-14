use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ProcessStat {
    pub name : String,
    pub cpu : f32,
    pub memory : f32
}