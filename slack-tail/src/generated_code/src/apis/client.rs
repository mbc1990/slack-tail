use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  admin_api: Box<::apis::AdminApi>,
  admin_apps_api: Box<::apis::AdminAppsApi>,
  admin_apps_approved_api: Box<::apis::AdminAppsApprovedApi>,
  admin_apps_requests_api: Box<::apis::AdminAppsRequestsApi>,
  admin_apps_restricted_api: Box<::apis::AdminAppsRestrictedApi>,
  admin_conversations_api: Box<::apis::AdminConversationsApi>,
  admin_emoji_api: Box<::apis::AdminEmojiApi>,
  admin_invite_requests_api: Box<::apis::AdminInviteRequestsApi>,
  admin_invite_requests_approved_api: Box<::apis::AdminInviteRequestsApprovedApi>,
  admin_invite_requests_denied_api: Box<::apis::AdminInviteRequestsDeniedApi>,
  admin_teams_api: Box<::apis::AdminTeamsApi>,
  admin_teams_admins_api: Box<::apis::AdminTeamsAdminsApi>,
  admin_teams_owners_api: Box<::apis::AdminTeamsOwnersApi>,
  admin_teams_settings_api: Box<::apis::AdminTeamsSettingsApi>,
  admin_users_api: Box<::apis::AdminUsersApi>,
  admin_users_session_api: Box<::apis::AdminUsersSessionApi>,
  api_api: Box<::apis::ApiApi>,
  apps_api: Box<::apis::AppsApi>,
  apps_permissions_api: Box<::apis::AppsPermissionsApi>,
  apps_permissions_resources_api: Box<::apis::AppsPermissionsResourcesApi>,
  apps_permissions_scopes_api: Box<::apis::AppsPermissionsScopesApi>,
  apps_permissions_users_api: Box<::apis::AppsPermissionsUsersApi>,
  auth_api: Box<::apis::AuthApi>,
  bots_api: Box<::apis::BotsApi>,
  channels_api: Box<::apis::ChannelsApi>,
  chat_api: Box<::apis::ChatApi>,
  chat_scheduled_messages_api: Box<::apis::ChatScheduledMessagesApi>,
  conversations_api: Box<::apis::ConversationsApi>,
  dialog_api: Box<::apis::DialogApi>,
  dnd_api: Box<::apis::DndApi>,
  emoji_api: Box<::apis::EmojiApi>,
  files_api: Box<::apis::FilesApi>,
  files_comments_api: Box<::apis::FilesCommentsApi>,
  files_remote_api: Box<::apis::FilesRemoteApi>,
  groups_api: Box<::apis::GroupsApi>,
  im_api: Box<::apis::ImApi>,
  migration_api: Box<::apis::MigrationApi>,
  mpim_api: Box<::apis::MpimApi>,
  oauth_api: Box<::apis::OauthApi>,
  oauth_v2_api: Box<::apis::OauthV2Api>,
  pins_api: Box<::apis::PinsApi>,
  reactions_api: Box<::apis::ReactionsApi>,
  reminders_api: Box<::apis::RemindersApi>,
  rtm_api: Box<::apis::RtmApi>,
  search_api: Box<::apis::SearchApi>,
  stars_api: Box<::apis::StarsApi>,
  team_api: Box<::apis::TeamApi>,
  team_profile_api: Box<::apis::TeamProfileApi>,
  usergroups_api: Box<::apis::UsergroupsApi>,
  usergroups_users_api: Box<::apis::UsergroupsUsersApi>,
  users_api: Box<::apis::UsersApi>,
  users_profile_api: Box<::apis::UsersProfileApi>,
  views_api: Box<::apis::ViewsApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      admin_api: Box::new(::apis::AdminApiClient::new(rc.clone())),
      admin_apps_api: Box::new(::apis::AdminAppsApiClient::new(rc.clone())),
      admin_apps_approved_api: Box::new(::apis::AdminAppsApprovedApiClient::new(rc.clone())),
      admin_apps_requests_api: Box::new(::apis::AdminAppsRequestsApiClient::new(rc.clone())),
      admin_apps_restricted_api: Box::new(::apis::AdminAppsRestrictedApiClient::new(rc.clone())),
      admin_conversations_api: Box::new(::apis::AdminConversationsApiClient::new(rc.clone())),
      admin_emoji_api: Box::new(::apis::AdminEmojiApiClient::new(rc.clone())),
      admin_invite_requests_api: Box::new(::apis::AdminInviteRequestsApiClient::new(rc.clone())),
      admin_invite_requests_approved_api: Box::new(::apis::AdminInviteRequestsApprovedApiClient::new(rc.clone())),
      admin_invite_requests_denied_api: Box::new(::apis::AdminInviteRequestsDeniedApiClient::new(rc.clone())),
      admin_teams_api: Box::new(::apis::AdminTeamsApiClient::new(rc.clone())),
      admin_teams_admins_api: Box::new(::apis::AdminTeamsAdminsApiClient::new(rc.clone())),
      admin_teams_owners_api: Box::new(::apis::AdminTeamsOwnersApiClient::new(rc.clone())),
      admin_teams_settings_api: Box::new(::apis::AdminTeamsSettingsApiClient::new(rc.clone())),
      admin_users_api: Box::new(::apis::AdminUsersApiClient::new(rc.clone())),
      admin_users_session_api: Box::new(::apis::AdminUsersSessionApiClient::new(rc.clone())),
      api_api: Box::new(::apis::ApiApiClient::new(rc.clone())),
      apps_api: Box::new(::apis::AppsApiClient::new(rc.clone())),
      apps_permissions_api: Box::new(::apis::AppsPermissionsApiClient::new(rc.clone())),
      apps_permissions_resources_api: Box::new(::apis::AppsPermissionsResourcesApiClient::new(rc.clone())),
      apps_permissions_scopes_api: Box::new(::apis::AppsPermissionsScopesApiClient::new(rc.clone())),
      apps_permissions_users_api: Box::new(::apis::AppsPermissionsUsersApiClient::new(rc.clone())),
      auth_api: Box::new(::apis::AuthApiClient::new(rc.clone())),
      bots_api: Box::new(::apis::BotsApiClient::new(rc.clone())),
      channels_api: Box::new(::apis::ChannelsApiClient::new(rc.clone())),
      chat_api: Box::new(::apis::ChatApiClient::new(rc.clone())),
      chat_scheduled_messages_api: Box::new(::apis::ChatScheduledMessagesApiClient::new(rc.clone())),
      conversations_api: Box::new(::apis::ConversationsApiClient::new(rc.clone())),
      dialog_api: Box::new(::apis::DialogApiClient::new(rc.clone())),
      dnd_api: Box::new(::apis::DndApiClient::new(rc.clone())),
      emoji_api: Box::new(::apis::EmojiApiClient::new(rc.clone())),
      files_api: Box::new(::apis::FilesApiClient::new(rc.clone())),
      files_comments_api: Box::new(::apis::FilesCommentsApiClient::new(rc.clone())),
      files_remote_api: Box::new(::apis::FilesRemoteApiClient::new(rc.clone())),
      groups_api: Box::new(::apis::GroupsApiClient::new(rc.clone())),
      im_api: Box::new(::apis::ImApiClient::new(rc.clone())),
      migration_api: Box::new(::apis::MigrationApiClient::new(rc.clone())),
      mpim_api: Box::new(::apis::MpimApiClient::new(rc.clone())),
      oauth_api: Box::new(::apis::OauthApiClient::new(rc.clone())),
      oauth_v2_api: Box::new(::apis::OauthV2ApiClient::new(rc.clone())),
      pins_api: Box::new(::apis::PinsApiClient::new(rc.clone())),
      reactions_api: Box::new(::apis::ReactionsApiClient::new(rc.clone())),
      reminders_api: Box::new(::apis::RemindersApiClient::new(rc.clone())),
      rtm_api: Box::new(::apis::RtmApiClient::new(rc.clone())),
      search_api: Box::new(::apis::SearchApiClient::new(rc.clone())),
      stars_api: Box::new(::apis::StarsApiClient::new(rc.clone())),
      team_api: Box::new(::apis::TeamApiClient::new(rc.clone())),
      team_profile_api: Box::new(::apis::TeamProfileApiClient::new(rc.clone())),
      usergroups_api: Box::new(::apis::UsergroupsApiClient::new(rc.clone())),
      usergroups_users_api: Box::new(::apis::UsergroupsUsersApiClient::new(rc.clone())),
      users_api: Box::new(::apis::UsersApiClient::new(rc.clone())),
      users_profile_api: Box::new(::apis::UsersProfileApiClient::new(rc.clone())),
      views_api: Box::new(::apis::ViewsApiClient::new(rc.clone())),
    }
  }

  pub fn admin_api(&self) -> &::apis::AdminApi{
    self.admin_api.as_ref()
  }

  pub fn admin_apps_api(&self) -> &::apis::AdminAppsApi{
    self.admin_apps_api.as_ref()
  }

  pub fn admin_apps_approved_api(&self) -> &::apis::AdminAppsApprovedApi{
    self.admin_apps_approved_api.as_ref()
  }

  pub fn admin_apps_requests_api(&self) -> &::apis::AdminAppsRequestsApi{
    self.admin_apps_requests_api.as_ref()
  }

  pub fn admin_apps_restricted_api(&self) -> &::apis::AdminAppsRestrictedApi{
    self.admin_apps_restricted_api.as_ref()
  }

  pub fn admin_conversations_api(&self) -> &::apis::AdminConversationsApi{
    self.admin_conversations_api.as_ref()
  }

  pub fn admin_emoji_api(&self) -> &::apis::AdminEmojiApi{
    self.admin_emoji_api.as_ref()
  }

  pub fn admin_invite_requests_api(&self) -> &::apis::AdminInviteRequestsApi{
    self.admin_invite_requests_api.as_ref()
  }

  pub fn admin_invite_requests_approved_api(&self) -> &::apis::AdminInviteRequestsApprovedApi{
    self.admin_invite_requests_approved_api.as_ref()
  }

  pub fn admin_invite_requests_denied_api(&self) -> &::apis::AdminInviteRequestsDeniedApi{
    self.admin_invite_requests_denied_api.as_ref()
  }

  pub fn admin_teams_api(&self) -> &::apis::AdminTeamsApi{
    self.admin_teams_api.as_ref()
  }

  pub fn admin_teams_admins_api(&self) -> &::apis::AdminTeamsAdminsApi{
    self.admin_teams_admins_api.as_ref()
  }

  pub fn admin_teams_owners_api(&self) -> &::apis::AdminTeamsOwnersApi{
    self.admin_teams_owners_api.as_ref()
  }

  pub fn admin_teams_settings_api(&self) -> &::apis::AdminTeamsSettingsApi{
    self.admin_teams_settings_api.as_ref()
  }

  pub fn admin_users_api(&self) -> &::apis::AdminUsersApi{
    self.admin_users_api.as_ref()
  }

  pub fn admin_users_session_api(&self) -> &::apis::AdminUsersSessionApi{
    self.admin_users_session_api.as_ref()
  }

  pub fn api_api(&self) -> &::apis::ApiApi{
    self.api_api.as_ref()
  }

  pub fn apps_api(&self) -> &::apis::AppsApi{
    self.apps_api.as_ref()
  }

  pub fn apps_permissions_api(&self) -> &::apis::AppsPermissionsApi{
    self.apps_permissions_api.as_ref()
  }

  pub fn apps_permissions_resources_api(&self) -> &::apis::AppsPermissionsResourcesApi{
    self.apps_permissions_resources_api.as_ref()
  }

  pub fn apps_permissions_scopes_api(&self) -> &::apis::AppsPermissionsScopesApi{
    self.apps_permissions_scopes_api.as_ref()
  }

  pub fn apps_permissions_users_api(&self) -> &::apis::AppsPermissionsUsersApi{
    self.apps_permissions_users_api.as_ref()
  }

  pub fn auth_api(&self) -> &::apis::AuthApi{
    self.auth_api.as_ref()
  }

  pub fn bots_api(&self) -> &::apis::BotsApi{
    self.bots_api.as_ref()
  }

  pub fn channels_api(&self) -> &::apis::ChannelsApi{
    self.channels_api.as_ref()
  }

  pub fn chat_api(&self) -> &::apis::ChatApi{
    self.chat_api.as_ref()
  }

  pub fn chat_scheduled_messages_api(&self) -> &::apis::ChatScheduledMessagesApi{
    self.chat_scheduled_messages_api.as_ref()
  }

  pub fn conversations_api(&self) -> &::apis::ConversationsApi{
    self.conversations_api.as_ref()
  }

  pub fn dialog_api(&self) -> &::apis::DialogApi{
    self.dialog_api.as_ref()
  }

  pub fn dnd_api(&self) -> &::apis::DndApi{
    self.dnd_api.as_ref()
  }

  pub fn emoji_api(&self) -> &::apis::EmojiApi{
    self.emoji_api.as_ref()
  }

  pub fn files_api(&self) -> &::apis::FilesApi{
    self.files_api.as_ref()
  }

  pub fn files_comments_api(&self) -> &::apis::FilesCommentsApi{
    self.files_comments_api.as_ref()
  }

  pub fn files_remote_api(&self) -> &::apis::FilesRemoteApi{
    self.files_remote_api.as_ref()
  }

  pub fn groups_api(&self) -> &::apis::GroupsApi{
    self.groups_api.as_ref()
  }

  pub fn im_api(&self) -> &::apis::ImApi{
    self.im_api.as_ref()
  }

  pub fn migration_api(&self) -> &::apis::MigrationApi{
    self.migration_api.as_ref()
  }

  pub fn mpim_api(&self) -> &::apis::MpimApi{
    self.mpim_api.as_ref()
  }

  pub fn oauth_api(&self) -> &::apis::OauthApi{
    self.oauth_api.as_ref()
  }

  pub fn oauth_v2_api(&self) -> &::apis::OauthV2Api{
    self.oauth_v2_api.as_ref()
  }

  pub fn pins_api(&self) -> &::apis::PinsApi{
    self.pins_api.as_ref()
  }

  pub fn reactions_api(&self) -> &::apis::ReactionsApi{
    self.reactions_api.as_ref()
  }

  pub fn reminders_api(&self) -> &::apis::RemindersApi{
    self.reminders_api.as_ref()
  }

  pub fn rtm_api(&self) -> &::apis::RtmApi{
    self.rtm_api.as_ref()
  }

  pub fn search_api(&self) -> &::apis::SearchApi{
    self.search_api.as_ref()
  }

  pub fn stars_api(&self) -> &::apis::StarsApi{
    self.stars_api.as_ref()
  }

  pub fn team_api(&self) -> &::apis::TeamApi{
    self.team_api.as_ref()
  }

  pub fn team_profile_api(&self) -> &::apis::TeamProfileApi{
    self.team_profile_api.as_ref()
  }

  pub fn usergroups_api(&self) -> &::apis::UsergroupsApi{
    self.usergroups_api.as_ref()
  }

  pub fn usergroups_users_api(&self) -> &::apis::UsergroupsUsersApi{
    self.usergroups_users_api.as_ref()
  }

  pub fn users_api(&self) -> &::apis::UsersApi{
    self.users_api.as_ref()
  }

  pub fn users_profile_api(&self) -> &::apis::UsersProfileApi{
    self.users_profile_api.as_ref()
  }

  pub fn views_api(&self) -> &::apis::ViewsApi{
    self.views_api.as_ref()
  }


}
