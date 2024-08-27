use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use crate::component::{Component, ComponentStatus};

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum IncidentStatus {
    Investigating,
    Identified,
    Monitoring,
    Resolved,
    Postmortem,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum IncidentImpact {
    None,
    Maintenance,
    Minor,
    Major,
    Critical,
}

#[derive(Debug, Deserialize)]
pub struct AffectedComponent {
    pub code: String,
    pub name: String,
    pub old_status: ComponentStatus,
    pub new_status: ComponentStatus,
}

#[derive(Debug, Deserialize)]
pub struct IncidentUpdate {
    pub id: String,
    pub status: IncidentStatus,
    pub body: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub display_at: DateTime<FixedOffset>,
    pub deliver_notifications: Option<bool>,
    #[serde(default, deserialize_with = "crate::utils::deserialize_null_default")]
    pub affected_components: Vec<AffectedComponent>,
}

#[derive(Debug, Deserialize)]
pub struct Incident {
    pub id: String,
    pub name: String,
    pub status: IncidentStatus,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: Option<DateTime<FixedOffset>>,
    pub monitoring_at: Option<DateTime<FixedOffset>>,
    pub resolved_at: Option<DateTime<FixedOffset>>,
    pub impact: IncidentImpact,
    pub shortlink: String,
    pub started_at: Option<DateTime<FixedOffset>>,
    pub page_id: Option<String>,
    #[serde(default)]
    pub incident_updates: Vec<IncidentUpdate>,
    #[serde(default)]
    pub components: Vec<Component>,
}
