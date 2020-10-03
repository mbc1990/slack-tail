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
pub struct ObjsUserProfile {
  #[serde(rename = "always_active")]
  always_active: Option<bool>,
  #[serde(rename = "api_app_id")]
  api_app_id: Option<::models::DefsAppId>,
  #[serde(rename = "avatar_hash")]
  avatar_hash: String,
  #[serde(rename = "bot_id")]
  bot_id: Option<::models::DefsBotId>,
  #[serde(rename = "display_name")]
  display_name: String,
  #[serde(rename = "display_name_normalized")]
  display_name_normalized: String,
  #[serde(rename = "email")]
  email: Option<String>,
  #[serde(rename = "first_name")]
  first_name: Option<String>,
  #[serde(rename = "guest_expiration_ts")]
  guest_expiration_ts: Option<i32>,
  #[serde(rename = "guest_invited_by")]
  guest_invited_by: Option<String>,
  #[serde(rename = "image_1024")]
  image_1024: Option<String>,
  #[serde(rename = "image_192")]
  image_192: Option<String>,
  #[serde(rename = "image_24")]
  image_24: Option<String>,
  #[serde(rename = "image_32")]
  image_32: Option<String>,
  #[serde(rename = "image_48")]
  image_48: Option<String>,
  #[serde(rename = "image_512")]
  image_512: Option<String>,
  #[serde(rename = "image_72")]
  image_72: Option<String>,
  #[serde(rename = "image_original")]
  image_original: Option<String>,
  #[serde(rename = "is_custom_image")]
  is_custom_image: Option<bool>,
  #[serde(rename = "last_name")]
  last_name: Option<String>,
  #[serde(rename = "phone")]
  phone: Option<String>,
  #[serde(rename = "real_name")]
  real_name: String,
  #[serde(rename = "real_name_normalized")]
  real_name_normalized: String,
  #[serde(rename = "skype")]
  skype: Option<String>,
  #[serde(rename = "status_emoji")]
  status_emoji: Option<String>,
  #[serde(rename = "status_expiration")]
  status_expiration: Option<i32>,
  #[serde(rename = "status_text")]
  status_text: Option<String>,
  #[serde(rename = "status_text_canonical")]
  status_text_canonical: Option<String>,
  #[serde(rename = "team")]
  team: Option<::models::DefsWorkspaceId>,
  #[serde(rename = "teams")]
  teams: Option<::models::DefsWorkspaceId>,
  #[serde(rename = "title")]
  title: Option<String>
}

impl ObjsUserProfile {
  pub fn new(avatar_hash: String, display_name: String, display_name_normalized: String, real_name: String, real_name_normalized: String) -> ObjsUserProfile {
    ObjsUserProfile {
      always_active: None,
      api_app_id: None,
      avatar_hash: avatar_hash,
      bot_id: None,
      display_name: display_name,
      display_name_normalized: display_name_normalized,
      email: None,
      first_name: None,
      guest_expiration_ts: None,
      guest_invited_by: None,
      image_1024: None,
      image_192: None,
      image_24: None,
      image_32: None,
      image_48: None,
      image_512: None,
      image_72: None,
      image_original: None,
      is_custom_image: None,
      last_name: None,
      phone: None,
      real_name: real_name,
      real_name_normalized: real_name_normalized,
      skype: None,
      status_emoji: None,
      status_expiration: None,
      status_text: None,
      status_text_canonical: None,
      team: None,
      teams: None,
      title: None
    }
  }

  pub fn set_always_active(&mut self, always_active: bool) {
    self.always_active = Some(always_active);
  }

  pub fn with_always_active(mut self, always_active: bool) -> ObjsUserProfile {
    self.always_active = Some(always_active);
    self
  }

  pub fn always_active(&self) -> Option<&bool> {
    self.always_active.as_ref()
  }

  pub fn reset_always_active(&mut self) {
    self.always_active = None;
  }

  pub fn set_api_app_id(&mut self, api_app_id: ::models::DefsAppId) {
    self.api_app_id = Some(api_app_id);
  }

  pub fn with_api_app_id(mut self, api_app_id: ::models::DefsAppId) -> ObjsUserProfile {
    self.api_app_id = Some(api_app_id);
    self
  }

