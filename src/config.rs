use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub anttp_url: String,
    pub anttp_mock_mode: bool,
    pub http_timeout: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            anttp_url: std::env::var("ANTP_URL")
                .unwrap_or_else(|_| "http://localhost:18888".to_string()),
            anttp_mock_mode: std::env::var("ANTP_MOCK_MODE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            http_timeout: 30,
        }
    }
}