use openapi::apis::configuration::Configuration;
use openapi::apis::oauth_api::oauth_access;
use openapi::apis::channels_api;
use openapi::models::*;
use tokio::runtime::Runtime;



pub struct SlackClient {
    configuration: Configuration
}

impl SlackClient  {
    pub fn new(oauth_access_token: &str) -> SlackClient {
        let mut runtime = Runtime::new().expect("Failed to create Tokio runtime");

        let mut configuration = Configuration::new();
        configuration.oauth_access_token = Some(oauth_access_token.to_string());

        let res_future = channels_api::channels_list(
            &configuration,
            Some(false),
            None,
            None,
            None,
           Some(true)
        );

        let res = runtime.block_on(res_future);
        match res {
            Ok(payload) => {
                println!("good: {:?}", payload) ;
            },
            Err(err) => {
                println!("Error: {:?}", err) ;
            }
        }
        SlackClient {configuration}
    }
}