  pub fn api_app_id(&self) -> Option<&::models::DefsAppId> {
    self.api_app_id.as_ref()
  }

  pub fn reset_api_app_id(&mut self) {
    self.api_app_id = None;
  }

  pub fn set_avatar_hash(&mut self, avatar_hash: String) {
    self.avatar_hash = avatar_hash;
  }

  pub fn with_avatar_hash(mut self, avatar_hash: String) -> ObjsUserProfile {
    self.avatar_hash = avatar_hash;
    self
  }

  pub fn avatar_hash(&self) -> &String {
    &self.avatar_hash
  }


  pub fn set_bot_id(&mut self, bot_id: ::models::DefsBotId) {
    self.bot_id = Some(bot_id);
  }

  pub fn with_bot_id(mut self, bot_id: ::models::DefsBotId) -> ObjsUserProfile {
    self.bot_id = Some(bot_id);
    self
  }

  pub fn bot_id(&self) -> Option<&::models::DefsBotId> {
    self.bot_id.as_ref()
  }

  pub fn reset_bot_id(&mut self) {
    self.bot_id = None;
  }

  pub fn set_display_name(&mut self, display_name: String) {
    self.display_name = display_name;
  }

  pub fn with_display_name(mut self, display_name: String) -> ObjsUserProfile {
    self.display_name = display_name;
    self
  }

  pub fn display_name(&self) -> &String {
    &self.display_name
  }


  pub fn set_display_name_normalized(&mut self, display_name_normalized: String) {
    self.display_name_normalized = display_name_normalized;
  }

  pub fn with_display_name_normalized(mut self, display_name_normalized: String) -> ObjsUserProfile {
    self.display_name_normalized = display_name_normalized;
    self
  }

  pub fn display_name_normalized(&self) -> &String {
    &self.display_name_normalized
  }


  pub fn set_email(&mut self, email: String) {
    self.email = Some(email);
  }

  pub fn with_email(mut self, email: String) -> ObjsUserProfile {
    self.email = Some(email);
    self
  }

  pub fn email(&self) -> Option<&String> {
    self.email.as_ref()
  }

  pub fn reset_email(&mut self) {
    self.email = None;
  }

  pub fn set_first_name(&mut self, first_name: String) {
    self.first_name = Some(first_name);
  }

  pub fn with_first_name(mut self, first_name: String) -> ObjsUserProfile {
    self.first_name = Some(first_name);
    self
  }

  pub fn first_name(&self) -> Option<&String> {
    self.first_name.as_ref()
  }

  pub fn reset_first_name(&mut self) {
    self.first_name = None;
  }

  pub fn set_guest_expiration_ts(&mut self, guest_expiration_ts: i32) {
    self.guest_expiration_ts = Some(guest_expiration_ts);
  }

  pub fn with_guest_expiration_ts(mut self, guest_expiration_ts: i32) -> ObjsUserProfile {
    self.guest_expiration_ts = Some(guest_expiration_ts);
    self
  }

  pub fn guest_expiration_ts(&self) -> Option<&i32> {
    self.guest_expiration_ts.as_ref()
  }

  pub fn reset_guest_expiration_ts(&mut self) {
    self.guest_expiration_ts = None;
  }

  pub fn set_guest_invited_by(&mut self, guest_invited_by: String) {
    self.guest_invited_by = Some(guest_invited_by);
  }

  pub fn with_guest_invited_by(mut self, guest_invited_by: String) -> ObjsUserProfile {
    self.guest_invited_by = Some(guest_invited_by);
    self
  }

  pub fn guest_invited_by(&self) -> Option<&String> {
    self.guest_invited_by.as_ref()
  }

  pub fn reset_guest_invited_by(&mut self) {
    self.guest_invited_by = None;
  }

  pub fn set_image_1024(&mut self, image_1024: String) {
    self.image_1024 = Some(image_1024);
  }

  pub fn with_image_1024(mut self, image_1024: String) -> ObjsUserProfile {
    self.image_1024 = Some(image_1024);
    self
  }

  pub fn image_1024(&self) -> Option<&String> {
    self.image_1024.as_ref()
  }

  pub fn reset_image_1024(&mut self) {
    self.image_1024 = None;
  }

  pub fn set_image_192(&mut self, image_192: String) {
    self.image_192 = Some(image_192);
  }

