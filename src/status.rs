use serde::Deserialize;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StatusIndicator {
    None,
    Maintenance,
    Minor,
    Major,
    Critical,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    pub indicator: StatusIndicator,
    pub description: String,
}
