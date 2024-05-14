use crate::bakery::{Deflatable, Deflated, Deflation, Leavenable};
use crate::entities::common::{Domain, Field, Subfield, TopicIds};
use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Topic {
    cited_by_count: Option<i64>,
    created_date: Option<String>,
    description: Option<String>,
    display_name: Option<String>,
    domain: Option<Domain>,
    field: Option<Field>,
    id: Option<String>,
    ids: Option<TopicIds>,
    keywords: Option<Vec<String>>,
    #[serde(default)]
    siblings: Option<Vec<String>>,
    subfield: Option<Subfield>,
    updated_date: Option<String>,
    works_count: Option<i32>,
}

impl Topic {
    pub fn new() -> Self {
        Self::default()
    }

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
