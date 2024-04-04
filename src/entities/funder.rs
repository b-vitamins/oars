use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{CountsByYear, FunderIds, Role, SummaryStats};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Funder {
    alternate_titles: Vec<String>,
    cited_by_count: i64,
    country_code: String,
    counts_by_year: Vec<CountsByYear>,
    created_date: String,
    description: String,
    display_name: String,
    grants_count: i32,
    homepage_url: String,
    id: String,
    ids: FunderIds,
    image_thumbnail_url: String,
    image_url: String,
    roles: Vec<Role>,
    summary_stats: SummaryStats,
    updated_date: String,
    works_count: i32,
}

impl Funder {
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

    #[test]
    fn test_funder_idempotence_sugarred() {
        crate::entity_idempotence_sugarred!(Funder, "testdata/entities/funder.json");
    }

    #[test]
    fn test_funder_idempotence_desugarred() {
        crate::entity_idempotence_desugarred!(Funder, "testdata/entities/funder.json");
    }
}
