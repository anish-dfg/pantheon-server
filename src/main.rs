mod app;
mod cli;
mod services;
mod state;

use clap::Parser;
use cli::Args;
use state::{AppState, State};

use services::{
    airtable::Airtable,
    auth::auth0::Auth0,
    storage::{Cache, Sql},
};

use crate::services::workspace::service_account::ServiceAccountWorkspaceClient;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("error loading environment variables");

    let args = Args::parse();

    let authenticator = Box::new(
        Auth0::new(&args.auth0_tenant_uri, args.auth0_audiences)
            .await
            .expect("error initializing auth backend"),
    );

    let workspace_client = Box::new(ServiceAccountWorkspaceClient::new(
        &args.workspace_client_email,
        &args.workspace_private_key_id,
        &args.workspace_private_key,
        &args.workspace_token_uri,
    ));

    let airtable = Airtable::new(&args.airtable_api_token);

    let sql = Sql::new(&args.database_url)
        .await
        .expect("error creating storage backend");

    let cache = Cache::new(&args.cache_url).expect("error");

    sqlx::migrate!().run(&sql.pg).await.expect("error running migrations");

    let state = AppState::new(State {
        authenticator,
        workspace_client,
        airtable,
        sql,
        cache,
    });
    let router = app::routes(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888")
        .await
        .expect("could not bind to tcp listener");

    axum::serve(listener, router).await.expect("failed to start app");
}
