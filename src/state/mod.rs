use anyhow::Error;
use sendgrid::SGClient;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use tokio::task::JoinHandle;

use crate::services::{airtable::Airtable, auth::Authenticator, storage::Storage, workspace::WorkspaceClient};

type TaskMap = HashMap<String, Option<JoinHandle<()>>>;

pub struct State {
    pub authenticator: Box<dyn Authenticator>,
    pub workspace_client: Box<dyn WorkspaceClient>,
    pub airtable: Airtable,
    pub storage: Storage,
    pub tasks: Mutex<TaskMap>,
    pub mail: SGClient,
}

pub type AppState = Arc<State>;
