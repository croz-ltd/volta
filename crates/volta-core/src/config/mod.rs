use serde::Deserialize;
pub mod load;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Mirrors {
    /// Base URL for Node.js distribution (e.g., https://nodejs.org/dist or a mirror)
    pub node: Option<String>,
    pub registry: Option<String>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct Settings {
    #[serde(default)]
    pub mirrors: Mirrors,
}
