use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    id: u32,
    testo: String,
    completato: bool,
}