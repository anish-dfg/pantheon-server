use auth::{auth0::Auth0, Authenticator};
use clap::Parser;
use cli::Args;
use state::{AppState, State};

mod app;
mod auth;
mod cli;
mod state;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("error loading environment variables");

    let args = Args::parse();
    dbg!(&args);

    let authenticator = Authenticator::Token(Box::new(
        Auth0::new(&args.auth0_tenant_uri, args.auth0_audiences)
            .await
            .expect("error initializing auth backend"),
    ));

    let state = AppState::new(State { authenticator });
    let router = app::routes(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888")
        .await
        .expect("could not bind to tcp listener");

    axum::serve(listener, router)
        .await
        .expect("failed to start app");
}
