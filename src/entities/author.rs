use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{
    Affiliation, AuthorIds, CountByYear, DehydratedConcept, DehydratedInstitution, SummaryStats,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    affiliations: Vec<Affiliation>,
    cited_by_count: i32,
    counts_by_year: Vec<CountByYear>,
    created_date: String,
    display_name: String,
    display_name_alternatives: Vec<String>,
    id: String,
    ids: AuthorIds,
    last_known_institution: Option<DehydratedInstitution>,
    last_known_institutions: Vec<DehydratedInstitution>,
    orcid: Option<String>,
    summary_stats: SummaryStats,
    updated_date: String,
    works_api_url: String,
    works_count: i32,
    x_concepts: Vec<DehydratedConcept>,
}

impl Author {
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

    #[test]
    fn test_author_idempotence_sugarred() {
        crate::entity_idempotence_sugarred!(Author, "testdata/entities/author.json");
    }

    #[test]
    fn test_author_idempotence_desugarred() {
        crate::entity_idempotence_desugarred!(Author, "testdata/entities/author.json");
    }
}
