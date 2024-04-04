use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{
    ApcPrice, CountsByYear, DehydratedConcept, Society, SourceIds, SourceType, SummaryStats,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    abbreviated_title: Option<String>,
    alternate_titles: Vec<String>,
    apc_prices: Vec<ApcPrice>,
    apc_usd: Option<i32>,
    cited_by_count: i64,
    country_code: String,
    counts_by_year: Vec<CountsByYear>,
    created_date: String,
    display_name: String,
    homepage_url: String,
    host_organization: String,
    host_organization_lineage: Vec<String>,
    host_organization_name: String,
    id: String,
    ids: SourceIds,
    is_in_doaj: bool,
    is_oa: bool,
    issn: Vec<String>,
    issn_l: String,
    societies: Vec<Society>,
    summary_stats: SummaryStats,
    #[serde(rename = "type")]
    source_type: SourceType,
    updated_date: String,
    works_api_url: String,
    works_count: i32,
    x_concepts: Vec<DehydratedConcept>,
}

impl Source {
    pub fn leaven<L: Leavenable>(input: L) -> Result<Self, SerdeError> {
        L::leaven(input)
    }
}

impl Deflatable for Source {
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

    // Source tests
    #[test]
    fn test_source_idempotence_sugarred() {
        crate::entity_idempotence_sugarred!(Source, "testdata/entities/source.json");
    }

    #[test]
    fn test_source_idempotence_desugarred() {
        crate::entity_idempotence_desugarred!(Source, "testdata/entities/source.json");
    }

}
