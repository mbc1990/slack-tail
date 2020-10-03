/* 
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * OpenAPI spec version: 1.5.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;

use hyper;
use serde_json::*;
use futures;
use futures::{Future, Stream};

use hyper::header::UserAgent;

use super::{Error, configuration};

pub struct AdminConversationsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> AdminConversationsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AdminConversationsApiClient<C> {
        AdminConversationsApiClient {
            configuration: configuration,
        }
    }
}

pub trait AdminConversationsApi {
    fn admin_conversations_set_teams(&self, channel_id: &str, token: &str, org_channel: bool, team_id: &str, target_team_ids: &str) -> Box<Future<Item = ::std::collections::HashMap<String, Value>, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>AdminConversationsApi for AdminConversationsApiClient<C> {
    fn admin_conversations_set_teams(&self, channel_id: &str, token: &str, org_channel: bool, team_id: &str, target_team_ids: &str) -> Box<Future<Item = ::std::collections::HashMap<String, Value>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref token) = configuration.oauth_access_token {
            let auth = hyper::header::Authorization(
                hyper::header::Bearer {
                    token: token.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/admin.conversations.setTeams?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("token", token);
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::std::collections::HashMap<String, Value>> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}
