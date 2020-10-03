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
pub struct ObjsChannel {
  #[serde(rename = "accepted_user")]
  accepted_user: Option<::models::DefsUserId>,
  #[serde(rename = "created")]
  created: i32,
  #[serde(rename = "creator")]
  creator: ::models::DefsUserId,
  #[serde(rename = "id")]
  id: ::models::DefsChannelId,
  #[serde(rename = "is_archived")]
  is_archived: Option<bool>,
  #[serde(rename = "is_channel")]
  is_channel: bool,
  #[serde(rename = "is_frozen")]
  is_frozen: Option<bool>,
  #[serde(rename = "is_general")]
  is_general: Option<bool>,
  #[serde(rename = "is_member")]
  is_member: Option<bool>,
  #[serde(rename = "is_moved")]
  is_moved: Option<i32>,
  #[serde(rename = "is_mpim")]
  is_mpim: bool,
  #[serde(rename = "is_non_threadable")]
  is_non_threadable: Option<bool>,
  #[serde(rename = "is_org_shared")]
  is_org_shared: bool,
  #[serde(rename = "is_pending_ext_shared")]
  is_pending_ext_shared: Option<bool>,
  #[serde(rename = "is_private")]
  is_private: bool,
  #[serde(rename = "is_read_only")]
  is_read_only: Option<bool>,
  #[serde(rename = "is_shared")]
  is_shared: bool,
  #[serde(rename = "is_thread_only")]
  is_thread_only: Option<bool>,
  #[serde(rename = "last_read")]
  last_read: Option<::models::DefsTs>,
  #[serde(rename = "latest")]
  latest: Option<Value>,
  #[serde(rename = "members")]
  members: Vec<::models::DefsUserId>,
  #[serde(rename = "name")]
  name: String,
  #[serde(rename = "name_normalized")]
  name_normalized: String,
  #[serde(rename = "num_members")]
  num_members: Option<i32>,
  #[serde(rename = "pending_shared")]
  pending_shared: Option<Vec<::models::DefsTeam>>,
  #[serde(rename = "previous_names")]
  previous_names: Option<Vec<::models::DefsChannelName>>,
  #[serde(rename = "priority")]
  priority: Option<f32>,
  #[serde(rename = "purpose")]
  purpose: ::models::ObjsChannelPurpose,
  #[serde(rename = "topic")]
  topic: ::models::ObjsChannelPurpose,
  #[serde(rename = "unlinked")]
  unlinked: Option<i32>,
  #[serde(rename = "unread_count")]
  unread_count: Option<i32>,
  #[serde(rename = "unread_count_display")]
  unread_count_display: Option<i32>
}

impl ObjsChannel {
  pub fn new(created: i32, creator: ::models::DefsUserId, id: ::models::DefsChannelId, is_channel: bool, is_mpim: bool, is_org_shared: bool, is_private: bool, is_shared: bool, members: Vec<::models::DefsUserId>, name: String, name_normalized: String, purpose: ::models::ObjsChannelPurpose, topic: ::models::ObjsChannelPurpose) -> ObjsChannel {
    ObjsChannel {
      accepted_user: None,
      created: created,
      creator: creator,
      id: id,
      is_archived: None,
      is_channel: is_channel,
      is_frozen: None,
      is_general: None,
      is_member: None,
      is_moved: None,
      is_mpim: is_mpim,
      is_non_threadable: None,
      is_org_shared: is_org_shared,
      is_pending_ext_shared: None,
      is_private: is_private,
      is_read_only: None,
      is_shared: is_shared,
      is_thread_only: None,
      last_read: None,
      latest: None,
      members: members,
      name: name,
      name_normalized: name_normalized,
      num_members: None,
      pending_shared: None,
      previous_names: None,
      priority: None,
      purpose: purpose,
      topic: topic,
      unlinked: None,
      unread_count: None,
      unread_count_display: None
    }
  }

  pub fn set_accepted_user(&mut self, accepted_user: ::models::DefsUserId) {
    self.accepted_user = Some(accepted_user);
  }

  pub fn with_accepted_user(mut self, accepted_user: ::models::DefsUserId) -> ObjsChannel {
    self.accepted_user = Some(accepted_user);
    self
  }

  pub fn accepted_user(&self) -> Option<&::models::DefsUserId> {
    self.accepted_user.as_ref()
  }

