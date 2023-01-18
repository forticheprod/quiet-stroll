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
#[test]
fn test_get_list_dir() {
    let expected: Vec<String> = vec![
        "./samples/aaa.001.tif".to_string(),
        "./samples/aaa.002.tif".to_string(),
        "./samples/aaa.003.tif".to_string(),
        "./samples/aaa.004.tif".to_string(),
        "./samples/aaa.005.tif".to_string(),
        "./samples/bbb.001.exr".to_string(),
    ];
    assert_eq!(expected, get_list_dir("./samples"));
}
#[test]
fn test_get_glob() {
    let expected: Vec<String> = vec![
        "samples/aaa.001.tif".to_string(),
        "samples/aaa.002.tif".to_string(),
        "samples/aaa.003.tif".to_string(),
        "samples/aaa.004.tif".to_string(),
        "samples/aaa.005.tif".to_string(),
    ];
    assert_eq!(expected, get_glob("./samples/*.tif"));
}
