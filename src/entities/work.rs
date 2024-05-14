use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{
    Apc, Authorship, Biblio, Concept, Grant, Keyword, Location, MeshTag, OpenAccess,
    PercentileYear, Sdg, Topic, WorkIds, YearCount,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Work {
    #[serde(skip_serializing_if = "Option::is_none")]
    abstract_inverted_index: Option<HashMap<String, Vec<usize>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorships: Option<Vec<Authorship>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apc_list: Option<Apc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apc_paid: Option<Apc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    best_oa_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    biblio: Option<Biblio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cited_by_api_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cited_by_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cited_by_percentile_year: Option<PercentileYear>,
    #[serde(skip_serializing_if = "Option::is_none")]
    concepts: Option<Vec<Concept>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    corresponding_author_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    corresponding_institution_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    countries_distinct_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    counts_by_year: Option<Vec<YearCount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    doi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fulltext_origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grants: Option<Vec<Grant>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    has_fulltext: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<WorkIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indexed_in: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    institutions_distinct_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_paratext: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_retracted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords: Option<Vec<Keyword>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locations: Option<Vec<Location>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locations_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mesh: Option<Vec<MeshTag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ngrams_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_access: Option<OpenAccess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_topic: Option<Topic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publication_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publication_year: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    referenced_works: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    referenced_works_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_works: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sustainable_development_goals: Option<Vec<Sdg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topics: Option<Vec<Topic>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    work_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    type_crossref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<String>>,
}

impl Work {
    pub fn new() -> Work {
        Self::default()
    }
    pub fn leaven<L: Leavenable>(input: L) -> Result<Self, SerdeError> {
        L::leaven(input)
    }
}

impl Deflatable for Work {
    fn deflate(&self, format: Deflation) -> Result<Deflated, SerdeError> {
        match format {
            Deflation::ToString => serde_json::to_string(self).map(Deflated::String),
            Deflation::ToJsonValue => serde_json::to_value(self).map(Deflated::JsonValue),
            Deflation::ToByteArray => serde_json::to_vec(self).map(Deflated::ByteArray),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work_idempotence_sugarred() {
        crate::entity_idempotence_sugarred!(Work, "testdata/work.json");
    }

    #[test]
    fn test_work_idempotence_desugarred() {
        crate::entity_idempotence_desugarred!(Work, "testdata/work.json");
    }
}