  pub fn reset_accepted_user(&mut self) {
    self.accepted_user = None;
  }

  pub fn set_created(&mut self, created: i32) {
    self.created = created;
  }

  pub fn with_created(mut self, created: i32) -> ObjsChannel {
    self.created = created;
    self
  }

  pub fn created(&self) -> &i32 {
    &self.created
  }


  pub fn set_creator(&mut self, creator: ::models::DefsUserId) {
    self.creator = creator;
  }

  pub fn with_creator(mut self, creator: ::models::DefsUserId) -> ObjsChannel {
    self.creator = creator;
    self
  }

  pub fn creator(&self) -> &::models::DefsUserId {
    &self.creator
  }


  pub fn set_id(&mut self, id: ::models::DefsChannelId) {
    self.id = id;
  }

  pub fn with_id(mut self, id: ::models::DefsChannelId) -> ObjsChannel {
    self.id = id;
    self
  }

  pub fn id(&self) -> &::models::DefsChannelId {
    &self.id
  }


  pub fn set_is_archived(&mut self, is_archived: bool) {
    self.is_archived = Some(is_archived);
  }

  pub fn with_is_archived(mut self, is_archived: bool) -> ObjsChannel {
    self.is_archived = Some(is_archived);
    self
  }

  pub fn is_archived(&self) -> Option<&bool> {
    self.is_archived.as_ref()
  }

  pub fn reset_is_archived(&mut self) {
    self.is_archived = None;
  }

  pub fn set_is_channel(&mut self, is_channel: bool) {
    self.is_channel = is_channel;
  }

  pub fn with_is_channel(mut self, is_channel: bool) -> ObjsChannel {
    self.is_channel = is_channel;
    self
  }

  pub fn is_channel(&self) -> &bool {
    &self.is_channel
  }


  pub fn set_is_frozen(&mut self, is_frozen: bool) {
    self.is_frozen = Some(is_frozen);
  }

  pub fn with_is_frozen(mut self, is_frozen: bool) -> ObjsChannel {
    self.is_frozen = Some(is_frozen);
    self
  }

  pub fn is_frozen(&self) -> Option<&bool> {
    self.is_frozen.as_ref()
  }

  pub fn reset_is_frozen(&mut self) {
    self.is_frozen = None;
  }

  pub fn set_is_general(&mut self, is_general: bool) {
    self.is_general = Some(is_general);
  }

  pub fn with_is_general(mut self, is_general: bool) -> ObjsChannel {
    self.is_general = Some(is_general);
    self
  }

  pub fn is_general(&self) -> Option<&bool> {
    self.is_general.as_ref()
  }

  pub fn reset_is_general(&mut self) {
    self.is_general = None;
  }

  pub fn set_is_member(&mut self, is_member: bool) {
    self.is_member = Some(is_member);
  }

  pub fn with_is_member(mut self, is_member: bool) -> ObjsChannel {
    self.is_member = Some(is_member);
    self
  }

  pub fn is_member(&self) -> Option<&bool> {
    self.is_member.as_ref()
  }

  pub fn reset_is_member(&mut self) {
    self.is_member = None;
  }

  pub fn set_is_moved(&mut self, is_moved: i32) {
    self.is_moved = Some(is_moved);
  }

  pub fn with_is_moved(mut self, is_moved: i32) -> ObjsChannel {
    self.is_moved = Some(is_moved);
    self
  }

  pub fn is_moved(&self) -> Option<&i32> {
    self.is_moved.as_ref()
  }

  pub fn reset_is_moved(&mut self) {
    self.is_moved = None;
  }

  pub fn set_is_mpim(&mut self, is_mpim: bool) {
    self.is_mpim = is_mpim;
  }

  pub fn with_is_mpim(mut self, is_mpim: bool) -> ObjsChannel {
    self.is_mpim = is_mpim;
    self
  }

  pub fn is_mpim(&self) -> &bool {
    &self.is_mpim
  }


  pub fn set_is_non_threadable(&mut self, is_non_threadable: bool) {
    self.is_non_threadable = Some(is_non_threadable);
  }

  pub fn with_is_non_threadable(mut self, is_non_threadable: bool) -> ObjsChannel {
    self.is_non_threadable = Some(is_non_threadable);
    self
  }

