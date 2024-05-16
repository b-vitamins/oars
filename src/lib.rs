pub mod entities {
    pub mod author;
    pub mod common;
    pub mod funder;
    pub mod institution;
    pub mod publisher;
    pub mod source;
    pub mod topic;
    pub mod work;
}

pub mod bakery;
pub mod config;
pub mod traits;
pub mod client;
pub mod error;

use tracing_subscriber::fmt::Subscriber;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init_logging() {
    INIT.call_once(|| {
        Subscriber::builder().init();
    });
}

#[macro_export]
macro_rules! entity_idempotence_desugarred {
    ($entity:ty, $json_path:expr) => {{
        use assert_json_diff::assert_json_include;

        let original_json = std::fs::read_to_string($json_path)
            .expect("Failed to read original JSON file");

        let entity: $entity = serde_json::from_str(&original_json)
            .expect("Failed to deserialize the original JSON");

        let serialized_json: String = serde_json::to_string(&entity)
            .expect("Failed to serialize the entity instance back to JSON");

        // Create a temporary file for comparison
        let mut temp_file = tempfile::NamedTempFile::new()
            .expect("Failed to create a temporary file");

        // Write the serialized JSON to the temporary file
        std::io::Write::write_all(&mut temp_file, serialized_json.as_bytes())
            .expect("Failed to write serialized JSON to temporary file");

        // Read back the serialized JSON from the temporary file
        let serialized_from_temp = std::fs::read_to_string(temp_file.path())
            .expect("Failed to read back the serialized JSON");

        let original_value: serde_json::Value = serde_json::from_str(&original_json)
            .expect("Failed to parse the original JSON into Value");
        let serialized_value: serde_json::Value = serde_json::from_str(&serialized_from_temp)
            .expect("Failed to parse the serialized JSON from temp file into Value");

        assert_json_include!(actual: original_value, expected: serialized_value);
    }};
}

#[macro_export]
macro_rules! entity_idempotence_sugarred {
    ($entity:ty, $json_path:expr) => {{
        use assert_json_diff::assert_json_include;

        let pristine_json = std::fs::read_to_string($json_path)
            .expect("Failed to read original JSON file");

        let leavened_entity: $entity = <$entity>::leaven(std::path::PathBuf::from($json_path))
            .expect("Failed to leaven the entity");

        let deflated_entity_str: Option<String> = leavened_entity
            .deflate($crate::bakery::Deflation::ToString)
            .expect("Failed to deflate entity into JSON String")
            .to_string();

        if let Some(deflated_entity) = deflated_entity_str {
            let original_value: serde_json::Value = serde_json::from_str(&pristine_json)
                .expect("Failed to parse original JSON string into Value");
            let serialized_value: serde_json::Value = serde_json::from_str(&deflated_entity)
                .expect("Failed to parse deflated JSON string into Value");

            assert_json_include!(actual: original_value, expected: serialized_value);
        } else {
            panic!("Deflation to string failed");
        }
    }};
}

#[macro_export]
macro_rules! check_entity_against_json {
    ($entity:ty, $json_path:expr, $entity_instance:expr) => {{
        use assert_json_diff::assert_json_include;

        // Read the expected JSON file
        let expected_json = std::fs::read_to_string($json_path)
            .expect("Failed to read expected JSON file");

        // Deserialize the expected JSON into the entity type
        let expected_entity: $entity = serde_json::from_str(&expected_json)
            .expect("Failed to deserialize the expected JSON");

        // Serialize both the expected entity and the in-memory entity
        let expected_serialized = serde_json::to_string(&expected_entity)
            .expect("Failed to serialize the expected entity");
        let actual_serialized = serde_json::to_string(&$entity_instance)
            .expect("Failed to serialize the in-memory entity");

        // Parse the serialized entities into serde_json::Value for comparison
        let expected_value: serde_json::Value = serde_json::from_str(&expected_serialized)
            .expect("Failed to parse the expected serialized JSON into Value");
        let actual_value: serde_json::Value = serde_json::from_str(&actual_serialized)
            .expect("Failed to parse the actual serialized JSON into Value");

        // Compare the actual entity with the expected entity
        assert_json_include!(actual: actual_value, expected: expected_value);
    }};
}

