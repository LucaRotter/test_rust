use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub testo: String,
    pub completato: bool,
}

impl Task{
    
}

#[derive(Debug)]
pub enum TaskError{
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    NotFound(u32),
    InvalidInput,
}