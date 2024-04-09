use std::env;

use serde::{Deserialize, Serialize};

use crate::services::airtable::{Airtable, ListRecordsOptions};

#[tokio::test]
pub async fn test_list_records() {
    dotenvy::dotenv().expect("error loading environment variables");

    let airtable =
        Airtable::new(&env::var("AIRTABLE_API_TOKEN").expect("missing airtable api token"));

    let (airtable_base, airtable_table, airtable_view) = (
        env::var("TEST_AIRTABLE_BASE").expect("missing test airtable base env variable"),
        env::var("TEST_AIRTABLE_TABLE").expect("missing test airtable table variable"),
        env::var("TEST_AIRTABLE_VIEW").expect("missing test airtable view variable"),
    );

    dbg!(&airtable.api_token);

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct TestDataStruct {
        first_name: String,
        last_name: String,
        email: String,
    }

    let records = airtable
        .list_records::<TestDataStruct>(
            &airtable_base,
            &airtable_table,
            &ListRecordsOptions {
                view: Some(airtable_view),
                fields: Some(vec!["FirstName".into(), "LastName".into(), "Email".into()]),
            },
        )
        .await
        .expect("error fetching records");

    dbg!(&records);
}
