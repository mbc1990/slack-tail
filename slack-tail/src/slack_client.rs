use swagger::apis::*;
use swagger::models::*;
use swagger::apis::configuration::Configuration;
use swagger::apis::client::APIClient;
use hyper::Client;
use tokio_core::reactor::Core;

pub struct SlackClient {
}

impl SlackClient  {
    pub fn new() -> SlackClient {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let hyper_client = Client::new(&handle);
        let configuration = Configuration::new(hyper_client);
        let api_client = APIClient::new(configuration);
        // TODO: Set up authentication, make a real request

        SlackClient {}
    }
}
