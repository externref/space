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

/**
Gets the path as [std::path::PathBuf] of the directory to use for data storage.
*/
pub fn get_data_save_dir() -> std::path::PathBuf {  
    if cfg!(target_os = "linux") {
        let xdg_dirs = std::env::var("HOME").unwrap();
        return std::path::PathBuf::from(format!(
            "{}/.local/share/",
            xdg_dirs.split(":").next().unwrap().to_owned(),
        ));
    } else if cfg!(target_os = "windows") {
        let appdata = std::env::var("APPDATA").unwrap();
        return std::path::PathBuf::from(format!("{}/", appdata));
    } else if cfg!(target_os = "macos") {
        let home = std::env::var("HOME").unwrap();
        return std::path::PathBuf::from(format!("{}/Library/Application Support/", home));
    } else {
        return std::path::PathBuf::from(format!("data/database/"));
    }
}


pub fn verify_setup(){
    let data_dir = get_data_save_dir();
}
  

pub fn get_database_dir(dbname: String)-> std::path::PathBuf{
    let mut strgdir = get_data_save_dir();
    strgdir.push(dbname);
    return strgdir;
}