use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{
    Affiliation, AuthorIds, CountByYear, DehydratedConcept, DehydratedInstitution, SummaryStats,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Author {
    #[serde(skip_serializing_if = "Option::is_none")]
    affiliations: Option<Vec<Affiliation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cited_by_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    counts_by_year: Option<Vec<CountByYear>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name_alternatives: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<AuthorIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_known_institution: Option<DehydratedInstitution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_known_institutions: Option<Vec<DehydratedInstitution>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orcid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary_stats: Option<SummaryStats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    works_api_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    works_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x_concepts: Option<Vec<DehydratedConcept>>,
}

impl Author {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn leaven<L: Leavenable>(input: L) -> Result<Self, SerdeError> {
        L::leaven(input)
    }
}

impl Deflatable for Author {
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
    use crate::{entity_idempotence_desugarred, entity_idempotence_sugarred};

    #[test]
    fn test_author_idempotence_sugarred() {
        entity_idempotence_sugarred!(Author, "testdata/author.json");
    }

    #[test]
    fn test_author_idempotence_desugarred() {
        entity_idempotence_desugarred!(Author, "testdata/author.json");
    }
}
