use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Aktiv,
    Inaktiv,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Benutzer {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub ist_admin: bool,
    pub tags: Vec<String>,
    pub status: Status,
}