#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate url;
extern crate swagger;

// pub mod apis;
// pub mod models;
mod slack_client;
// mod codegen;

#[cfg(test)]
mod tests {
    use crate::slack_client::SlackClient;
    // use crate::codegen::apis::ApiApiClient;

    #[test]
    fn it_works() {
        let client = SlackClient::new();
        assert_eq!(2 + 2, 4);
    }
}
