use crate::entities::common::{Domain, Field, TopicIds, Subfield};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Topic {
    cited_by_count: i64,
    created_date: String,
    description: String,
    display_name: String,
    domain: Domain,
    field: Field,
    id: String,
    ids: TopicIds,
    keywords: Vec<String>,
    subfield: Subfield,
    updated_date: String,
    works_count: i32,
}