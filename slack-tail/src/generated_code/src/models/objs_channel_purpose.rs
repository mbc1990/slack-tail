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
pub struct ObjsChannelPurpose {
  #[serde(rename = "creator")]
  creator: ::models::DefsTopicPurposeCreator,
  #[serde(rename = "last_set")]
  last_set: i32,
  #[serde(rename = "value")]
  value: String
}

impl ObjsChannelPurpose {
  pub fn new(creator: ::models::DefsTopicPurposeCreator, last_set: i32, value: String) -> ObjsChannelPurpose {
    ObjsChannelPurpose {
      creator: creator,
      last_set: last_set,
      value: value
    }
  }

  pub fn set_creator(&mut self, creator: ::models::DefsTopicPurposeCreator) {
    self.creator = creator;
  }

  pub fn with_creator(mut self, creator: ::models::DefsTopicPurposeCreator) -> ObjsChannelPurpose {
    self.creator = creator;
    self
  }

  pub fn creator(&self) -> &::models::DefsTopicPurposeCreator {
    &self.creator
  }


  pub fn set_last_set(&mut self, last_set: i32) {
    self.last_set = last_set;
  }

  pub fn with_last_set(mut self, last_set: i32) -> ObjsChannelPurpose {
    self.last_set = last_set;
    self
  }

  pub fn last_set(&self) -> &i32 {
    &self.last_set
  }


  pub fn set_value(&mut self, value: String) {
    self.value = value;
  }

  pub fn with_value(mut self, value: String) -> ObjsChannelPurpose {
    self.value = value;
    self
  }

  pub fn value(&self) -> &String {
    &self.value
  }


}



