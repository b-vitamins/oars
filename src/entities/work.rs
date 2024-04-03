use crate::entities::common::{Authorship, Apc, Location, Biblio, YearCount, Concept, Grant, Keyword, MeshTag, WorkIds, OpenAccess, License, Sdg, Topic, WorkType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Work {
    abstract_inverted_index: Option<HashMap<String, Vec<usize>>>,
    authorships: Vec<Authorship>,
    apc_list: Option<Apc>,
    apc_paid: Option<Apc>,
    best_oa_location: Option<Location>,
    biblio: Option<Biblio>,
    cited_by_api_url: String,
    cited_by_count: usize,
    concepts: Vec<Concept>,
    corresponding_author_ids: Vec<String>,
    corresponding_institution_ids: Vec<String>,
    countries_distinct_count: usize,
    counts_by_year: Vec<YearCount>,
    created_date: Option<String>,
    display_name: String,
    doi: Option<String>,
    fulltext_origin: Option<String>,
    grants: Vec<Grant>,
    has_fulltext: bool,
    id: String,
    ids: WorkIds,
    indexed_in: Vec<String>,
    institutions_distinct_count: usize,
    is_paratext: bool,
    is_retracted: bool,
    keywords: Option<Vec<Keyword>>,
    language: Option<String>,
    license: Option<License>,
    locations: Vec<Location>,
    locations_count: usize,
    mesh: Option<Vec<MeshTag>>,
    ngrams_url: Option<String>,
    open_access: OpenAccess,
    primary_location: Option<Location>,
    primary_topic: Option<Topic>,
    publication_date: Option<String>,
    publication_year: Option<usize>,
    referenced_works: Option<Vec<String>>,
    related_works: Option<Vec<String>>,
    sustainable_development_goals: Option<Vec<Sdg>>,
    topics: Option<Vec<Topic>>,
    title: String,
    #[serde(rename = "type")]
    work_type: WorkType,
    type_crossref: Option<String>,
    updated_date: Option<String>,
}
