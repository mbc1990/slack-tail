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
pub struct ObjsComment {
  #[serde(rename = "comment")]
  comment: String,
  #[serde(rename = "created")]
  created: i32,
  #[serde(rename = "id")]
  id: ::models::DefsCommentId,
  #[serde(rename = "is_intro")]
  is_intro: bool,
  #[serde(rename = "is_starred")]
  is_starred: Option<bool>,
  #[serde(rename = "num_stars")]
  num_stars: Option<i32>,
  #[serde(rename = "pinned_info")]
  pinned_info: Option<::models::DefsPinnedInfo>,
  #[serde(rename = "pinned_to")]
  pinned_to: Option<Vec<::models::DefsChannel>>,
  #[serde(rename = "reactions")]
  reactions: Option<Vec<::models::ObjsReaction>>,
  #[serde(rename = "timestamp")]
  timestamp: i32,
  #[serde(rename = "user")]
  user: ::models::DefsUserId
}

impl ObjsComment {
  pub fn new(comment: String, created: i32, id: ::models::DefsCommentId, is_intro: bool, timestamp: i32, user: ::models::DefsUserId) -> ObjsComment {
    ObjsComment {
      comment: comment,
      created: created,
      id: id,
      is_intro: is_intro,
      is_starred: None,
      num_stars: None,
      pinned_info: None,
      pinned_to: None,
      reactions: None,
      timestamp: timestamp,
      user: user
    }
  }

  pub fn set_comment(&mut self, comment: String) {
    self.comment = comment;
  }

  pub fn with_comment(mut self, comment: String) -> ObjsComment {
    self.comment = comment;
    self
  }

  pub fn comment(&self) -> &String {
    &self.comment
  }


  pub fn set_created(&mut self, created: i32) {
    self.created = created;
  }

  pub fn with_created(mut self, created: i32) -> ObjsComment {
    self.created = created;
    self
  }

  pub fn created(&self) -> &i32 {
    &self.created
  }


  pub fn set_id(&mut self, id: ::models::DefsCommentId) {
    self.id = id;
  }

  pub fn with_id(mut self, id: ::models::DefsCommentId) -> ObjsComment {
    self.id = id;
    self
  }

  pub fn id(&self) -> &::models::DefsCommentId {
    &self.id
  }


  pub fn set_is_intro(&mut self, is_intro: bool) {
    self.is_intro = is_intro;
  }

  pub fn with_is_intro(mut self, is_intro: bool) -> ObjsComment {
    self.is_intro = is_intro;
    self
  }

  pub fn is_intro(&self) -> &bool {
    &self.is_intro
  }


  pub fn set_is_starred(&mut self, is_starred: bool) {
    self.is_starred = Some(is_starred);
  }

  pub fn with_is_starred(mut self, is_starred: bool) -> ObjsComment {
    self.is_starred = Some(is_starred);
    self
  }

  pub fn is_starred(&self) -> Option<&bool> {
    self.is_starred.as_ref()
  }

  pub fn reset_is_starred(&mut self) {
    self.is_starred = None;
  }

  pub fn set_num_stars(&mut self, num_stars: i32) {
    self.num_stars = Some(num_stars);
  }

  pub fn with_num_stars(mut self, num_stars: i32) -> ObjsComment {
    self.num_stars = Some(num_stars);
    self
  }

  pub fn num_stars(&self) -> Option<&i32> {
    self.num_stars.as_ref()
  }

  pub fn reset_num_stars(&mut self) {
    self.num_stars = None;
  }

  pub fn set_pinned_info(&mut self, pinned_info: ::models::DefsPinnedInfo) {
    self.pinned_info = Some(pinned_info);
  }

  pub fn with_pinned_info(mut self, pinned_info: ::models::DefsPinnedInfo) -> ObjsComment {
    self.pinned_info = Some(pinned_info);
    self
  }

  pub fn pinned_info(&self) -> Option<&::models::DefsPinnedInfo> {
    self.pinned_info.as_ref()
  }

  pub fn reset_pinned_info(&mut self) {
    self.pinned_info = None;
  }

  pub fn set_pinned_to(&mut self, pinned_to: Vec<::models::DefsChannel>) {
    self.pinned_to = Some(pinned_to);
  }

  pub fn with_pinned_to(mut self, pinned_to: Vec<::models::DefsChannel>) -> ObjsComment {
    self.pinned_to = Some(pinned_to);
    self
  }

  pub fn pinned_to(&self) -> Option<&Vec<::models::DefsChannel>> {
    self.pinned_to.as_ref()
  }

  pub fn reset_pinned_to(&mut self) {
    self.pinned_to = None;
  }

  pub fn set_reactions(&mut self, reactions: Vec<::models::ObjsReaction>) {
    self.reactions = Some(reactions);
  }

  pub fn with_reactions(mut self, reactions: Vec<::models::ObjsReaction>) -> ObjsComment {
    self.reactions = Some(reactions);
    self
  }

  pub fn reactions(&self) -> Option<&Vec<::models::ObjsReaction>> {
    self.reactions.as_ref()
  }

  pub fn reset_reactions(&mut self) {
    self.reactions = None;
  }

  pub fn set_timestamp(&mut self, timestamp: i32) {
    self.timestamp = timestamp;
  }

  pub fn with_timestamp(mut self, timestamp: i32) -> ObjsComment {
    self.timestamp = timestamp;
    self
  }

  pub fn timestamp(&self) -> &i32 {
    &self.timestamp
  }


  pub fn set_user(&mut self, user: ::models::DefsUserId) {
    self.user = user;
  }

  pub fn with_user(mut self, user: ::models::DefsUserId) -> ObjsComment {
    self.user = user;
    self
  }

  pub fn user(&self) -> &::models::DefsUserId {
    &self.user
  }


}



