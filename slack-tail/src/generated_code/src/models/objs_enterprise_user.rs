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
pub struct ObjsEnterpriseUser {
  #[serde(rename = "enterprise_id")]
  enterprise_id: ::models::DefsEnterpriseId,
  #[serde(rename = "enterprise_name")]
  enterprise_name: ::models::DefsEnterpriseName,
  #[serde(rename = "id")]
  id: ::models::DefsEnterpriseUserId,
  #[serde(rename = "is_admin")]
  is_admin: bool,
  #[serde(rename = "is_owner")]
  is_owner: bool,
  #[serde(rename = "teams")]
  teams: Vec<::models::DefsTeam>
}

impl ObjsEnterpriseUser {
  pub fn new(enterprise_id: ::models::DefsEnterpriseId, enterprise_name: ::models::DefsEnterpriseName, id: ::models::DefsEnterpriseUserId, is_admin: bool, is_owner: bool, teams: Vec<::models::DefsTeam>) -> ObjsEnterpriseUser {
    ObjsEnterpriseUser {
      enterprise_id: enterprise_id,
      enterprise_name: enterprise_name,
      id: id,
      is_admin: is_admin,
      is_owner: is_owner,
      teams: teams
    }
  }

  pub fn set_enterprise_id(&mut self, enterprise_id: ::models::DefsEnterpriseId) {
    self.enterprise_id = enterprise_id;
  }

  pub fn with_enterprise_id(mut self, enterprise_id: ::models::DefsEnterpriseId) -> ObjsEnterpriseUser {
    self.enterprise_id = enterprise_id;
    self
  }

  pub fn enterprise_id(&self) -> &::models::DefsEnterpriseId {
    &self.enterprise_id
  }


  pub fn set_enterprise_name(&mut self, enterprise_name: ::models::DefsEnterpriseName) {
    self.enterprise_name = enterprise_name;
  }

  pub fn with_enterprise_name(mut self, enterprise_name: ::models::DefsEnterpriseName) -> ObjsEnterpriseUser {
    self.enterprise_name = enterprise_name;
    self
  }

  pub fn enterprise_name(&self) -> &::models::DefsEnterpriseName {
    &self.enterprise_name
  }


  pub fn set_id(&mut self, id: ::models::DefsEnterpriseUserId) {
    self.id = id;
  }

  pub fn with_id(mut self, id: ::models::DefsEnterpriseUserId) -> ObjsEnterpriseUser {
    self.id = id;
    self
  }

  pub fn id(&self) -> &::models::DefsEnterpriseUserId {
    &self.id
  }


  pub fn set_is_admin(&mut self, is_admin: bool) {
    self.is_admin = is_admin;
  }

  pub fn with_is_admin(mut self, is_admin: bool) -> ObjsEnterpriseUser {
    self.is_admin = is_admin;
    self
  }

  pub fn is_admin(&self) -> &bool {
    &self.is_admin
  }


  pub fn set_is_owner(&mut self, is_owner: bool) {
    self.is_owner = is_owner;
  }

  pub fn with_is_owner(mut self, is_owner: bool) -> ObjsEnterpriseUser {
    self.is_owner = is_owner;
    self
  }

  pub fn is_owner(&self) -> &bool {
    &self.is_owner
  }


  pub fn set_teams(&mut self, teams: Vec<::models::DefsTeam>) {
    self.teams = teams;
  }

  pub fn with_teams(mut self, teams: Vec<::models::DefsTeam>) -> ObjsEnterpriseUser {
    self.teams = teams;
    self
  }

  pub fn teams(&self) -> &Vec<::models::DefsTeam> {
    &self.teams
  }


}


