/* 
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * OpenAPI spec version: 1.5.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjsMessageReplies {
  #[serde(rename = "ts")]
  ts: ::models::DefsTs,
  #[serde(rename = "user")]
  user: ::models::DefsUserId
}

impl ObjsMessageReplies {
  pub fn new(ts: ::models::DefsTs, user: ::models::DefsUserId) -> ObjsMessageReplies {
    ObjsMessageReplies {
      ts: ts,
      user: user
    }
  }

  pub fn set_ts(&mut self, ts: ::models::DefsTs) {
    self.ts = ts;
  }

  pub fn with_ts(mut self, ts: ::models::DefsTs) -> ObjsMessageReplies {
    self.ts = ts;
    self
  }

  pub fn ts(&self) -> &::models::DefsTs {
    &self.ts
  }


  pub fn set_user(&mut self, user: ::models::DefsUserId) {
    self.user = user;
  }

  pub fn with_user(mut self, user: ::models::DefsUserId) -> ObjsMessageReplies {
    self.user = user;
    self
  }

  pub fn user(&self) -> &::models::DefsUserId {
    &self.user
  }


}


