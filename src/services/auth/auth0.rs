use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use jsonwebtoken::{
    jwk::{AlgorithmParameters, JwkSet},
    DecodingKey, Validation,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{userdata::UserData, Authenticator};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth0Configuration {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub device_authorization_endpoint: String,
    pub userinfo_endpoint: String,
    pub mfa_challenge_endpoint: String,
    pub jwks_uri: String,
    pub registration_endpoint: String,
    pub revocation_endpoint: String,
    pub scopes_supported: Vec<String>,
    pub response_types_supported: Vec<String>,
    pub code_challenge_methods_supported: Vec<String>,
    pub response_modes_supported: Vec<String>,
    pub subject_types_supported: Vec<String>,
    pub id_token_signing_alg_values_supported: Vec<String>,
    pub token_endpoint_auth_methods_supported: Vec<String>,
    pub claims_supported: Vec<String>,
    pub request_uri_parameter_supported: bool,
    pub request_parameter_supported: bool,
    pub token_endpoint_auth_signing_alg_values_supported: Vec<String>,
    pub backchannel_logout_supported: Option<bool>,
    pub backchannel_logout_session_supported: Option<bool>,
    pub end_session_endpoint: String,
}

#[derive(Debug, Clone)]
pub struct Auth0 {
    pub tenant_base_uri: String,
    pub audiences: Vec<String>,
    pub configuration: Auth0Configuration,
    http: Client,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub sub: String,
    pub nickname: String,
    pub name: String,
    pub picture: String,
    pub updated_at: DateTime<Utc>,
    pub email: String,
    pub email_verified: bool,
}

impl Auth0 {
    const DISCOVERY_ENDPOINT_SUFFIX: &'static str = "/.well-known/openid-configuration";
    const USERINFO_ENDPOINT: &'static str = "/userinfo";

    pub async fn new(tenant_base_uri: &str, audiences: Vec<String>) -> Result<Self> {
        let http = Client::new();
        let discovery_endpoint = tenant_base_uri.to_owned() + Self::DISCOVERY_ENDPOINT_SUFFIX;
        let res = http
            .get(&discovery_endpoint)
            .send()
            .await
            .context("fetch auth0 openid configuration")?;

        let configuration = res
            .json::<Auth0Configuration>()
            .await
            .context("deserialize auth0 openid configuration")?;

        Ok(Self {
            tenant_base_uri: tenant_base_uri.into(),
            audiences,
            configuration,
            http,
        })
    }
}

#[async_trait]
impl Authenticator for Auth0 {
    async fn authenticate(&self, token: &str) -> Result<UserData> {
        let http = &self.http;
        let jwks_uri = &self.configuration.jwks_uri;

        let res = http.get(jwks_uri).send().await.context("fetch auth0 jwks")?;

        let jwks = res.json::<JwkSet>().await.context("deserialize auth0 jwks")?;

        let header = jsonwebtoken::decode_header(token).context("decode auth0 token header")?;

        let Some(kid) = header.kid else { bail!("missing key id") };

        // check if any valid jwk id matches the kid in the token header
        let Some(jwk) = jwks.keys.into_iter().find(|jwk| match jwk.common.key_id.clone() {
            Some(jwk_id) => jwk_id == kid,
            None => false,
        }) else {
            bail!("no matching key id")
        };

        let decoded = match jwk.algorithm {
            AlgorithmParameters::EllipticCurve(_) => bail!("unimplemented algorithm"),
            AlgorithmParameters::RSA(rsa) => {
                let (n, e) = (rsa.n, rsa.e);
                let mut validator = Validation::new(header.alg);
                validator.set_audience(&self.audiences);
                let Ok(decoded) = jsonwebtoken::decode::<Value>(
                    token,
                    &DecodingKey::from_rsa_components(&n, &e).context("create decoding key from rsa components")?,
                    &validator,
                ) else {
                    bail!("unable to verify token signature");
                };
                decoded
            }
            AlgorithmParameters::OctetKey(_) => bail!("unimplemented algorithm"),
            AlgorithmParameters::OctetKeyPair(_) => bail!("unimplemented algorithm"),
        };

        let userinfo_endpoint = self.tenant_base_uri.clone() + Self::USERINFO_ENDPOINT;
        let userinfo_res = self
            .http
            .get(userinfo_endpoint)
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await
            .context("fetch user info")?;

        let user_data = userinfo_res
            .json::<UserInfo>()
            .await
            .context("deserialize user information")?;

        Ok(UserData::Auth0(user_data))
    }
}
