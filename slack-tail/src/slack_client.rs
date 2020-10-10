use openapi::apis::configuration::Configuration;
use openapi::apis::oauth_api::oauth_access;
use openapi::apis::channels_api;
use openapi::apis::conversations_api;
use openapi::models::*;
use tokio::runtime::{Runtime, Builder};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, RecvError};
use tokio::prelude::*;
use tokio::time::delay_for;
use std::time::{Duration, Instant};
use std::thread;
use crate::tail_task::TailTask;
use reqwest::{Client};


pub struct SlackClient {
    configuration: Configuration,
    task_tx: Sender<TailTask>,
    runtime: Runtime
}

fn runtime_entry(rx: Receiver<TailTask>) {
    /*
    tokio::spawn(async move {
        println!("In the debug tokio task!");
    });
    loop {
        let task_result: Result<TailTask, RecvError> = rx.recv();
        match task_result {
            Ok(task) => {
                println!("Task received {:?}", task);
                task.output_tx.send("Starting tailer...".to_string());
                tokio::spawn(async move {
                    println!("In tokio task");
                    task.output_tx.send("Message from inside a tailer!".to_string());
                });
            },
            Err(e) => {
                println!("Error receiving {:?}", e.to_string());
            }
        }
    }
    */
}

async fn tail_enter(my_conf: Configuration, my_channel: String, tx: Sender<String>) {
    println!("Querying channel");
    tx.send("test message 1".to_string());
    let last_message_timestamp = None;
    loop {
        tx.send("test message 2".to_string());
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
}

    impl SlackClient  {
        pub fn new(oauth_access_token: &str) -> SlackClient {
            let mut runtime = Builder::new()
                .threaded_scheduler()
                .build()
                .unwrap();
            let mut configuration = Configuration::new();
            configuration.oauth_access_token = Some(oauth_access_token.to_string());
            let handle = runtime.handle().clone();
            let (mut tx, mut rx): (std::sync::mpsc::Sender<TailTask>, std::sync::mpsc::Receiver<TailTask>) = mpsc::channel();
            /*
            let mut my_conf = configuration.clone();
            let client = Client::new();
            my_conf.client = client;
            thread::spawn(move ||{
                handle.block_on(async move {
                    println!("hello from block_on");

                    // Worked in client test
                    // tokio::spawn(async { println!("hello from handle"); });

                    loop {
                        match rx.recv() {
                            Ok(data) => {
                                println!("(Tailer runtime): Data received {:?}", data);
                                let stupid_hacky_copy_conf = my_conf.clone();
                                tokio::spawn(async move {
                                    println!("(Tailer http task): Making history query");
                                    let last_message_timestamp = None;
                                    let history_result =
                                        conversations_api::conversations_history(
                                        &stupid_hacky_copy_conf,
                                        Some(false),
                                        None,
                                        None,
                                        Some(100),
                                        last_message_timestamp,
                                        Some(&data.slack_channel_id),
                                        None
                                    ).await;
                                    println!("(Tailer http task): History query complete");

                                    match history_result {
                                        Ok(resp) => {
                                            println!("(Tailer http task): Got a good response from slack API");
                                            data.output_tx.send("This would be the history results".to_string());
                                            // TODO: Iterate and write to channel
                                        },
                                        Err(err) => {
                                            println!("Error from slack api {:?}", err);
                                        }
                                    }
                                });
                            },
                            Err(err) => {
                                println!("(Tailer runtime) Error receiving {:?}", err);
                            }
                        }
                    }
                });
            });
            */
            SlackClient {configuration: configuration, task_tx: tx, runtime: runtime}
        }

    pub fn tail_channel(&self, channel: String) -> Receiver<String> {
        println!("Beginning tail channel");
        let (mut tx, mut rx) = mpsc::channel();
        // let tail_task = TailTask{slack_channel_id: channel, output_tx: tx};
        // self.task_tx.send(tail_task);
        let my_conf = self.configuration.clone();
        let my_channel = channel.clone();

        // let my_rt_handle = self.runtime.handle();
        // println!("About to enter rt");
        // let tail_future = my_rt_handle.enter(|| tail_enter(my_conf, my_channel, tx));

        println!("After entering rt");
        // TODO: See if starting this with 'enter' will let it spawn tasks in the generated code
        tokio::spawn(async move {
            println!("Querying channel");
            tx.send("test message 1".to_string());
            let last_message_timestamp = None;
            loop {
                tx.send("test message 2".to_string());
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
        // TODO: Do stuff with tx...
        return rx;

    }

    // TODO: Tail #channel - returns a channel
    // TODO: Send message, takes channel, string
}
