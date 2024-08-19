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

#[derive(Clone, Debug)]
pub enum DataType {
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
pub struct Schema {
    pub name: String,
    pub path: std::path::PathBuf,
    database_path: std::path::PathBuf,
    raw_configs: std::collections::HashMap<String, String>,
    pub configs: std::collections::HashMap<String, DataType>,
}

impl Schema {
    fn generate_configs(
        raw_configs: &std::collections::HashMap<String, String>,
    ) -> std::collections::HashMap<String, DataType> {
        // let type_map = std::collections::HashMap::from([
        //     ("string", DataType::String),
        //     ("integer", DataType::Integer),
        //     ("float", DataType::Float),
        //     ("bool", DataType::Bool),
        //     ("a_string", DataType::ArrayString),
        //     ("a_integer", DataType::Integer),
        //     ("a_float", DataType::ArrayFloat),
        //     ("a_bool", DataType::ArrayBool),
        // ]);
        let mut configs: std::collections::HashMap<String, DataType> =
            std::collections::HashMap::new();
        for (attr, val) in raw_configs.iter() {
            let valid = [
                "string",
                "integer",
                "float",
                "bool",
                "a_string",
                "a_integer",
                "a_float",
                "a_bool",
            ];
            if !valid.contains(&val.as_str()) {
                panic!("Invalid data type in schema: \"{}\"", val)
            }
            if val == "string" {
                configs.insert(attr.to_string(), DataType::String);
            } else if val == "integer" {
                configs.insert(attr.to_string(), DataType::Integer);
            } else if val == "float" {
                configs.insert(attr.to_string(), DataType::Float);
            } else if val == "bool" {
                configs.insert(attr.to_string(), DataType::Bool);
            } else if val == "a_string" {
                configs.insert(attr.to_string(), DataType::ArrayString);
            } else if val == "a_integer" {
                configs.insert(attr.to_string(), DataType::ArrayInteger);
            } else if val == "a_float" {
                configs.insert(attr.to_string(), DataType::ArrayFloat);
            } else if val == "a_bool" {
                configs.insert(attr.to_string(), DataType::ArrayBool);
            }
        }
        return configs;
    }

    pub fn new(
        path: std::path::PathBuf,
        database_path: std::path::PathBuf,
        name: String,
        raw_configs: std::collections::HashMap<String, String>,
    ) -> Schema {
        let configs = Schema::generate_configs(&raw_configs);
        return Schema {
            path,
            database_path,
            name,
            raw_configs,
            configs,
        };
    }
    pub fn validate_input(
        &self,
        input: &std::collections::HashMap<String, serde_json::Value>,
    ) -> bool {
        for (key, dtype) in &self.configs {
            let value = input.get(key);

            let is_type_correct = match dtype {
                DataType::String => value.unwrap().is_string(),
                DataType::Integer => value.unwrap().is_i64(),
                DataType::Float => value.unwrap().is_f64(),
                DataType::Bool => value.unwrap().is_boolean(),

                _ => {
                    let val = value.unwrap();
                    if !val.is_array() {
                        return false;
                    }
                    let internal_type_correct = match dtype {
                        DataType::ArrayString => {
                            for item in val.as_array().unwrap() {
                                if !item.is_string() {
                                    return false;
                                }
                            }
                            true
                        }
                        DataType::ArrayInteger => {
                            for item in val.as_array().unwrap() {
                                if !item.is_i64() {
                                    println!(" breaks here");
                                    return false;
                                }
                            }
                            true
                        }
                        DataType::ArrayFloat => {
                            for item in val.as_array().unwrap() {
                                if !item.is_f64() {
                                    return false;
                                }
                            }
                            true
                        }
                        DataType::ArrayBool => {
                            for item in val.as_array().unwrap() {
                                if !item.is_boolean() {
                                    return false;
                                }
                            }
                            true
                        }
                        _ => false,
                    };
                    internal_type_correct
                }
            };
            if !is_type_correct {
                return false;
            }
        }
        return true;
    }

    pub fn write(&self, id: String, data: std::collections::HashMap<String, serde_json::Value>) {
        if !self.validate_input(&data) {
            panic!("invalid data input");
        }
        std::fs::write(
            self.database_path
                .join(format!("data/{}/{}.space.json", self.name, id)),
            serde_json::to_string_pretty(&data).unwrap(),
        )
        .unwrap();
    }

    pub fn create(&self) {
        // add schema files under schemas folder
        let json_data = serde_json::to_string_pretty(&self.raw_configs).unwrap();
        std::fs::write(self.path.clone(), json_data).unwrap();
        // create folder for schema data
        std::fs::create_dir(self.database_path.join(format!("data/{}", self.name))).unwrap();
    }
}
