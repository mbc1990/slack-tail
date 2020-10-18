use openapi::apis::configuration::Configuration;
use openapi::apis::conversations_api;
use openapi::apis::chat_api;
use openapi::apis::auth_api;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration};
use tokio::time::delay_for;
use serde_json::Value;
use std::collections::HashMap;
use tokio::task::JoinHandle;


pub struct SlackClient {
    configuration: Configuration,
    user_id: String,
    url: String,
    xoxs_token: Option<String>
}

pub fn construct_string(strs: &[&str]) -> String {
    let mut ret = String::new();
    for st in strs.iter() {
        ret.push_str(st);
    }
    ret
}

// TODO: Move to other file? Re-integrate with SlackClient to reference self.configuration?
fn tail_channel_to_existing_tx(conf: Configuration, channel_id: String, tx: Sender<Value>) -> JoinHandle<()> {
    let my_conf = conf.clone();
    let my_channel = channel_id.clone();
    let handle = tokio::spawn(async move {
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
                        let mut msg = message.clone();
                        // TODO: MessageWrapper or something...

                        tx.send(message.clone());
                    }
                },
                Err(err) => {
                    println!("Error from slack api {:?}", err);
                }
            }
            delay_for(Duration::from_millis(2000)).await;
        }
    });
    return handle;
}

impl SlackClient  {
    pub async fn new(oauth_access_token: &str) -> SlackClient {
        let mut configuration = Configuration::new();
        configuration.oauth_access_token = Some(oauth_access_token.to_string());
        let my_conf = configuration.clone();

        let resp = auth_api::auth_test(
            &my_conf,
           ""
        ).await;
        let mut user_id = "".to_string();
        let mut url = "".to_string();
        match resp {
            Ok(res) => {
                user_id = res.get("user_id").unwrap().as_str().unwrap().to_string();
                url = res.get("url").unwrap().as_str().unwrap().to_string();
            },
            Err(err) => {
                println!("Error geting bot user id {:?}", err);
            }
        }
        SlackClient {configuration: configuration, user_id: user_id, url: url, xoxs_token: None}
    }

    // TODO: *really* Need to refactor this into message structs that get deserialized from the response
    pub fn is_mention(&self, message: String, user: String) -> bool {
        return user != self.user_id && message.contains(&self.user_id);
    }

    pub async fn send_message(&self, message: &str, channel: &str) {
        /*
            chat_post_message(configuration: &configuration::Configuration,
            token: &str,
            channel: &str,
            attachments: Option<&str>,
            unfurl_links: Option<bool>,
            text: Option<&str>,
            unfurl_media: Option<bool>,
            parse: Option<&str>,
            as_user: Option<&str>,
            mrkdwn: Option<bool>,
            blocks: Option<&str>,
            icon_emoji: Option<&str>,
            link_names: Option<bool>,
            reply_broadcast: Option<bool>,
            thread_ts: Option<&str>,
            username: Option<&str>,
            icon_url: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<ChatPostMessageError>> {
        */
        let my_conf = self.configuration.clone();
        chat_api::chat_post_message(
            &my_conf,
            "",
            channel,
            None,
            None,
            Some(message),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None).await;
    }

    pub fn set_xoxs_token(&mut self, xoxs_token: String) {
        self.xoxs_token = Some(xoxs_token);
    }

    pub fn add_emoji(&self, emoji_name: String, path: String) -> Result<reqwest::Response, reqwest::Error>{
        if self.xoxs_token.is_none() {
            panic!("xoxs_token not set - you cannot use internal Slack APIs without setting the internal token (xoxs-)");
        }
        let url = construct_string(&[&self.url, "api/emoji.add"]);
        let client = reqwest::Client::new();
        let form = reqwest::multipart::Form::new()
            .text("name", emoji_name.clone())
            .text("mode", "data")
            .file("image", path);
        let my_token = self.xoxs_token.as_ref().unwrap().clone();
        return client.post(&url)
            .multipart(form.unwrap())
            .bearer_auth(&my_token)
            .send();
    }

    // Tails all channels the bot belongs to
    pub async fn tail_member_of(&mut self) -> Receiver<Value> {
        let (tx, rx) = mpsc::channel();
        let loops_conf = self.configuration.clone();
        let _task = tokio::spawn(async move {
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

                let current_polled = live_api_pollers.keys();
                current_polled.for_each(|polled_channel_id| {
                    if !bot_in_channels.contains(polled_channel_id) {
                        // Kill task if we're no longer in the channel
                        drop(live_api_pollers.get(polled_channel_id).unwrap());
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
                delay_for(Duration::from_millis(2000)).await;
            }
        });
        return rx;
    }

    pub fn tail_channel(&self, channel: String) -> Receiver<Value> {
        let (tx, rx) = mpsc::channel();
        let my_conf = self.configuration.clone();
        let my_channel = channel.clone();
        tokio::spawn(async move {
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
                delay_for(Duration::from_millis(2000)).await;
            }
        });
        return rx;
    }
}
