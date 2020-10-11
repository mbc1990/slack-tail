use openapi::apis::configuration::Configuration;
use openapi::apis::conversations_api;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver};
use tokio::time::delay_for;
use std::time::{Duration};
use serde_json::Value;


pub struct SlackClient {
    configuration: Configuration
}

impl SlackClient  {
    pub fn new(oauth_access_token: &str) -> SlackClient {
        let mut configuration = Configuration::new();
        configuration.oauth_access_token = Some(oauth_access_token.to_string());
        SlackClient {configuration: configuration}
    }

    pub fn tail_channel(&self, channel: String) -> Receiver<Value> {
        println!("Beginning tail channel");
        let (tx, rx) = mpsc::channel();
        let my_conf = self.configuration.clone();
        let my_channel = channel.clone();
        tokio::spawn(async move {
            println!("Querying channel");
            let mut last_message_timestamp = None;
            loop {
                let history_result = conversations_api::conversations_history(
                    &my_conf,
                    Some(false),
                    None,
                    None,
                    Some(100),
                    last_message_timestamp.clone(),
                    Some(&my_channel),
                    None
                ).await;

                match history_result {
                    Ok(resp) => {
                        let messages = resp.get("messages").unwrap().as_array().unwrap();
                        let mut updated_offset = false;
                        for message in messages {
                            if (!updated_offset) {
                                let ts = message.get("ts").unwrap().as_str().unwrap();
                                let my_ts = ts.to_string();
                                last_message_timestamp = Some(my_ts);
                                updated_offset = true;
                            }

                            // TODO: We're getting duplicates of the most recent message sent every query
                            tx.send(message.clone());
                        }
                    },
                    Err(err) => {
                        println!("Error from slack api {:?}", err);
                    }
                }
                delay_for(Duration::from_millis(100)).await;
            }
        });
        return rx;
    }
}
