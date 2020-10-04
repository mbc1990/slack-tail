use openapi::apis::configuration::Configuration;
use openapi::apis::oauth_api::oauth_access;
use openapi::apis::channels_api;
use openapi::apis::conversations_api;
use openapi::models::*;
use tokio::runtime::Runtime;


pub struct SlackClient {
    configuration: Configuration,
    runtime: Runtime
}
impl SlackClient  {
    pub fn new(oauth_access_token: &str) -> SlackClient {
        let mut runtime = Runtime::new().expect("Failed to create Tokio runtime");
        let mut configuration = Configuration::new();
        configuration.oauth_access_token = Some(oauth_access_token.to_string());
        SlackClient {configuration, runtime}
    }

    // TODO: Tail #channel - returns a channel
    // TODO: Send message, takes channel, string
}
