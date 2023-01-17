mod utils;
#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::{json::Json, Deserialize, Serialize};

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
impl Paths {
    pub fn from_listdir(input_path: Json<InputPath>) -> Paths {
        Paths {
            output_paths: utils::get_list_dir(&input_path.input_path[..]),
        }
    }
    pub fn from_glob(input_path: Json<InputPath>) -> Paths {
        Paths {
            output_paths: utils::get_glob(&input_path.input_path[..]),
        }
    }
    pub fn from_walk(input_path: Json<InputPath>) -> Paths {
        Paths {
            output_paths: utils::get_walk(&input_path.input_path[..]),
        }
    }
}
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/listdir", format = "application/json", data = "<input_path>")]
fn flistdir(input_path: Json<InputPath>) -> Json<Paths> {
    Json(Paths::from_listdir(input_path))
}

#[post("/glob", format = "application/json", data = "<input_path>")]
fn fglob(input_path: Json<InputPath>) -> Json<Paths> {
    Json(Paths::from_glob(input_path))
}

#[post("/walk", format = "application/json", data = "<input_path>")]
fn fwalk(input_path: Json<InputPath>) -> Json<Paths> {
    Json(Paths::from_walk(input_path))
}

#[get("/coffe")]
fn coffe() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(Status::ImATeapot, content::RawJson("{ \"hi\": \"world\" }"))
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, flistdir, fglob, fwalk, coffe])
}
