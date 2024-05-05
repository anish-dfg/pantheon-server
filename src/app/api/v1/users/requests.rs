use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub generated_email: Option<String>,
}

impl PartialEq for ExportUser {
    fn eq(&self, other: &Self) -> bool {
        match (&self.generated_email, &other.generated_email) {
            (Some(first), Some(second)) => {
                self.first_name == other.first_name
                    && self.last_name == other.last_name
                    && self.email == other.email
                    && first == second
            }
            (_, _) => {
                self.first_name == other.first_name && self.last_name == other.last_name && self.email == other.email
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum DownloadUsersFileFormat {
    Csv,
    Json,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadUserData {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    // #[serde(rename(deserialize = "ProjectName (from OrgName)"))]
    pub project: Option<String>,
    // #[serde(rename(deserialize = "RecordID"))]
    pub record_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadUsersRequest {
    pub send_to: String,
    pub format: DownloadUsersFileFormat,
    pub user_data: Vec<DownloadUserData>,
    pub columns: Vec<String>,
}
