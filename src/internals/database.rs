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

use crate::fs_utils;

use crate::internals::schema;

pub struct Database {
    name: String,
    schemas: Vec<schema::Schema>,
    path: std::path::PathBuf,
}

impl Database {
    /**
     * loads all schema files from the database storage and adds it to the database instance.
     */
    fn load_schemas(&self) {}
    fn create_if_not_exist(&self) {
      
        if !(self.path.exists() & self.path.is_dir()) {
            println!("Writing to {:?}", self.path);
            std::fs::create_dir(self.path.clone()).unwrap();
        }
    }
    pub fn new(name: String) -> Database {
        let schemas: Vec<schema::Schema> = vec![];
        let path = fs_utils::get_database_dir(name.to_string());
        let database = Database {
            name,
            schemas,
            path,
        };
        database.create_if_not_exist();
       
        database.load_schemas();
        return database;
    }
}
