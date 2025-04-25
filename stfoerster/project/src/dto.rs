use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Aktiv,
    Inaktiv,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Benutzer {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub ist_admin: bool,
    pub tags: Vec<String>,
    pub status: Status,
}