  pub fn with_image_192(mut self, image_192: String) -> ObjsUserProfile {
    self.image_192 = Some(image_192);
    self
  }

  pub fn image_192(&self) -> Option<&String> {
    self.image_192.as_ref()
  }

  pub fn reset_image_192(&mut self) {
    self.image_192 = None;
  }

  pub fn set_image_24(&mut self, image_24: String) {
    self.image_24 = Some(image_24);
  }

  pub fn with_image_24(mut self, image_24: String) -> ObjsUserProfile {
    self.image_24 = Some(image_24);
    self
  }

  pub fn image_24(&self) -> Option<&String> {
    self.image_24.as_ref()
  }

  pub fn reset_image_24(&mut self) {
    self.image_24 = None;
  }

  pub fn set_image_32(&mut self, image_32: String) {
    self.image_32 = Some(image_32);
  }

  pub fn with_image_32(mut self, image_32: String) -> ObjsUserProfile {
    self.image_32 = Some(image_32);
    self
  }

  pub fn image_32(&self) -> Option<&String> {
    self.image_32.as_ref()
  }

  pub fn reset_image_32(&mut self) {
    self.image_32 = None;
  }

  pub fn set_image_48(&mut self, image_48: String) {
    self.image_48 = Some(image_48);
  }

  pub fn with_image_48(mut self, image_48: String) -> ObjsUserProfile {
    self.image_48 = Some(image_48);
    self
  }

  pub fn image_48(&self) -> Option<&String> {
    self.image_48.as_ref()
  }

  pub fn reset_image_48(&mut self) {
    self.image_48 = None;
  }

  pub fn set_image_512(&mut self, image_512: String) {
    self.image_512 = Some(image_512);
  }

  pub fn with_image_512(mut self, image_512: String) -> ObjsUserProfile {
    self.image_512 = Some(image_512);
    self
  }

  pub fn image_512(&self) -> Option<&String> {
    self.image_512.as_ref()
  }

  pub fn reset_image_512(&mut self) {
    self.image_512 = None;
  }

  pub fn set_image_72(&mut self, image_72: String) {
    self.image_72 = Some(image_72);
  }

  pub fn with_image_72(mut self, image_72: String) -> ObjsUserProfile {
    self.image_72 = Some(image_72);
    self
  }

  pub fn image_72(&self) -> Option<&String> {
    self.image_72.as_ref()
  }

  pub fn reset_image_72(&mut self) {
    self.image_72 = None;
  }

  pub fn set_image_original(&mut self, image_original: String) {
    self.image_original = Some(image_original);
  }

  pub fn with_image_original(mut self, image_original: String) -> ObjsUserProfile {
    self.image_original = Some(image_original);
    self
  }

  pub fn image_original(&self) -> Option<&String> {
    self.image_original.as_ref()
  }

  pub fn reset_image_original(&mut self) {
    self.image_original = None;
  }

  pub fn set_is_custom_image(&mut self, is_custom_image: bool) {
    self.is_custom_image = Some(is_custom_image);
  }

  pub fn with_is_custom_image(mut self, is_custom_image: bool) -> ObjsUserProfile {
    self.is_custom_image = Some(is_custom_image);
    self
  }

  pub fn is_custom_image(&self) -> Option<&bool> {
    self.is_custom_image.as_ref()
  }

  pub fn reset_is_custom_image(&mut self) {
    self.is_custom_image = None;
  }

  pub fn set_last_name(&mut self, last_name: String) {
    self.last_name = Some(last_name);
  }

  pub fn with_last_name(mut self, last_name: String) -> ObjsUserProfile {
    self.last_name = Some(last_name);
    self
  }

  pub fn last_name(&self) -> Option<&String> {
    self.last_name.as_ref()
  }

  pub fn reset_last_name(&mut self) {
    self.last_name = None;
  }

  pub fn set_phone(&mut self, phone: String) {
    self.phone = Some(phone);
  }

  pub fn with_phone(mut self, phone: String) -> ObjsUserProfile {
    self.phone = Some(phone);
    self
  }

  pub fn phone(&self) -> Option<&String> {
    self.phone.as_ref()
  }

  pub fn reset_phone(&mut self) {
    self.phone = None;
  }

  pub fn set_real_name(&mut self, real_name: String) {
    self.real_name = real_name;
  }

