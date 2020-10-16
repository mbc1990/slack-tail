use openapi::apis::configuration::Configuration;
use openapi::apis::conversations_api;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use tokio::time::sleep;
use std::time::{Duration};
use serde_json::Value;
use std::collections::HashMap;
use tokio::task::JoinHandle;


pub struct SlackClient {
    configuration: Configuration,
}

fn tail_channel_to_existing_tx(conf: Configuration, channel_id: String, tx: Sender<Value>) -> JoinHandle<()> {
    let my_conf = conf.clone();
    let my_channel = channel_id.clone();
    let handle = tokio::spawn(async move {
        println!("Querying channel");
        let mut last_message_timestamp = None;
        loop {
            let history_result = conversations_api::conversations_history(
                &my_conf,
                None,
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

                        // First message is newest, so it becomes the oldest timestamp for the next query
                        if !updated_offset {
                            let ts = message.get("ts").unwrap().as_str().unwrap();
                            let my_ts = ts.to_string();
                            last_message_timestamp = Some(my_ts);
                            updated_offset = true;
                        }
                        tx.send(message.clone());
                    }
                },
                Err(err) => {
                    println!("Error from slack api {:?}", err);
                }
            }
            sleep(Duration::from_millis(100)).await;
        }
    });
    return handle;
}

impl SlackClient  {
    pub fn new(oauth_access_token: &str) -> SlackClient {
        let mut configuration = Configuration::new();
        configuration.oauth_access_token = Some(oauth_access_token.to_string());
        SlackClient {configuration: configuration}
    }


    // Tails all channels the bot belongs to
    pub fn tail_member_of(&mut self) -> Receiver<Value> {
        let (tx, rx) = mpsc::channel();
        let loops_conf = self.configuration.clone();
        let task = tokio::spawn(async move {
            let mut live_api_pollers: HashMap<String, JoinHandle<()>> = HashMap::new();
            loop {
                let my_conf = loops_conf.clone();
                let list_result = conversations_api::conversations_list(
                    &my_conf,
                    None,
                    None,
                    Some(1000),
                    None,
                    None
                ).await;

                let list = list_result.unwrap();
                let channels = list.get("channels").unwrap().as_array().unwrap();
                let bot_in_channels: Vec<String> = channels.iter()
                    .filter(|channel| {
                        let member = channel.get("is_member").unwrap().as_bool().unwrap();
                        return member;
                    })
                    .map(|channel| {
                        return channel.get("id").unwrap().as_str().unwrap().to_string();
                    })
                    .collect();
                println!("Got a list result");

                let current_polled = live_api_pollers.keys();
                current_polled.for_each(|polled_channel_id| {
                    if !bot_in_channels.contains(polled_channel_id) {
                        // Kill task if we're no longer in the channel
                        live_api_pollers.get(polled_channel_id).unwrap().abort();
                    }
                });
                bot_in_channels.iter()
                    .for_each(|to_poll_channel_id| {
                        if !live_api_pollers.contains_key(to_poll_channel_id) {
                            // Start task if we're in channel
                            let handle = tail_channel_to_existing_tx(loops_conf.clone(), to_poll_channel_id.to_string().clone(), tx.clone());
                            live_api_pollers.insert(to_poll_channel_id.to_string(), handle);
                        }
                    });
                sleep(Duration::from_millis(100)).await;
            }
        });
        return rx;
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
                    None,
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

                            // First message is newest, so it becomes the oldest timestamp for the next query
                            if !updated_offset {
                                let ts = message.get("ts").unwrap().as_str().unwrap();
                                let my_ts = ts.to_string();
                                last_message_timestamp = Some(my_ts);
                                updated_offset = true;
                            }
                            tx.send(message.clone());
                        }
                    },
                    Err(err) => {
                        println!("Error from slack api {:?}", err);
                    }
                }
                sleep(Duration::from_millis(100)).await;
            }
        });
        return rx;
    }
}
