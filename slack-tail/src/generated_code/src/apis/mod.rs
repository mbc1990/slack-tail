use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod admin_api;
pub use self::admin_api::{ AdminApi, AdminApiClient };
mod admin_apps_api;
pub use self::admin_apps_api::{ AdminAppsApi, AdminAppsApiClient };
mod admin_apps_approved_api;
pub use self::admin_apps_approved_api::{ AdminAppsApprovedApi, AdminAppsApprovedApiClient };
mod admin_apps_requests_api;
pub use self::admin_apps_requests_api::{ AdminAppsRequestsApi, AdminAppsRequestsApiClient };
mod admin_apps_restricted_api;
pub use self::admin_apps_restricted_api::{ AdminAppsRestrictedApi, AdminAppsRestrictedApiClient };
mod admin_conversations_api;
pub use self::admin_conversations_api::{ AdminConversationsApi, AdminConversationsApiClient };
mod admin_emoji_api;
pub use self::admin_emoji_api::{ AdminEmojiApi, AdminEmojiApiClient };
mod admin_invite_requests_api;
pub use self::admin_invite_requests_api::{ AdminInviteRequestsApi, AdminInviteRequestsApiClient };
mod admin_invite_requests_approved_api;
pub use self::admin_invite_requests_approved_api::{ AdminInviteRequestsApprovedApi, AdminInviteRequestsApprovedApiClient };
mod admin_invite_requests_denied_api;
pub use self::admin_invite_requests_denied_api::{ AdminInviteRequestsDeniedApi, AdminInviteRequestsDeniedApiClient };
mod admin_teams_api;
pub use self::admin_teams_api::{ AdminTeamsApi, AdminTeamsApiClient };
mod admin_teams_admins_api;
pub use self::admin_teams_admins_api::{ AdminTeamsAdminsApi, AdminTeamsAdminsApiClient };
mod admin_teams_owners_api;
pub use self::admin_teams_owners_api::{ AdminTeamsOwnersApi, AdminTeamsOwnersApiClient };
mod admin_teams_settings_api;
pub use self::admin_teams_settings_api::{ AdminTeamsSettingsApi, AdminTeamsSettingsApiClient };
mod admin_users_api;
pub use self::admin_users_api::{ AdminUsersApi, AdminUsersApiClient };
mod admin_users_session_api;
pub use self::admin_users_session_api::{ AdminUsersSessionApi, AdminUsersSessionApiClient };
mod api_api;
pub use self::api_api::{ ApiApi, ApiApiClient };
mod apps_api;
pub use self::apps_api::{ AppsApi, AppsApiClient };
mod apps_permissions_api;
pub use self::apps_permissions_api::{ AppsPermissionsApi, AppsPermissionsApiClient };
mod apps_permissions_resources_api;
pub use self::apps_permissions_resources_api::{ AppsPermissionsResourcesApi, AppsPermissionsResourcesApiClient };
mod apps_permissions_scopes_api;
pub use self::apps_permissions_scopes_api::{ AppsPermissionsScopesApi, AppsPermissionsScopesApiClient };
mod apps_permissions_users_api;
pub use self::apps_permissions_users_api::{ AppsPermissionsUsersApi, AppsPermissionsUsersApiClient };
mod auth_api;
pub use self::auth_api::{ AuthApi, AuthApiClient };
mod bots_api;
pub use self::bots_api::{ BotsApi, BotsApiClient };
mod channels_api;
pub use self::channels_api::{ ChannelsApi, ChannelsApiClient };
mod chat_api;
pub use self::chat_api::{ ChatApi, ChatApiClient };
mod chat_scheduled_messages_api;
pub use self::chat_scheduled_messages_api::{ ChatScheduledMessagesApi, ChatScheduledMessagesApiClient };
mod conversations_api;
pub use self::conversations_api::{ ConversationsApi, ConversationsApiClient };
mod dialog_api;
pub use self::dialog_api::{ DialogApi, DialogApiClient };
mod dnd_api;
pub use self::dnd_api::{ DndApi, DndApiClient };
mod emoji_api;
pub use self::emoji_api::{ EmojiApi, EmojiApiClient };
mod files_api;
pub use self::files_api::{ FilesApi, FilesApiClient };
mod files_comments_api;
pub use self::files_comments_api::{ FilesCommentsApi, FilesCommentsApiClient };
mod files_remote_api;
pub use self::files_remote_api::{ FilesRemoteApi, FilesRemoteApiClient };
mod groups_api;
pub use self::groups_api::{ GroupsApi, GroupsApiClient };
mod im_api;
pub use self::im_api::{ ImApi, ImApiClient };
mod migration_api;
pub use self::migration_api::{ MigrationApi, MigrationApiClient };
mod mpim_api;
pub use self::mpim_api::{ MpimApi, MpimApiClient };
mod oauth_api;
pub use self::oauth_api::{ OauthApi, OauthApiClient };
mod oauth_v2_api;
pub use self::oauth_v2_api::{ OauthV2Api, OauthV2ApiClient };
mod pins_api;
pub use self::pins_api::{ PinsApi, PinsApiClient };
mod reactions_api;
pub use self::reactions_api::{ ReactionsApi, ReactionsApiClient };
mod reminders_api;
pub use self::reminders_api::{ RemindersApi, RemindersApiClient };
mod rtm_api;
pub use self::rtm_api::{ RtmApi, RtmApiClient };
mod search_api;
pub use self::search_api::{ SearchApi, SearchApiClient };
mod stars_api;
pub use self::stars_api::{ StarsApi, StarsApiClient };
mod team_api;
pub use self::team_api::{ TeamApi, TeamApiClient };
mod team_profile_api;
pub use self::team_profile_api::{ TeamProfileApi, TeamProfileApiClient };
mod usergroups_api;
pub use self::usergroups_api::{ UsergroupsApi, UsergroupsApiClient };
mod usergroups_users_api;
pub use self::usergroups_users_api::{ UsergroupsUsersApi, UsergroupsUsersApiClient };
mod users_api;
pub use self::users_api::{ UsersApi, UsersApiClient };
mod users_profile_api;
pub use self::users_profile_api::{ UsersProfileApi, UsersProfileApiClient };
mod views_api;
pub use self::views_api::{ ViewsApi, ViewsApiClient };

pub mod configuration;
pub mod client;
