use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{
    ApcPrice, CountsByYear, DehydratedConcept, Society, SourceIds, SummaryStats,
};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    abbreviated_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alternate_titles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apc_prices: Option<Vec<ApcPrice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    apc_usd: Option<i32>,
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
    homepage_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_organization_lineage: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_organization_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<SourceIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_in_doaj: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_oa: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issn: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issn_l: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    societies: Option<Vec<Society>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary_stats: Option<SummaryStats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    source_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    works_api_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    works_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    use crate::{entity_idempotence_desugarred, entity_idempotence_sugarred};

    #[test]
    fn test_source_idempotence_sugarred() {
        entity_idempotence_sugarred!(Source, "testdata/source.json");
    }

    #[test]
    fn test_source_idempotence_desugarred() {
        entity_idempotence_desugarred!(Source, "testdata/source.json");
    }
}
