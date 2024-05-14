use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{CountsByYear, FunderIds, Role, SummaryStats};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Funder {
    #[serde(skip_serializing_if = "Option::is_none")]
    alternate_titles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cited_by_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    counts_by_year: Option<Vec<CountsByYear>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grants_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    homepage_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ids: Option<FunderIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<Vec<Role>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary_stats: Option<SummaryStats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    works_count: Option<i32>,
}

impl Funder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn leaven<L: Leavenable>(input: L) -> Result<Self, SerdeError> {
        L::leaven(input)
    }
}

impl Deflatable for Funder {
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
    fn test_funder_idempotence_sugarred() {
        entity_idempotence_sugarred!(Funder, "testdata/funder.json");
    }

    #[test]
    fn test_funder_idempotence_desugarred() {
        entity_idempotence_desugarred!(Funder, "testdata/funder.json");
    }
}
