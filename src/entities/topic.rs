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

#[derive(Serialize, Deserialize, Debug)]
pub struct Domain {
    id: i32,
    display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    id: i32,
    display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subfield {
    id: i32,
    display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopicIds {
    openalex: String,
    wikipedia: Option<String>,
}