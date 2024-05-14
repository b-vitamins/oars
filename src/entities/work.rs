use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{
    Apc, Authorship, Biblio, Concept, Grant, Keyword, Location, MeshTag, OpenAccess,
    PercentileYear, Sdg, Topic, WorkIds, WorkType, YearCount,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Work {
    abstract_inverted_index: Option<HashMap<String, Vec<usize>>>,
    authorships: Option<Vec<Authorship>>,
    apc_list: Option<Apc>,
    apc_paid: Option<Apc>,
    best_oa_location: Option<Location>,
    biblio: Option<Biblio>,
    cited_by_api_url: Option<String>,
    cited_by_count: Option<usize>,
    cited_by_percentile_year: Option<PercentileYear>,
    concepts: Option<Vec<Concept>>,
    corresponding_author_ids: Option<Vec<String>>,
    corresponding_institution_ids: Option<Vec<String>>,
    countries_distinct_count: Option<usize>,
    counts_by_year: Option<Vec<YearCount>>,
    created_date: Option<String>,
    display_name: Option<String>,
    doi: Option<String>,
    fulltext_origin: Option<String>,
    grants: Option<Vec<Grant>>,
    has_fulltext: Option<bool>,
    id: Option<String>,
    ids: Option<WorkIds>,
    indexed_in: Option<Vec<String>>,
    institutions_distinct_count: Option<usize>,
    is_paratext: Option<bool>,
    is_retracted: Option<bool>,
    keywords: Option<Vec<Keyword>>,
    language: Option<String>,
    locations: Option<Vec<Location>>,
    locations_count: Option<usize>,
    mesh: Option<Vec<MeshTag>>,
    ngrams_url: Option<String>,
    open_access: Option<OpenAccess>,
    primary_location: Option<Location>,
    primary_topic: Option<Topic>,
    publication_date: Option<String>,
    publication_year: Option<usize>,
    referenced_works: Option<Vec<String>>,
    referenced_works_count: Option<usize>,
    related_works: Option<Vec<String>>,
    sustainable_development_goals: Option<Vec<Sdg>>,
    title: Option<String>,
    topics: Option<Vec<Topic>>,
    #[serde(rename = "type")]
    work_type: Option<WorkType>,
    type_crossref: Option<String>,
    updated_date: Option<String>,
    #[serde(default)]
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
        crate::entity_idempotence_sugarred!(Work, "testdata/entities/work.json");
    }

    #[test]
    fn test_work_idempotence_desugarred() {
        crate::entity_idempotence_desugarred!(Work, "testdata/entities/work.json");
    }
}
