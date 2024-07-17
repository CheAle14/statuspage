use serde::Deserialize;

use crate::{component::Component, incident::Incident, status::Status};

#[derive(Debug, Deserialize)]
pub struct Summary {
    pub components: Vec<Component>,
    pub incidents: Vec<Incident>,
    pub status: Status,
}
