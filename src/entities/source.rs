use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{
    ApcPrice, CountsByYear, DehydratedConcept, Society, SourceIds, SourceType, SummaryStats,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Source {
    abbreviated_title: Option<String>,
    alternate_titles: Option<Vec<String>>,
    apc_prices: Option<Vec<ApcPrice>>,
    apc_usd: Option<i32>,
    cited_by_count: Option<i64>,
    country_code: Option<String>,
    counts_by_year: Option<Vec<CountsByYear>>,
    created_date: Option<String>,
    display_name: Option<String>,
    homepage_url: Option<String>,
    host_organization: Option<String>,
    host_organization_lineage: Option<Vec<String>>,
    host_organization_name: Option<String>,
    id: Option<String>,
    ids: Option<SourceIds>,
    is_in_doaj: Option<bool>,
    is_oa: Option<bool>,
    issn: Option<Vec<String>>,
    issn_l: Option<String>,
    societies: Option<Vec<Society>>,
    summary_stats: Option<SummaryStats>,
    #[serde(rename = "type")]
    source_type: Option<SourceType>,
    updated_date: Option<String>,
    works_api_url: Option<String>,
    works_count: Option<i32>,
    x_concepts: Option<Vec<DehydratedConcept>>,
}

impl Source {
    pub fn new() -> Self {
        Self::default()
    }

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
    use crate::{entity_idempotence_sugarred, entity_idempotence_desugarred};

    #[test]
    fn test_source_idempotence_sugarred() {
        entity_idempotence_sugarred!(Source, "testdata/entities/source.json");
    }

    #[test]
    fn test_source_idempotence_desugarred() {
        entity_idempotence_desugarred!(Source, "testdata/entities/source.json");
    }
}