  pub fn is_non_threadable(&self) -> Option<&bool> {
    self.is_non_threadable.as_ref()
  }

  pub fn reset_is_non_threadable(&mut self) {
    self.is_non_threadable = None;
  }

  pub fn set_is_org_shared(&mut self, is_org_shared: bool) {
    self.is_org_shared = is_org_shared;
  }

  pub fn with_is_org_shared(mut self, is_org_shared: bool) -> ObjsChannel {
    self.is_org_shared = is_org_shared;
    self
  }

  pub fn is_org_shared(&self) -> &bool {
    &self.is_org_shared
  }


  pub fn set_is_pending_ext_shared(&mut self, is_pending_ext_shared: bool) {
    self.is_pending_ext_shared = Some(is_pending_ext_shared);
  }

  pub fn with_is_pending_ext_shared(mut self, is_pending_ext_shared: bool) -> ObjsChannel {
    self.is_pending_ext_shared = Some(is_pending_ext_shared);
    self
  }

  pub fn is_pending_ext_shared(&self) -> Option<&bool> {
    self.is_pending_ext_shared.as_ref()
  }

  pub fn reset_is_pending_ext_shared(&mut self) {
    self.is_pending_ext_shared = None;
  }

  pub fn set_is_private(&mut self, is_private: bool) {
    self.is_private = is_private;
  }

  pub fn with_is_private(mut self, is_private: bool) -> ObjsChannel {
    self.is_private = is_private;
    self
  }

  pub fn is_private(&self) -> &bool {
    &self.is_private
  }


  pub fn set_is_read_only(&mut self, is_read_only: bool) {
    self.is_read_only = Some(is_read_only);
  }

  pub fn with_is_read_only(mut self, is_read_only: bool) -> ObjsChannel {
    self.is_read_only = Some(is_read_only);
    self
  }

  pub fn is_read_only(&self) -> Option<&bool> {
    self.is_read_only.as_ref()
  }

  pub fn reset_is_read_only(&mut self) {
    self.is_read_only = None;
  }

  pub fn set_is_shared(&mut self, is_shared: bool) {
    self.is_shared = is_shared;
  }

  pub fn with_is_shared(mut self, is_shared: bool) -> ObjsChannel {
    self.is_shared = is_shared;
    self
  }

  pub fn is_shared(&self) -> &bool {
    &self.is_shared
  }


  pub fn set_is_thread_only(&mut self, is_thread_only: bool) {
    self.is_thread_only = Some(is_thread_only);
  }

  pub fn with_is_thread_only(mut self, is_thread_only: bool) -> ObjsChannel {
    self.is_thread_only = Some(is_thread_only);
    self
  }

  pub fn is_thread_only(&self) -> Option<&bool> {
    self.is_thread_only.as_ref()
  }

  pub fn reset_is_thread_only(&mut self) {
    self.is_thread_only = None;
  }

  pub fn set_last_read(&mut self, last_read: ::models::DefsTs) {
    self.last_read = Some(last_read);
  }

  pub fn with_last_read(mut self, last_read: ::models::DefsTs) -> ObjsChannel {
    self.last_read = Some(last_read);
    self
  }

  pub fn last_read(&self) -> Option<&::models::DefsTs> {
    self.last_read.as_ref()
  }

  pub fn reset_last_read(&mut self) {
    self.last_read = None;
  }

  pub fn set_latest(&mut self, latest: Value) {
    self.latest = Some(latest);
  }

  pub fn with_latest(mut self, latest: Value) -> ObjsChannel {
    self.latest = Some(latest);
    self
  }

  pub fn latest(&self) -> Option<&Value> {
    self.latest.as_ref()
  }

  pub fn reset_latest(&mut self) {
    self.latest = None;
  }

  pub fn set_members(&mut self, members: Vec<::models::DefsUserId>) {
    self.members = members;
  }

  pub fn with_members(mut self, members: Vec<::models::DefsUserId>) -> ObjsChannel {
    self.members = members;
    self
  }

