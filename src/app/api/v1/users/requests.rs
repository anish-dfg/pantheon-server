use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailPolicy {
    pub use_both_first_and_last_names: bool,
    pub add_unique_numeric_suffix: bool,
    pub separator: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordPolicy {
    pub change_password_at_next_login: bool,
    pub generated_password_length: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExportUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
// #[serde(untagged)]
pub enum ExportConflictPolicy {
    ExportDifference,
    Reject,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportUsersRequest {
    pub users: Vec<ExportUser>,
    pub email_policy: EmailPolicy,
    pub password_policy: PasswordPolicy,
    pub export_conflict_policy: ExportConflictPolicy,
}
