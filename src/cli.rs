use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, env)]
    pub auth0_tenant_uri: String,
    #[arg(long, env, value_delimiter = ',')]
    pub auth0_audiences: Vec<String>,
    #[arg(long, env)]
    pub workspace_client_email: String,
    #[arg(long, env)]
    pub workspace_private_key_id: String,
    #[arg(long, env)]
    pub workspace_private_key: String,
    #[arg(long, env, default_value = "https://oauth2.googleapis.com/token")]
    pub workspace_token_uri: String,
    #[arg(long, env)]
    pub airtable_api_token: String,
    #[arg(long, env)]
    pub database_url: String,
    #[arg(long, env)]
    pub cache_url: String,
    #[arg(long, env)]
    pub sendgrid_api_key: String,
}
