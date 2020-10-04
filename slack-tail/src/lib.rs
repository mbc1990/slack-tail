extern crate openapi;

pub mod slack_client {
    pub use openapi::apis::*;
    pub use openapi::models::*;
}

#[cfg(test)]
mod tests {
    use crate::slack_client::SlackClient;

    #[test]
    fn it_works() {
        let oauth_access_token = "xoxb-472934603414-1418425791745-VmKqXWVQQLg1O0G7qdCwUuc1";
        let client = SlackClient::new(oauth_access_token);
        assert_eq!(2 + 2, 4);
    }
}
