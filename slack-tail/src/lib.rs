extern crate openapi;

mod slack_client;
mod slack_message;
mod slack_message_send_task;

pub mod raw_api {
    pub use openapi::apis::*;
    pub use openapi::models::*;
}

pub mod streaming {
    pub use crate::slack_client::SlackClient;
    pub use crate::slack_message::SlackMessage;
    pub use crate::slack_message_send_task::SlackMessageSendTask;
}


#[cfg(test)]
mod tests {
    use crate::slack_client::SlackClient;

    #[test]
    fn it_works() {
    }
}
