extern crate openapi;

mod slack_client;
mod tail_task;

pub mod raw_api {
    pub use openapi::apis::*;
    pub use openapi::models::*;
}

pub mod streaming {
    pub use crate::slack_client::SlackClient;
}


#[cfg(test)]
mod tests {
    use crate::slack_client::SlackClient;
    use futures::join;

    #[test]
    fn it_works() {
        let oauth_access_token = "";
        let client = SlackClient::new(oauth_access_token);
        println!("About to tail channel");
        let rx = client.tail_channel("CDWH7Q5FH".to_string());
        println!("got a rx");

        match rx.recv() {
            Ok(data) => {
                println!("Data received {:?}", data);
            },
            Err(err) => {
                println!("Error receiving {:?}", err);
            }
        }
        assert_eq!(2 + 2, 4);
    }
}
