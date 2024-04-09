use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, env)]
    pub auth0_tenant_uri: String,
    #[arg(long, env, value_delimiter = ',')]
    pub auth0_audiences: Vec<String>,
}
