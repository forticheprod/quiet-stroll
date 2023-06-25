use glob::glob;
use jwalk::WalkDir;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use std::fs;

pub fn get_walk(input_path: &str) -> Vec<String> {
    WalkDir::new(input_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|x| x.path().display().to_string())
        .collect()
}
#[test]
fn test_get_walk() {
    assert_eq!(9, get_walk("./samples/").len());
}

pub fn get_list_dir(input_path: &str) -> Vec<String> {
    fs::read_dir(input_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|x| x.path().display().to_string())
        .collect()
}
#[test]
fn test_get_list_dir() {
    assert_eq!(7, get_list_dir("./samples").len());
}
pub fn get_glob(input_path: &str) -> Vec<String> {
    glob(input_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|x| x.display().to_string())
        .collect()
}

#[test]
fn test_get_glob() {
    assert_eq!(5, get_glob("./samples/*.tif").len());
    assert_eq!(3, get_glob("./samples/aaa.00[1-3].tif").len());
}

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
pub struct InputPath {
    input_path: String,
}
impl InputPath {
    pub fn new(s: String) -> InputPath {
        InputPath { input_path: s }
    }
}
#[derive(Serialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
pub struct Paths {
    paths_list: Vec<String>,
}

impl Paths {
    pub fn from_listdir(input_path: Json<InputPath>) -> Paths {
        Paths {
            paths_list: get_list_dir(input_path.input_path.as_str()),
        }
    }
    pub fn from_glob(input_path: Json<InputPath>) -> Paths {
        Paths {
            paths_list: get_glob(input_path.input_path.as_str()),
        }
    }
    pub fn from_walk(input_path: Json<InputPath>) -> Paths {
        Paths {
            paths_list: get_walk(input_path.input_path.as_str()),
        }
    }
}
