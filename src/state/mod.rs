use std::sync::Arc;

use crate::services::{auth::Authenticator, workspace::WorkspaceClient};

pub struct State {
    pub authenticator: Authenticator,
    pub workspace_client: Box<dyn WorkspaceClient>,
}

pub type AppState = Arc<State>;
