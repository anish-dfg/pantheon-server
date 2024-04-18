use super::{
    users::{CreateWorkspaceUser, WorkspaceUser, WorkspaceUserData},
    WorkspaceClient,
};
use anyhow::{Context, Result};
use axum::async_trait;
use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct ServiceAccountWorkspaceClient {
    pub client_email: String,
    pub private_key_id: String,
    pub private_key: String,
    pub token_uri: String,
    pub http: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct Assertion {
    pub grant_type: String,
    pub assertion: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AssertionClaims {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub scope: String,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GoogleAccessTokenResponse {
    pub access_token: String,
}

impl ServiceAccountWorkspaceClient {
    const BEARER_TOKEN_GRANT_TYPE: &'static str = "urn:ietf:params:oauth:grant-type:jwt-bearer";

    pub fn new(client_email: &str, private_key_id: &str, private_key: &str, token_uri: &str) -> Self {
        Self {
            client_email: client_email.into(),
            private_key_id: private_key_id.into(),
            private_key: private_key.into(),
            token_uri: token_uri.into(),
            http: Client::new(),
        }
    }

    pub fn create_assertion_token(&self, impersonate: &str, scope: &str) -> Result<String> {
        let assertion_claims = AssertionClaims {
            iss: self.client_email.clone(),
            aud: self.token_uri.clone(),
            sub: impersonate.to_owned(),
            scope: scope.to_owned(),

            iat: Utc::now().timestamp(),
            exp: Utc::now().timestamp() + 3600,
        };

        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(self.private_key_id.clone());
        let assertion = jsonwebtoken::encode(
            &header,
            &assertion_claims,
            &EncodingKey::from_rsa_pem(self.private_key.as_bytes()).context("create assertion token")?,
        )?;
        Ok(assertion)
    }

    pub async fn get_access_token(&self, impersonate: &str, scope: &str) -> Result<String> {
        let assertion_token = self.create_assertion_token(impersonate, scope)?;
        let assertion = Assertion {
            assertion: assertion_token.to_owned(),
            grant_type: Self::BEARER_TOKEN_GRANT_TYPE.into(),
        };

        let res = self
            .http
            .get(&self.token_uri)
            .json(&assertion)
            .send()
            .await
            .context("fetch access token response")?;

        let data = res
            .json::<GoogleAccessTokenResponse>()
            .await
            .context("deserialize access token response")?;

        Ok(data.access_token)
    }
}

#[async_trait]
impl WorkspaceClient for ServiceAccountWorkspaceClient {
    async fn list_users(&self, impersonate: &str) -> Result<WorkspaceUserData> {
        // let access_token = self.get_access_token(
        //     impersonate,
        //     "https://www.googleapis.com/auth/admin.directory.user.readonly",
        // );
        todo!()
    }

    async fn create_user(&self, impersonate: &str, user: CreateWorkspaceUser) -> Result<()> {
        let access_token = self
            .get_access_token(impersonate, "https://www.googleapis.com/auth/admin.directory.user")
            .await?;
        let auth_header = format!("Bearer {access_token}");
        let url = "https://admin.googleapis.com/admin/directory/v1/users";

        let _ = self
            .http
            .post(url)
            .header("Authorization", auth_header)
            .json(&user)
            .send()
            .await?;

        Ok(())
    }
}
