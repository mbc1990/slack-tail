use openapi::apis::configuration::Configuration;
use openapi::apis::conversations_api;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver};
use tokio::time::delay_for;
use std::time::{Duration};


pub struct SlackClient {
    configuration: Configuration
}

impl SlackClient  {
    pub fn new(oauth_access_token: &str) -> SlackClient {
        let mut configuration = Configuration::new();
        configuration.oauth_access_token = Some(oauth_access_token.to_string());
        SlackClient {configuration: configuration}
    }

    pub fn tail_channel(&self, channel: String) -> Receiver<String> {
        println!("Beginning tail channel");
        let (tx, rx) = mpsc::channel();
        let my_conf = self.configuration.clone();
        let my_channel = channel.clone();
        tokio::spawn(async move {
            println!("Querying channel");
            tx.send("test message 1".to_string());
            let last_message_timestamp = None;
            loop {
                tx.send("test message 2".to_string());
                let history_result = conversations_api::conversations_history(
                    &my_conf,
                    Some(false),
                    None,
                    None,
                    Some(100),
                    last_message_timestamp,
                    Some(&my_channel),
                    None
                ).await;

                match history_result {
                    Ok(resp) => {
                        tx.send("test message 3".to_string());
                        println!("Got a response!");
                        // TODO: Iterate and write to channel
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
