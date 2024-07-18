use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentStatus {
    Operational,
    UnderMaintenance,
    DegradedPerformance,
    PartialOutage,
    MajorOutage,
}

#[derive(Debug, Deserialize)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub status: ComponentStatus,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub position: i32,
    pub description: Option<String>,
    pub showcase: bool,
    pub start_date: Option<NaiveDate>,
    pub group_id: Option<String>,
    pub page_id: String,
    pub group: Option<bool>,
    pub only_show_if_degraded: Option<bool>,
    #[serde(default)]
    pub components: Vec<String>,
}
