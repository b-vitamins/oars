use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{
    CountsByYear, DehydratedConcept, DehydratedInstitutionWithRelationship, Geo, InstitutionIds,
    InternationalDisplayNames, Repository, Role, SummaryStats,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Institution {
    #[serde(skip_serializing_if = "Option::is_none")]
    associated_institutions: Option<Vec<DehydratedInstitutionWithRelationship>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cited_by_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    counts_by_year: Option<Vec<CountsByYear>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name_acronyms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name_alternatives: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    geo: Option<Geo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    homepage_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<InstitutionIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    international: Option<InternationalDisplayNames>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lineage: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repositories: Option<Vec<Repository>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<Vec<Role>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ror: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary_stats: Option<SummaryStats>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    institution_type: Option<String>,
    #[serde(rename = "type_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    institution_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    works_api_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    works_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
        crate::entity_idempotence_sugarred!(Institution, "testdata/institution.json");
    }

    #[test]
    fn test_institution_idempotence_desugarred() {
        crate::entity_idempotence_desugarred!(Institution, "testdata/institution.json");
    }
}
