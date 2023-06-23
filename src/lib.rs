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
