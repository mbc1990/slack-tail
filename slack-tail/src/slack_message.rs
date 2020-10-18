use serde_json::Value;

#[derive(Debug, Clone)]
pub struct SlackMessage {
    pub body: Value,
    pub channel: String
}