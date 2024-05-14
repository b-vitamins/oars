use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{
    CountsByYear, DehydratedConcept, DehydratedInstitutionWithRelationship, Geo, InstitutionIds,
    InternationalDisplayNames, Repository, Role, SummaryStats,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Institution {
    associated_institutions: Option<Vec<DehydratedInstitutionWithRelationship>>,
    cited_by_count: Option<i64>,
    country_code: Option<String>,
    counts_by_year: Option<Vec<CountsByYear>>,
    created_date: Option<String>,
    display_name: Option<String>,
    display_name_acronyms: Option<Vec<String>>,
    display_name_alternatives: Option<Vec<String>>,
    geo: Option<Geo>,
    homepage_url: Option<String>,
    id: Option<String>,
    ids: Option<InstitutionIds>,
    image_thumbnail_url: Option<String>,
    image_url: Option<String>,
    international: Option<InternationalDisplayNames>,
    lineage: Option<Vec<String>>,
    repositories: Option<Vec<Repository>>,
    roles: Option<Vec<Role>>,
    ror: Option<String>,
    summary_stats: Option<SummaryStats>,
    #[serde(rename = "type")]
    institution_type: Option<String>,
    #[serde(rename = "type_id")]
    institution_type_id: Option<String>,
    updated_date: Option<String>,
    works_api_url: Option<String>,
    works_count: Option<i32>,
    x_concepts: Option<Vec<DehydratedConcept>>,
}

impl Institution {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn leaven<L: Leavenable>(input: L) -> Result<Self, SerdeError> {
        L::leaven(input)
    }
}

impl Deflatable for Institution {
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
    fn test_institution_idempotence_sugarred() {
        crate::entity_idempotence_sugarred!(Institution, "testdata/entities/institution.json");
    }

    #[test]
    fn test_institution_idempotence_desugarred() {
        crate::entity_idempotence_desugarred!(Institution, "testdata/entities/institution.json");
    }
}