  pub fn with_real_name(mut self, real_name: String) -> ObjsUserProfile {
    self.real_name = real_name;
    self
  }

  pub fn real_name(&self) -> &String {
    &self.real_name
  }


  pub fn set_real_name_normalized(&mut self, real_name_normalized: String) {
    self.real_name_normalized = real_name_normalized;
  }

  pub fn with_real_name_normalized(mut self, real_name_normalized: String) -> ObjsUserProfile {
    self.real_name_normalized = real_name_normalized;
    self
  }

  pub fn real_name_normalized(&self) -> &String {
    &self.real_name_normalized
  }


  pub fn set_skype(&mut self, skype: String) {
    self.skype = Some(skype);
  }

  pub fn with_skype(mut self, skype: String) -> ObjsUserProfile {
    self.skype = Some(skype);
    self
  }

  pub fn skype(&self) -> Option<&String> {
    self.skype.as_ref()
  }

  pub fn reset_skype(&mut self) {
    self.skype = None;
  }

  pub fn set_status_emoji(&mut self, status_emoji: String) {
    self.status_emoji = Some(status_emoji);
  }

  pub fn with_status_emoji(mut self, status_emoji: String) -> ObjsUserProfile {
    self.status_emoji = Some(status_emoji);
    self
  }

  pub fn status_emoji(&self) -> Option<&String> {
    self.status_emoji.as_ref()
  }

  pub fn reset_status_emoji(&mut self) {
    self.status_emoji = None;
  }

  pub fn set_status_expiration(&mut self, status_expiration: i32) {
    self.status_expiration = Some(status_expiration);
  }

  pub fn with_status_expiration(mut self, status_expiration: i32) -> ObjsUserProfile {
    self.status_expiration = Some(status_expiration);
    self
  }

  pub fn status_expiration(&self) -> Option<&i32> {
    self.status_expiration.as_ref()
  }

  pub fn reset_status_expiration(&mut self) {
    self.status_expiration = None;
  }

  pub fn set_status_text(&mut self, status_text: String) {
    self.status_text = Some(status_text);
  }

  pub fn with_status_text(mut self, status_text: String) -> ObjsUserProfile {
    self.status_text = Some(status_text);
    self
  }

  pub fn status_text(&self) -> Option<&String> {
    self.status_text.as_ref()
  }

  pub fn reset_status_text(&mut self) {
    self.status_text = None;
  }

  pub fn set_status_text_canonical(&mut self, status_text_canonical: String) {
    self.status_text_canonical = Some(status_text_canonical);
  }

  pub fn with_status_text_canonical(mut self, status_text_canonical: String) -> ObjsUserProfile {
    self.status_text_canonical = Some(status_text_canonical);
    self
  }

  pub fn status_text_canonical(&self) -> Option<&String> {
    self.status_text_canonical.as_ref()
  }

  pub fn reset_status_text_canonical(&mut self) {
    self.status_text_canonical = None;
  }

  pub fn set_team(&mut self, team: ::models::DefsWorkspaceId) {
    self.team = Some(team);
  }

  pub fn with_team(mut self, team: ::models::DefsWorkspaceId) -> ObjsUserProfile {
    self.team = Some(team);
    self
  }

  pub fn team(&self) -> Option<&::models::DefsWorkspaceId> {
    self.team.as_ref()
  }

  pub fn reset_team(&mut self) {
    self.team = None;
  }

  pub fn set_teams(&mut self, teams: ::models::DefsWorkspaceId) {
    self.teams = Some(teams);
  }

  pub fn with_teams(mut self, teams: ::models::DefsWorkspaceId) -> ObjsUserProfile {
    self.teams = Some(teams);
    self
  }

  pub fn teams(&self) -> Option<&::models::DefsWorkspaceId> {
    self.teams.as_ref()
  }

  pub fn reset_teams(&mut self) {
    self.teams = None;
  }

  pub fn set_title(&mut self, title: String) {
    self.title = Some(title);
  }

  pub fn with_title(mut self, title: String) -> ObjsUserProfile {
    self.title = Some(title);
    self
  }

  pub fn title(&self) -> Option<&String> {
    self.title.as_ref()
  }

  pub fn reset_title(&mut self) {
    self.title = None;
  }

}



