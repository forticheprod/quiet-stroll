use glob::glob;
use jwalk::WalkDir;
use std::fs;
#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::{json::Json, Deserialize, Serialize};

fn get_walk(input_path: &str) -> Vec<String> {
    let items: Vec<String> = WalkDir::new(input_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|x| x.path().display().to_string())
        .collect();
    items
}

fn get_list_dir(input_path: &str) -> Vec<String> {
    let items: Vec<String> = fs::read_dir(input_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|x| x.path().display().to_string())
        .collect();
    items
}
fn get_glob(input_path: &str) -> Vec<String> {
    let items: Vec<String> = glob(input_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|x| x.display().to_string())
        .collect();
    items
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct InputPath {
    input_path: String,
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Paths {
    output_paths: Vec<String>,
}

#[post("/listdir", format = "application/json", data = "<input_path>")]
fn flistdir(input_path: Json<InputPath>) -> Json<Paths> {
    let p: &str = &input_path.input_path[..];
    println!("{}", p);
    let body: Paths = Paths {
        output_paths: get_list_dir(p),
    };
    Json(body)
}

#[post("/glob", format = "application/json", data = "<input_path>")]
fn fglob(input_path: Json<InputPath>) -> Json<Paths> {
    let p: &str = &input_path.input_path[..];
    println!("{}", p);
    let body: Paths = Paths {
        output_paths: get_glob(p),
    };
    Json(body)
}

#[post("/walk", format = "application/json", data = "<input_path>")]
fn fwalk(input_path: Json<InputPath>) -> Json<Paths> {
    let p: &str = &input_path.input_path[..];
    println!("{}", p);
    let body: Paths = Paths {
        output_paths: get_walk(p),
    };
    Json(body)
}

#[get("/coffe")]
fn coffe() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(Status::ImATeapot, content::RawJson("{ \"hi\": \"world\" }"))
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, flistdir, fglob, fwalk, coffe])
}
