use framels::{basic_listing, paths::Paths};
use glob::glob;
use glob::PatternError;
use jwalk::rayon::prelude::IntoParallelRefIterator;
use jwalk::rayon::prelude::ParallelIterator;
use jwalk::WalkDir;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use std::fs;
use std::path::PathBuf;

fn common_file_operation<F>(input_path: &str, file_op: F) -> Vec<String>
where
    F: Fn(fs::DirEntry) -> String,
{
    fs::read_dir(input_path)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(file_op)
        .collect()
}

pub fn get_walk(input_path: &str) -> Vec<String> {
    WalkDir::new(input_path)
        .sort(true)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path().display().to_string())
        .collect()
}

pub fn get_list_dir(input_path: &str) -> Vec<String> {
    let mut dir_list =
        common_file_operation(input_path, |entry| entry.path().display().to_string());
    dir_list.sort();
    dir_list
}

pub fn get_glob(input_path: &str) -> Result<Vec<String>, PatternError> {
    let paths = glob(input_path)?;
    Ok(paths
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.display().to_string())
        .collect())
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
    /// Create a QuietPaths from a get_walk function
    pub fn to_paths(&self) -> Paths {
        let data: Vec<PathBuf> = self
            .paths_list
            .par_iter()
            .map(|x| PathBuf::from(x))
            .collect::<Vec<PathBuf>>();
        Paths::from(data)
    }
    /// Create a QuietPaths from a get_walk function
    pub fn from_paths(paths: Paths) -> Self {
        let paths_list: Vec<String> = paths
            .par_iter()
            .map(|f| f.to_string_lossy().into_owned())
            .collect::<Vec<String>>();
        QuietPaths {
            paths_list: paths_list,
        }
    }
    /// Pack a quiet path to a basic listing using framels lib
    pub fn packed(&self) -> Self {
        QuietPaths::from_paths(basic_listing(self.to_paths()).get_paths())
    }
    /// create a QuietPaths from a string
    /// mainly for testing purpose
    pub fn from_string(s: String) -> Self {
        QuietPaths {
            paths_list: vec![s],
        }
    }
}
