pub mod bases;
pub mod record;
pub mod schema;

#[cfg(test)]
mod tests;

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use self::record::Record;

pub struct Airtable {
    pub http: Client,
    pub api_token: String,
}

pub struct ListRecordsOptions {
    pub fields: Option<Vec<String>>,
    pub view: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListRecordsResponse<T> {
    pub records: Vec<Record<T>>,
}

impl Airtable {
    const V0_BASE_URI: &'static str = "https://api.airtable.com/v0";

    pub fn new(api_token: &str) -> Self {
        Self {
            http: Client::new(),
            api_token: api_token.to_owned(),
        }
    }

    pub async fn list_records<T>(
        &self,
        base_id: &str,
        table_id_or_name: &str,
        opts: &ListRecordsOptions,
    ) -> Result<Vec<Record<T>>>
    where
        T: DeserializeOwned,
    {
        let http = &self.http;
        let mut uri = format!("{}/{}/{}?", Self::V0_BASE_URI, base_id, table_id_or_name);

        let fields_query = opts.fields.as_ref().map(|fields| {
            fields
                .iter()
                .map(|f| "&fields%5B%5D=".to_owned() + &f)
                .fold("".to_owned(), |acc, el| acc + &el)
        });

        let auth_header = "Bearer ".to_owned() + &self.api_token;

        if let Some(ref query) = fields_query {
            uri = uri + query;
        };

        if let Some(ref view) = opts.view {
            uri = uri + "&view=" + view;
        };

        let res = http
            .get(uri)
            .header("Authorization", auth_header)
            .send()
            .await
            .context("list airtable records")?;

        let data = res
            .json::<ListRecordsResponse<T>>()
            .await
            .expect("deserialize airtable records");

        Ok(data.records)
    }
}
