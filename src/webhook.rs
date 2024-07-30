use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

use crate::{component::ComponentStatus, incident::Incident, status::StatusIndicator};

#[derive(Debug, Deserialize)]
pub struct WebhookMeta {
    pub unsubscribe: String,
    pub documentation: String,
}

#[derive(Debug, Deserialize)]
pub struct WebhookPage {
    pub id: String,
    pub status_indicator: StatusIndicator,
    pub status_description: String,
}

#[derive(Debug, Deserialize)]
pub struct WebhookComponentUpdate {
    pub created_at: DateTime<FixedOffset>,
    pub new_status: ComponentStatus,
    pub old_status: ComponentStatus,
    pub id: String,
    pub component_id: String,
}

#[derive(Debug, Deserialize)]
pub struct WebhookComponent {
    pub created_at: DateTime<FixedOffset>,
    pub id: String,
    pub name: String,
    pub status: ComponentStatus,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum WebhookPayload {
    Component {
        component_update: WebhookComponentUpdate,
        component: WebhookComponent,
    },
    Incident {
        incident: Incident,
    },
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct StatusWebhook {
    pub meta: WebhookMeta,
    pub page: WebhookPage,
    #[serde(flatten)]
    pub payload: WebhookPayload,
}
