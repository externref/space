// MIT License

// Copyright (c) 2024 sarthak

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod fs_utils;
mod internals;

fn main() {
    let mut args = Vec::from_iter(std::env::args());
    let _exec_from = args.remove(0);
    let db_name = args.remove(0);
    let db = internals::database::Database::new(db_name);
    db.run();
}

#[cfg(test)]
mod tests {
    use crate::fs_utils;
    use crate::internals;

    fn rm_schema_if_exists() {
        let schema_file = fs_utils::get_database_dir(".space_tests".to_string())
            .join(".schemas/test_schema.space.json");
        let schema_dir =
            fs_utils::get_database_dir(".space_tests".to_string()).join("data/test_schema");
        if schema_file.exists() {
            std::fs::remove_file(schema_file).unwrap();
        }
        if schema_dir.exists() {
            std::fs::remove_dir_all(schema_dir).unwrap();
        }
    }

    #[test]
    fn create_schema() {
        rm_schema_if_exists();
        let db = internals::database::Database::new(".space_tests".to_string());
        let schema_config: std::collections::HashMap<String, String> =
            std::collections::HashMap::from([
                ("name".to_string(), "string".to_string()),
                ("marks".to_string(), "a_integer".to_string()),
            ]);
        let schema = internals::schema::Schema::new(
            db.path.join(".schemas/test_schema.space.json"),
            db.path,
            "test_schema".to_string(),
            schema_config.clone(),
        );
        schema.create();
        let saved_file_data: std::collections::HashMap<String, String> =
            serde_json::from_str(std::fs::read_to_string(schema.path).unwrap().as_str()).unwrap();
        assert_eq!(schema_config, saved_file_data);
    }

    #[test]
    fn test_input_validation() {
        let db: internals::database::Database =
            internals::database::Database::new(".space_tests".to_string());
        let schema: &internals::schema::Schema = db.schemas.get("test_schema").unwrap();
        // schema.create();
        let invalid_input: std::collections::HashMap<String, serde_json::Value> =
            serde_json::from_str(
                r#"{
                    "name": "sarthak",
                    "marks": [1, "hello"]
                }"#,
            )
            .unwrap();
        let valid_input: std::collections::HashMap<String, serde_json::Value> =
            serde_json::from_str(
                r#"{
                    "name": "not sarthak",
                    "marks": [1, 2, 4, 5]
                }"#,
            )
            .unwrap();
        assert_eq!(schema.validate_input(&valid_input), true);
        assert_eq!(schema.validate_input(&invalid_input), false);
    }
}
