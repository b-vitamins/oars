use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{
    CountsByYear, DehydratedConcept, DehydratedInstitutionWithRelationship, Geo, InstitutionIds,
    InternationalDisplayNames, Repository, Role, SummaryStats,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Institution {
    associated_institutions: Vec<DehydratedInstitutionWithRelationship>,
    cited_by_count: i64,
    country_code: String,
    counts_by_year: Vec<CountsByYear>,
    created_date: String,
    display_name: String,
    display_name_acronyms: Vec<String>,
    display_name_alternatives: Vec<String>,
    geo: Geo,
    homepage_url: String,
    id: String,
    ids: InstitutionIds,
    image_thumbnail_url: String,
    image_url: String,
    international: InternationalDisplayNames,
    lineage: Vec<String>,
    repositories: Vec<Repository>,
    roles: Vec<Role>,
    ror: String,
    summary_stats: SummaryStats,
    #[serde(rename = "type")]
    institution_type: Option<String>,
    #[serde(rename = "type_id")]
    institution_type_id: Option<String>,
    updated_date: String,
    works_api_url: String,
    works_count: i32,
    x_concepts: Vec<DehydratedConcept>,
}

impl Institution {
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