  pub fn members(&self) -> &Vec<::models::DefsUserId> {
    &self.members
  }


  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> ObjsChannel {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_name_normalized(&mut self, name_normalized: String) {
    self.name_normalized = name_normalized;
  }

  pub fn with_name_normalized(mut self, name_normalized: String) -> ObjsChannel {
    self.name_normalized = name_normalized;
    self
  }

  pub fn name_normalized(&self) -> &String {
    &self.name_normalized
  }


  pub fn set_num_members(&mut self, num_members: i32) {
    self.num_members = Some(num_members);
  }

  pub fn with_num_members(mut self, num_members: i32) -> ObjsChannel {
    self.num_members = Some(num_members);
    self
  }

  pub fn num_members(&self) -> Option<&i32> {
    self.num_members.as_ref()
  }

  pub fn reset_num_members(&mut self) {
    self.num_members = None;
  }

  pub fn set_pending_shared(&mut self, pending_shared: Vec<::models::DefsTeam>) {
    self.pending_shared = Some(pending_shared);
  }

  pub fn with_pending_shared(mut self, pending_shared: Vec<::models::DefsTeam>) -> ObjsChannel {
    self.pending_shared = Some(pending_shared);
    self
  }

  pub fn pending_shared(&self) -> Option<&Vec<::models::DefsTeam>> {
    self.pending_shared.as_ref()
  }

  pub fn reset_pending_shared(&mut self) {
    self.pending_shared = None;
  }

  pub fn set_previous_names(&mut self, previous_names: Vec<::models::DefsChannelName>) {
    self.previous_names = Some(previous_names);
  }

  pub fn with_previous_names(mut self, previous_names: Vec<::models::DefsChannelName>) -> ObjsChannel {
    self.previous_names = Some(previous_names);
    self
  }

  pub fn previous_names(&self) -> Option<&Vec<::models::DefsChannelName>> {
    self.previous_names.as_ref()
  }

  pub fn reset_previous_names(&mut self) {
    self.previous_names = None;
  }

  pub fn set_priority(&mut self, priority: f32) {
    self.priority = Some(priority);
  }

  pub fn with_priority(mut self, priority: f32) -> ObjsChannel {
    self.priority = Some(priority);
    self
  }

  pub fn priority(&self) -> Option<&f32> {
    self.priority.as_ref()
  }

  pub fn reset_priority(&mut self) {
    self.priority = None;
  }

  pub fn set_purpose(&mut self, purpose: ::models::ObjsChannelPurpose) {
    self.purpose = purpose;
  }

  pub fn with_purpose(mut self, purpose: ::models::ObjsChannelPurpose) -> ObjsChannel {
    self.purpose = purpose;
    self
  }

  pub fn purpose(&self) -> &::models::ObjsChannelPurpose {
    &self.purpose
  }


  pub fn set_topic(&mut self, topic: ::models::ObjsChannelPurpose) {
    self.topic = topic;
  }

  pub fn with_topic(mut self, topic: ::models::ObjsChannelPurpose) -> ObjsChannel {
    self.topic = topic;
    self
  }

  pub fn topic(&self) -> &::models::ObjsChannelPurpose {
    &self.topic
  }


  pub fn set_unlinked(&mut self, unlinked: i32) {
    self.unlinked = Some(unlinked);
  }

  pub fn with_unlinked(mut self, unlinked: i32) -> ObjsChannel {
    self.unlinked = Some(unlinked);
    self
  }

  pub fn unlinked(&self) -> Option<&i32> {
    self.unlinked.as_ref()
  }

  pub fn reset_unlinked(&mut self) {
    self.unlinked = None;
  }

  pub fn set_unread_count(&mut self, unread_count: i32) {
    self.unread_count = Some(unread_count);
  }

  pub fn with_unread_count(mut self, unread_count: i32) -> ObjsChannel {
    self.unread_count = Some(unread_count);
    self
  }

  pub fn unread_count(&self) -> Option<&i32> {
    self.unread_count.as_ref()
  }

  pub fn reset_unread_count(&mut self) {
    self.unread_count = None;
  }

  pub fn set_unread_count_display(&mut self, unread_count_display: i32) {
    self.unread_count_display = Some(unread_count_display);
  }

  pub fn with_unread_count_display(mut self, unread_count_display: i32) -> ObjsChannel {
    self.unread_count_display = Some(unread_count_display);
    self
  }

  pub fn unread_count_display(&self) -> Option<&i32> {
    self.unread_count_display.as_ref()
  }

  pub fn reset_unread_count_display(&mut self) {
    self.unread_count_display = None;
  }

}



