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
