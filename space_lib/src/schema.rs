use crate::errors;

#[derive(Clone, Debug)]
pub(crate) enum DataType {
    String,
    Integer,
    Float,
    Bool,
    ArrayString,
    ArrayInteger,
    ArrayFloat,
    ArrayBool,
}
#[derive(Clone, Debug)]
pub(crate) struct Schema {
    pub name: String,
    pub configs: std::collections::HashMap<String, DataType>,
    raw_configs: std::collections::HashMap<String, String>,
    database_path: std::path::PathBuf,
    storage_path: std::path::PathBuf,
    path: std::path::PathBuf,
}

impl Schema {
    pub fn new(
        name: &str,
        raw_configs: std::collections::HashMap<String, String>,
        database_path: std::path::PathBuf,
    ) -> Result<Schema, errors::InputError> {
        let type_map = std::collections::HashMap::from([
            ("string", DataType::String),
            ("integer", DataType::Integer),
            ("float", DataType::Float),
            ("bool", DataType::Bool),
            ("a_string", DataType::ArrayString),
            ("a_integer", DataType::Integer),
            ("a_float", DataType::ArrayFloat),
            ("a_bool", DataType::ArrayBool),
        ]);
        let mut configs: std::collections::HashMap<String, DataType> =
            std::collections::HashMap::new();
        for (key, dstr) in raw_configs.iter() {
            let dtype = type_map.get(key.as_str());

            if dtype.is_none() {
                return Err(errors::InputError::new(&format!(
                    "invalid data type input for schema attribute {}: {}",
                    key, dstr
                )));
            }
            configs.insert(key.to_string(), dtype.unwrap().clone());
        }
        let path = database_path.join(format!("schemas/{}.space.json", name));
        let storage_path = database_path.join(format!("data/{}", name));
        Ok(Schema {
            name: name.to_string(),
            raw_configs,
            configs,
            database_path,
            path,
            storage_path,
        })
    }

    pub fn validate_input(
        &self,
        input: std::collections::HashMap<String, serde_json::Value>,
    ) -> bool {
        for (key, dtype) in self.configs.iter() {
            let entry = input.get(key);
            if entry.is_none() {
                return false;
            };
            let value = entry.unwrap();
            let result = match dtype {
                DataType::String => value.is_string(),
                DataType::Integer => value.is_i64(),
                DataType::Float => value.is_f64(),
                DataType::Bool => value.is_boolean(),
                _ => {
                    if !value.is_array() {
                        return false;
                    }
                    fn _verify_iter(dtype: DataType, val: &serde_json::Value) -> bool {
                        for item in val.as_array().unwrap() {
                            match dtype {
                                DataType::ArrayString => {
                                    if !val.is_string() {
                                        return false;
                                    }
                                }
                                DataType::ArrayInteger => {
                                    if !val.is_i64() {
                                        return false;
                                    }
                                }
                                DataType::ArrayFloat => {
                                    if !val.is_f64() {
                                        return false;
                                    }
                                }
                                DataType::ArrayBool => {
                                    if !val.is_boolean() {
                                        return false;
                                    }
                                }
                                _ => return false,
                            }
                        }
                        true
                    }

                    _verify_iter(dtype.clone(), value)
                }
            };
        }
        true
    }
}
