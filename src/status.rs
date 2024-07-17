use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StatusIndicator {
    None,
    Minor,
    Major,
    Critical,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    pub indicator: StatusIndicator,
    pub description: String,
}
