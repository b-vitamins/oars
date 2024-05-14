use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{
    Affiliation, AuthorIds, CountByYear, DehydratedConcept, DehydratedInstitution, SummaryStats,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Author {
    affiliations: Option<Vec<Affiliation>>,
    cited_by_count: Option<i32>,
    counts_by_year: Option<Vec<CountByYear>>,
    created_date: Option<String>,
    display_name: Option<String>,
    display_name_alternatives: Option<Vec<String>>,
    id: Option<String>,
    ids: Option<AuthorIds>,
    last_known_institution: Option<DehydratedInstitution>,
    last_known_institutions: Option<Vec<DehydratedInstitution>>,
    orcid: Option<String>,
    summary_stats: Option<SummaryStats>,
    updated_date: Option<String>,
    works_api_url: Option<String>,
    works_count: Option<i32>,
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
    use crate::{entity_idempotence_sugarred, entity_idempotence_desugarred};

    #[test]
    fn test_author_idempotence_sugarred() {
        entity_idempotence_sugarred!(Author, "testdata/entities/author.json");
    }

    #[test]
    fn test_author_idempotence_desugarred() {
        entity_idempotence_desugarred!(Author, "testdata/entities/author.json");
    }
}
