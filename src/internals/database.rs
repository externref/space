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

use std::io::Write;

use crate::fs_utils;
use crate::internals::schema;

#[derive(Clone)]
pub struct CommandHandler {
    db_path: std::path::PathBuf,
}

impl CommandHandler {
    fn create_schema_command(&self, mut args: Vec<&str>) {
        let schema_name = args.get(0).unwrap().to_string();
        args.remove(0);
        let schema_data: std::collections::HashMap<String, String> =
            serde_json::from_str(args.join(" ").as_str()).unwrap();
        let mut path = self.db_path.clone();
        path.push(format!(".schemas/{}.space.json", schema_name.clone()));
        let schema = schema::Schema::new(path, self.db_path.clone(), schema_name, schema_data);
        schema.create();
    }

    #[allow(dead_code, unreachable_code, unused)]
    fn interactive_write(&self, schema: &schema::Schema) {
        todo!();
        let mut data: std::collections::HashMap<String, serde_json::Value> =
            std::collections::HashMap::new();
        for (name, dtype) in schema.configs.iter() {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let convr: serde_json::Value = serde_json::from_str(&input).unwrap();
            data.insert(name.to_string(), convr);
        }
    }
}
#[derive(Clone)]
pub struct Database {
    pub name: String,
    pub schemas: std::collections::HashMap<String, schema::Schema>,
    pub path: std::path::PathBuf,
    pub command_handler: CommandHandler,
}

impl Database {
    /**
     * loads all schema files from the database storage and adds it to the database instance.
     */
    fn load_schemas(&mut self) {
        let mut schemas = self.path.clone();
        schemas.push(".schemas");
        for _file in std::fs::read_dir(schemas).unwrap() {
            let file = _file.unwrap();
            let raw_configs: std::collections::HashMap<String, String> = serde_json::from_reader(
                std::io::BufReader::new(std::fs::File::open(&file.path()).unwrap()),
            )
            .unwrap();

            let schema = schema::Schema::new(
                file.path(),
                self.path.clone(),
                file.file_name().into_string().unwrap(),
                raw_configs,
            );
            self.schemas
                .insert(schema.name.split(".").next().unwrap().to_string(), schema);
        }
    }
    pub fn new(name: String) -> Database {
        let schemas = std::collections::HashMap::new();
        let path = fs_utils::get_database_dir(name.to_string());
        let command_handler = CommandHandler {
            db_path: path.clone(),
        };
        let mut database = Database {
            name: name.clone(),
            schemas,
            path,
            command_handler,
        };
        fs_utils::verify_setup(database.name.clone());
        database.load_schemas();
        return database;
    }

    #[allow(unused_mut)]
    pub fn run_command(&self, command: &str, mut tokens: Vec<&str>) {
        match command {
            "!create_schema" => {
                self.command_handler.create_schema_command(tokens);
            }
            _ => {}
        }
    }

    pub fn run(&self) {
        clearscreen::clear().unwrap();
        color_print::cprintln!(
            r#"<blue>  ___  ____    ____   ____   ____ 
 /___)|  _ \  / _  | / ___) / _  )       *         ☀️     .
|___ || | | |( ( | |( (___ ( (/ /   .     🌏     *       '
(___/ | ||_/  \_||_| \____) \____)         *        * 
      |_|"#
        );

        let mut cmd: String = String::new();
        color_print::cprintln!(
            "<green> Welcome to space-db, connected to <cyan><bold>[{}]",
            self.name
        );
        loop {
            print!("> ");
            cmd.clear();
            std::io::stdout().flush().unwrap();
            while !cmd.trim().ends_with(";") {
                std::io::stdin().read_line(&mut cmd).unwrap();
                if [":q", ":quit", "!q", "!quit"].contains(&cmd.trim()) {
                    color_print::cprintln!("Thanks for using <blue><bold>space 🌌");
                    std::process::exit(0);
                }
            }
            let mut tokens: Vec<&str> = cmd
                .trim()
                .strip_suffix(';')
                .unwrap()
                .split_whitespace()
                .collect();
            let command = tokens.remove(0);
            self.run_command(command, tokens);
        }
    }
}
