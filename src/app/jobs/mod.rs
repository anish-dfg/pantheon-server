use std::time::Duration;

use anyhow::Result;
use rand::Rng;
use serde_json::{Map, Value};
use uuid::Uuid;

use crate::{services::airtable::ListRecordsOptionsBuilder, state::AppState};

// pub struct FetchAirtableDataParams {
//
// }

pub async fn fetch_and_cache_airtable_data(
    state: AppState,
    job_id: Uuid,
    new_datasource_view_id: String,
    base: String,
    table: String,
    view: String,
    fields: Vec<String>,
    offset: Option<String>,
) -> Result<()> {
    let (airtable, db, cache) = (&state.airtable, &state.storage.db, &state.storage.cache);

    let Ok(mut opts) = ListRecordsOptionsBuilder::default()
        .view(view)
        .fields(fields)
        .offset(offset)
        .build()
    else {
        db.mark_job_errored(job_id).await?;
        return Ok(());
    };

    let records = airtable.list_all_records::<Value>(&base, &table, &mut opts).await?;

    cache.set_json(&new_datasource_view_id, records).await?;

    Ok(())
}
