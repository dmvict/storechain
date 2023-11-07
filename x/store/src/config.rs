use serde::Deserialize;
/// Setup app.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub pg_url: String,
}
