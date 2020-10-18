extern crate openapi;

mod slack_client;
mod slack_message;

pub mod raw_api {
    pub use openapi::apis::*;
    pub use openapi::models::*;
}

pub mod streaming {
    pub use crate::slack_client::SlackClient;
    pub use crate::slack_message::SlackMessage;
}


#[cfg(test)]
mod tests {
    use crate::slack_client::SlackClient;

    #[test]
    fn it_works() {
    }
}
