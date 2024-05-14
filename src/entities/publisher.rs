use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{CountByYear, PublisherIds, Role, SummaryStats};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Publisher {
    alternate_titles: Option<Vec<String>>,
    cited_by_count: Option<i64>,
    country_codes: Option<Vec<String>>,
    counts_by_year: Option<Vec<CountByYear>>,
    created_date: Option<String>,
    display_name: Option<String>,
    hierarchy_level: Option<i32>,
    homepage_url: Option<Option<String>>,
    id: Option<String>,
    ids: Option<PublisherIds>,
    image_thumbnail_url: Option<String>,
    image_url: Option<String>,
    lineage: Option<Vec<String>>,
    parent_publisher: Option<Option<String>>,
    roles: Option<Vec<Role>>,
    sources_api_url: Option<String>,
    summary_stats: Option<SummaryStats>,
    updated_date: Option<String>,
    works_count: Option<i32>,
}

impl Publisher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn leaven<L: Leavenable>(input: L) -> Result<Self, SerdeError> {
        L::leaven(input)
    }
}

impl Deflatable for Publisher {
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
    fn test_publisher_idempotence_sugarred() {
        crate::entity_idempotence_sugarred!(Publisher, "testdata/entities/publisher.json");
    }

    #[test]
    fn test_publisher_idempotence_desugarred() {
        crate::entity_idempotence_desugarred!(Publisher, "testdata/entities/publisher.json");
    }
}
