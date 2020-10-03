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
pub struct ObjsResponseMetadata {
  #[serde(rename = "next_cursor")]
  next_cursor: String
}

impl ObjsResponseMetadata {
  pub fn new(next_cursor: String) -> ObjsResponseMetadata {
    ObjsResponseMetadata {
      next_cursor: next_cursor
    }
  }

  pub fn set_next_cursor(&mut self, next_cursor: String) {
    self.next_cursor = next_cursor;
  }

  pub fn with_next_cursor(mut self, next_cursor: String) -> ObjsResponseMetadata {
    self.next_cursor = next_cursor;
    self
  }

  pub fn next_cursor(&self) -> &String {
    &self.next_cursor
  }


}


