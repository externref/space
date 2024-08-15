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

use super::database::Database;

enum DataType {
    String,
    Integer,
    Float,
    Bool,
    ArrayString,
    ArrayInteger,
    ArrayFloat,
    ArrayBool,
}

pub(crate) struct Schema {
    database: Database,
    name: String,
    raw_configs: std::collections::HashMap<String, String>,
    configs: std::collections::HashMap<String, DataType>,
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
        self,
        database:Database,
        name: String,
        raw_configs: std::collections::HashMap<String, String>,
    ) -> Schema {
        let configs = Schema::generate_configs(&raw_configs);
        return Schema {
            database,
            name,
            raw_configs,
            configs,
        };
    }

    pub fn create(database: Database, name: String, configs: std::collections::HashMap<String, String> ) -> Schema{
        todo!()
    }
}
