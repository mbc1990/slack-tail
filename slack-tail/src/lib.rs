extern crate swagger;

// pub mod apis;
// pub mod models;
mod slack_client;
// mod codegen;

use swagger::apis::*;
use swagger::models::*;

#[cfg(test)]
mod tests {
    use crate::slack_client::SlackClient;

    #[test]
    fn it_works() {
        let client = SlackClient::new();
        assert_eq!(2 + 2, 4);
    }
}
