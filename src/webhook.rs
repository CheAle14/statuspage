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
    Unknown(serde::de::IgnoredAny),
}

#[derive(Debug, Deserialize)]
pub struct StatusWebhook {
    pub meta: WebhookMeta,
    pub page: WebhookPage,
    #[serde(flatten)]
    pub payload: WebhookPayload,
}

#[cfg(test)]
mod tests {
    use crate::{
        component::ComponentStatus,
        incident::{IncidentImpact, IncidentStatus},
    };

    use super::StatusWebhook;

    const INCIDENT_1: &str = include_str!("../webhooks/incident1.json");
    const INCIDENT_2: &str = include_str!("../webhooks/incident2.json");
    const COMPONENT_1: &str = include_str!("../webhooks/component1.json");
    const COMPONENT_2: &str = include_str!("../webhooks/component2.json");

    const INVALID: &str = include_str!("../webhooks/invalid.json");

    #[test]
    pub fn test_parse_first_component() {
        let component: StatusWebhook = serde_json::from_str(COMPONENT_1).unwrap();
        match component.payload {
            super::WebhookPayload::Incident { .. } => {
                panic!("unexpected incident pyload")
            }
            super::WebhookPayload::Unknown(_) => panic!("unexpected unknown payload"),
            super::WebhookPayload::Component {
                component_update,
                component,
            } => {
                assert_eq!(component.id, "rhznvxg4v7yh");
                assert_eq!(
                    component_update.new_status,
                    ComponentStatus::DegradedPerformance
                );
            }
        }
    }

    #[test]
    pub fn test_parse_second_component() {
        let component: StatusWebhook = serde_json::from_str(COMPONENT_2).unwrap();
        match component.payload {
            super::WebhookPayload::Incident { .. } => {
                panic!("unexpected incident pyload")
            }
            super::WebhookPayload::Unknown(_) => panic!("unexpected unknown payload"),
            super::WebhookPayload::Component {
                component_update,
                component,
            } => {
                assert_eq!(component.id, "rhznvxg4v7yh");
                assert_eq!(component_update.new_status, ComponentStatus::Operational);
                assert_eq!(
                    component_update.old_status,
                    ComponentStatus::DegradedPerformance
                );
            }
        }
    }

    #[test]
    pub fn test_parse_first_incident() {
        let incident: StatusWebhook = serde_json::from_str(INCIDENT_1).unwrap();

        match incident.payload {
            super::WebhookPayload::Component { .. } => panic!("unexpected component update"),
            super::WebhookPayload::Unknown(_) => panic!("unexpected unknown"),
            super::WebhookPayload::Incident { incident } => {
                assert_eq!(incident.id, "cc5gchpvgyhh");
                assert_eq!(incident.impact, IncidentImpact::Minor);
                assert_eq!(incident.status, IncidentStatus::Identified);
                assert_eq!(incident.name, "Issue with Polls Ending Properly");
            }
        }
    }

    #[test]
    pub fn test_parse_second_incident() {
        let incident: StatusWebhook = serde_json::from_str(INCIDENT_2).unwrap();

        match incident.payload {
            super::WebhookPayload::Component { .. } => panic!("unexpected component update"),
            super::WebhookPayload::Unknown(_) => panic!("unexpected unknown"),
            super::WebhookPayload::Incident { incident } => {
                assert_eq!(incident.id, "cc5gchpvgyhh");
                assert_eq!(incident.impact, IncidentImpact::Minor);
                assert_eq!(incident.name, "Issue with Polls Ending Properly");
                assert_eq!(incident.status, IncidentStatus::Resolved);
            }
        }
    }

    #[test]
    pub fn test_parse_invalid_incident() {
        let incident: StatusWebhook = serde_json::from_str(INVALID).unwrap();

        match incident.payload {
            super::WebhookPayload::Component { .. } => panic!("unexpected component"),
            super::WebhookPayload::Incident { .. } => panic!("unexpected incident"),
            super::WebhookPayload::Unknown(_) => (),
        }
    }
}
