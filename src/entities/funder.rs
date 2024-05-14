use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{CountsByYear, FunderIds, Role, SummaryStats};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Funder {
    alternate_titles: Option<Vec<String>>,
    cited_by_count: Option<i64>,
    country_code: Option<String>,
    counts_by_year: Option<Vec<CountsByYear>>,
    created_date: Option<String>,
    description: Option<String>,
    display_name: Option<String>,
    grants_count: Option<i32>,
    homepage_url: Option<String>,
    id: Option<String>,
    ids: Option<FunderIds>,
    image_thumbnail_url: Option<String>,
    image_url: Option<String>,
    roles: Option<Vec<Role>>,
    summary_stats: Option<SummaryStats>,
    updated_date: Option<String>,
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
    use crate::{entity_idempotence_sugarred, entity_idempotence_desugarred};

    #[test]
    fn test_funder_idempotence_sugarred() {
        entity_idempotence_sugarred!(Funder, "testdata/entities/funder.json");
    }

    #[test]
    fn test_funder_idempotence_desugarred() {
        entity_idempotence_desugarred!(Funder, "testdata/entities/funder.json");
    }
}
