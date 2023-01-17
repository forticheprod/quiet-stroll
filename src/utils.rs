use glob::glob;
use jwalk::WalkDir;
use std::fs;
pub fn get_walk(input_path: &str) -> Vec<String> {
    WalkDir::new(input_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|x| x.path().display().to_string())
        .collect()
}

pub fn get_list_dir(input_path: &str) -> Vec<String> {
    fs::read_dir(input_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|x| x.path().display().to_string())
        .collect()
}
pub fn get_glob(input_path: &str) -> Vec<String> {
    glob(input_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|x| x.display().to_string())
        .collect()
}
