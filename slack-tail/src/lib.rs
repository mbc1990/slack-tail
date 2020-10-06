extern crate openapi;

mod slack_client;

pub mod raw_api {
    pub use openapi::apis::*;
    pub use openapi::models::*;
}

#[cfg(test)]
mod tests {
    use crate::slack_client::SlackClient;

    #[test]
    fn it_works() {

        let client = SlackClient::new(oauth_access_token);
        assert_eq!(2 + 2, 4);
    }
}
