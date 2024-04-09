use std::sync::Arc;

use crate::auth::Authenticator;

pub struct State {
    pub authenticator: Authenticator,
}

pub type AppState = Arc<State>;
