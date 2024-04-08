use state::{AppState, State};

mod app;
mod auth;
mod state;

#[tokio::main]
async fn main() {
    let state = AppState::new(State {});
    let router = app::routes(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888")
        .await
        .expect("could not bind to tcp listener");

    axum::serve(listener, router)
        .await
        .expect("failed to start app");
}
