use airtable_api::{Airtable, Record};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serenity::model::prelude::User;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Score {
    pub id: String,
    pub score: isize,
}

pub async fn get_scores() -> Result<Vec<Score>> {
    let airtable_api_key = std::env::var("AIRTABLE_API_KEY").unwrap();
    let airtable_table_id = std::env::var("AIRTABLE_TABLE_ID").unwrap();
    let airtable = Airtable::new(airtable_api_key, airtable_table_id, "");
    let records: Vec<Record<Score>> = airtable
        .list_records("Test", "Grid view", vec!["Id", "Score"])
        .await?;

    Ok(records.iter().map(|record| record.fields.clone()).collect())
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CompleteScore {
    pub id: String,
    pub name: String,
    pub score: isize,
}

pub async fn update_score(user: &User, update: impl Fn(isize) -> isize) -> Result<isize> {
    let airtable_api_key = std::env::var("AIRTABLE_API_KEY").unwrap();
    let airtable_table_id = std::env::var("AIRTABLE_TABLE_ID").unwrap();
    let airtable = Airtable::new(airtable_api_key, airtable_table_id, "");
    let records: Vec<Record<CompleteScore>> = airtable
        .list_records("Test", "Grid view", vec!["Id", "Score", "Name"])
        .await?;
    let record = records
        .into_iter()
        .find(|record| record.fields.id == user.id.to_string());

    if let Some(mut record) = record {
        record.fields.name = user.name.clone();
        record.fields.score = update(record.fields.score);
        let res = record.fields.score;
        airtable.update_records("Test", vec![record]).await?;
        Ok(res)
    } else {
        let record = Record {
            id: "".to_string(),
            fields: CompleteScore {
                id: user.id.to_string(),
                name: user.name.clone(),
                score: update(0),
            },
            created_time: None,
        };
        let res = record.fields.score;
        airtable.create_records("Test", vec![record]).await?;
        Ok(res)
    }
}
