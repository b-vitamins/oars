use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{CountByYear, PublisherIds, Role, SummaryStats};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Publisher {
    alternate_titles: Vec<String>,
    cited_by_count: i64,
    country_codes: Vec<String>,
    counts_by_year: Vec<CountByYear>,
    created_date: String,
    display_name: String,
    hierarchy_level: i32,
    homepage_url: Option<String>,
    id: String,
    ids: PublisherIds,
    image_thumbnail_url: String,
    image_url: String,
    lineage: Vec<String>,
    parent_publisher: Option<String>,
    roles: Vec<Role>,
    sources_api_url: String,
    summary_stats: SummaryStats,
    updated_date: String,
    works_count: i32,
}

impl Publisher {
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
