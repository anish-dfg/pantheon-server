use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceUser {
    pub kind: String,
    pub id: String,
    pub etag: String,
    pub primary_email: String,
    pub name: Name,
    pub is_admin: bool,
    pub is_delegated_admin: bool,
    pub last_login_time: String,
    pub creation_time: String,
    pub agreed_to_terms: bool,
    pub suspended: bool,
    pub archived: bool,
    pub change_password_at_next_login: bool,
    pub ip_whitelisted: bool,
    pub emails: Vec<Email>,
    pub languages: Vec<Language>,
    pub non_editable_aliases: Vec<String>,
    pub customer_id: String,
    pub org_unit_path: String,
    pub is_mailbox_setup: bool,
    #[serde(rename = "isEnrolledIn2Sv")]
    pub is_enrolled_in2sv: bool,
    #[serde(rename = "isEnforcedIn2Sv")]
    pub is_enforced_in2sv: bool,
    pub include_in_global_address_list: bool,
    pub recovery_email: Option<String>,
}

#[allow(clippy::struct_field_names)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub given_name: String,
    pub family_name: String,
    pub full_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    pub address: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub primary: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    pub language_code: String,
    pub preference: String,
}

pub type WorkspaceUsers = Vec<WorkspaceUser>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceUserData {
    pub kind: String,
    pub etag: String,
    pub users: WorkspaceUsers,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkspaceUser {
    pub primary_email: String,
    pub name: Name,
    pub password: String,
    pub change_password_at_next_login: bool,
}
