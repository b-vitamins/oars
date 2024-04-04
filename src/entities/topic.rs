use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{Domain, Field, Subfield, TopicIds};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Topic {
    cited_by_count: i64,
    created_date: String,
    description: String,
    display_name: String,
    domain: Domain,
    field: Field,
    id: String,
    ids: TopicIds,
    keywords: Vec<String>,
    #[serde(default)]
    siblings: Option<Vec<String>>,
    subfield: Subfield,
    updated_date: String,
    works_count: i32,
}

impl Topic {
    pub fn leaven<L: Leavenable>(input: L) -> Result<Self, SerdeError> {
        L::leaven(input)
    }
}

impl Deflatable for Topic {
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
    fn test_author_idempotence_sugarred() {
        crate::entity_idempotence_sugarred!(Topic, "testdata/entities/topic.json");
    }

    #[test]
    fn test_author_idempotence_desugarred() {
        crate::entity_idempotence_desugarred!(Topic, "testdata/entities/topic.json");
    }
}
