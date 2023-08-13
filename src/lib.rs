use framels::{basic_listing, paths::Paths};
use glob::PatternError;
use glob::glob;
use jwalk::WalkDir;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use std::fs;

/// get_walk is a function to walk the content of a directory and his
/// subfolders
/// The result is sorted to be consistant trough the os
pub fn get_walk(input_path: &str) -> Vec<String> {
    WalkDir::new(input_path)
        .sort(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|x| x.path().display().to_string())
        .collect()
}
#[test]
fn test_get_walk() {
    assert_eq!(9, get_walk("./samples/").len());
}
/// get_list_dir is a function to list the content of a directory
/// The result is sorted to be consistant trough the os
pub fn get_list_dir(input_path: &str) -> Vec<String> {
    let mut dir_list: Vec<String> = fs::read_dir(input_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|x| x.path().display().to_string())
        .collect();
    dir_list.sort();
    dir_list
}
#[test]
fn test_get_list_dir() {
    assert_eq!(7, get_list_dir("./samples").len());
}
/// get_glob is a function to glob the content of a directory. Return
/// an error pattern if the pattern can't be used by glod lib
pub fn get_glob(input_path: &str) -> Result<Vec<String>, PatternError> {
    let paths = glob(input_path)?;
    Ok(paths
        .filter_map(|e| e.ok())
        .map(|x| x.display().to_string())
        .collect::<Vec<String>>())
}

#[test]
fn test_get_glob() {
    assert_eq!(5, get_glob("./samples/*.tif").unwrap().len());
    assert_eq!(3, get_glob("./samples/aaa.00[1-3].tif").unwrap().len());
}
///Basic function to translate a Windows path to Unix
pub fn from_slash(s: String) -> String {
    let temp_str = str::replace(&s, "\\\\", "\\");
    str::replace(&temp_str, "\\", "/")
}
#[test]
fn test_from_slash() {
    assert_eq!(
        "/caroline/bank/",
        from_slash("\\\\caroline\\bank\\".to_string())
    )
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
/// InputPath is a simple struct to represent an input_path
pub struct InputPath {
    input_path: String,
}
impl InputPath {
    /// This function create a new InputPath based on a String
    /// Mainly create for testing purpose
    pub fn new(s: String) -> InputPath {
        InputPath { input_path: s }
    }
}
#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
/// QuietPaths is a simple struct to represent the output of the crate by
/// getting a vector of strings
pub struct QuietPaths {
    paths_list: Vec<String>,
}

impl QuietPaths {
    /// Create a QuietPaths from a get_list_tdir function
    pub fn from_listdir(input_path: Json<InputPath>) -> Self {
        QuietPaths {
            paths_list: get_list_dir(input_path.input_path.as_str()),
        }
    }
    /// Create a QuietPaths from a get_glob function can return a error if
    /// if the pattern is not supported by glob lib like double wild card in
    /// the middle of a path
    pub fn from_glob(input_path: Json<InputPath>) -> Result<Self, PatternError> {
        Ok(QuietPaths {
            paths_list: get_glob(input_path.input_path.as_str())?,
        })
    }
    /// Create a QuietPaths from a get_walk function
    pub fn from_walk(input_path: Json<InputPath>) -> Self {
        QuietPaths {
            paths_list: get_walk(input_path.input_path.as_str()),
        }
    }
    pub fn to_paths(&self) -> Paths {
        Paths::new(self.paths_list.clone())
    }
    pub fn from_paths(paths: Paths) -> Self {
        QuietPaths {
            paths_list: paths.to_vec(),
        }
    }
    pub fn packed(&self) -> Self {
        QuietPaths::from_paths(basic_listing(self.to_paths()).get_paths())
    }
    pub fn from_string(s: String)->Self{
        QuietPaths { paths_list: vec![s] }
    }
}
