use openapi::apis::configuration::Configuration;
use openapi::apis::oauth_api::oauth_access;
use openapi::apis::channels_api;
use openapi::apis::conversations_api;
use openapi::models::*;
use tokio::runtime::Runtime;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, RecvError};
use tokio::prelude::*;
use tokio::time::delay_for;
use std::time::{Duration, Instant};
use std::thread;
use crate::tail_task::TailTask;


pub struct SlackClient {
    configuration: Configuration,
    task_tx: Sender<TailTask>
}


impl SlackClient  {
    pub fn new(oauth_access_token: &str) -> SlackClient {
        let mut runtime = Runtime::new().expect("Failed to create Tokio runtime");
        let mut configuration = Configuration::new();
        configuration.oauth_access_token = Some(oauth_access_token.to_string());
        let (mut tx, mut rx) = mpsc::channel();
        thread::spawn(move ||{
            runtime.block_on(async{
                loop {
                    let task_result: Result<TailTask, RecvError> = rx.recv();
                    match task_result {
                        Ok(task) => {
                            println!("Task received {:?}", task);
                            task.output_tx.send("This would be the output from slack".to_string());
                        },
                        Err(e) => {
                            println!("Error receiving {:?}", e.to_string());
                        }
                    }
                }
            });
        });
        SlackClient {configuration: configuration, task_tx: tx}
    }

    pub fn tail_channel(&self, channel: String) -> Receiver<String> {
        println!("Beginning tail channel");
        let (mut tx, mut rx) = mpsc::channel();
        let tail_task = TailTask{slack_channel_id: channel, output_tx: tx};
        self.task_tx.send(tail_task);
        /*
        let my_conf = self.configuration.clone();
        let my_channel = channel.clone();

        // TODO: This task doesn't seem to be run
        tokio::spawn(async move {
            println!("Querying channel");
            tx.send("test message".to_string());
            let last_message_timestamp = None;
            loop {
                tx.send("test message".to_string());
                // TODO: Query slack
                // Result<::std::collections::HashMap<String, serde_json::Value>, Error<ConversationsHistoryError>>
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
                        tx.send("test message 2".to_string());
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
        // TODO: Do stuff with tx...
        */
        return rx;

    }

    // TODO: Tail #channel - returns a channel
    // TODO: Send message, takes channel, string
}
