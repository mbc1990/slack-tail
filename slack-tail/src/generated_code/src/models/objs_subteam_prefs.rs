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
pub struct ObjsSubteamPrefs {
  #[serde(rename = "channels")]
  channels: Vec<::models::DefsChannelId>,
  #[serde(rename = "groups")]
  groups: Vec<::models::DefsGroupId>
}

impl ObjsSubteamPrefs {
  pub fn new(channels: Vec<::models::DefsChannelId>, groups: Vec<::models::DefsGroupId>) -> ObjsSubteamPrefs {
    ObjsSubteamPrefs {
      channels: channels,
      groups: groups
    }
  }

  pub fn set_channels(&mut self, channels: Vec<::models::DefsChannelId>) {
    self.channels = channels;
  }

  pub fn with_channels(mut self, channels: Vec<::models::DefsChannelId>) -> ObjsSubteamPrefs {
    self.channels = channels;
    self
  }

  pub fn channels(&self) -> &Vec<::models::DefsChannelId> {
    &self.channels
  }


  pub fn set_groups(&mut self, groups: Vec<::models::DefsGroupId>) {
    self.groups = groups;
  }

  pub fn with_groups(mut self, groups: Vec<::models::DefsGroupId>) -> ObjsSubteamPrefs {
    self.groups = groups;
    self
  }

  pub fn groups(&self) -> &Vec<::models::DefsGroupId> {
    &self.groups
  }


}


