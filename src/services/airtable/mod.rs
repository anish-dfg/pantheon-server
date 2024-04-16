pub mod bases;
pub mod record;
pub mod schema;

#[cfg(test)]
mod tests;

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use self::{bases::Bases, record::Record, schema::Schema};

pub struct Airtable {
    pub http: Client,
    pub api_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListRecordsOptions {
    pub fields: Option<Vec<String>>,
    pub view: Option<String>,
    pub offset: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListRecordsResponse<T> {
    pub records: Vec<Record<T>>,
    pub offset: Option<String>,
}

impl Airtable {
    const V0_BASE_URI: &'static str = "https://api.airtable.com/v0";

    pub fn new(api_token: &str) -> Self {
        Self {
            http: Client::new(),
            api_token: api_token.to_owned(),
        }
    }

    pub async fn list_bases(&self) -> Result<Bases> {
        let http = &self.http;
        let uri = format!("{}{}", Self::V0_BASE_URI, "/meta/bases");

        let res = http
            .get(uri)
            .header("Authorization", format!("Bearer {}", &self.api_token))
            .send()
            .await
            .context("fetch airtable bases")?;

        let bases = res.json::<Bases>().await.context("deserialize airtable bases")?;

        Ok(bases)
    }

    pub async fn fetch_schema(&self, base_id: &str) -> Result<Schema> {
        let http = &self.http;

        let uri = format!("{}/meta/bases/{}/tables", Self::V0_BASE_URI, base_id);

        let res = http
            .get(uri)
            .header("Authorization", format!("Bearer {}", &self.api_token))
            .send()
            .await
            .context("error fetching airtable base schema")?;

        let schema = res
            .json::<Schema>()
            .await
            .context("error deserializing airtable base schema")?;
        Ok(schema)
    }

    pub async fn list_records<T>(
        &self,
        base_id: &str,
        table_id_or_name: &str,
        opts: &ListRecordsOptions,
    ) -> Result<ListRecordsResponse<T>>
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

        if let Some(ref offset) = opts.offset {
            uri = uri + "&offset=" + offset;
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
            .context("deserialize airtable records")?;

        Ok(data)
    }
}
