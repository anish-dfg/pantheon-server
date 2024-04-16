use std::sync::Arc;

use crate::services::{
    airtable::Airtable,
    auth::Authenticator,
    storage::{Cache, Sql},
    workspace::WorkspaceClient,
};

pub struct State {
    pub authenticator: Box<dyn Authenticator>,
    pub workspace_client: Box<dyn WorkspaceClient>,
    pub airtable: Airtable,
    pub sql: Sql,
    pub cache: Cache,
}

pub type AppState = Arc<State>;
