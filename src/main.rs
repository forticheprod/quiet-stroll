//! # quiet-stroll
//!
//! ## Decription
//!
//! This repository is intend to create a POC of using rust to deliver client/server FS tools to:
//!
//! - **walk**, crawl the file system from an entrypoint in the file tree
//! - **listdir**, simply list the files in a directory
//! - **glob**, use glob
//!
#[macro_use]
extern crate rocket;
use quiet_stroll::{InputPath, QuietPaths};
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};
#[cfg(test)]
mod tests;

#[openapi(tag = "Default")]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[openapi(tag = "Fun")]
#[get("/coffee")]
/// # coffee
///
/// ## Description
///
/// Coffee is a fun function to use
///  [HTCPCP](https://en.wikipedia.org/wiki/Hyper_Text_Coffee_Pot_Control_Protocol)
/// and generate a error 418
fn coffee() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(Status::ImATeapot, content::RawJson("{ \"hi\": \"world\" }"))
}
#[openapi(tag = "FileSystem")]
#[post("/walk?<packed>", format = "application/json", data = "<input_path>")]
/// # walk
///
/// ## Description
///
/// Walk the directories from the entrypoint and return a Json of the paths
///
/// ## Tips
///
/// It is recommanded to use path with slash `/` instead of backslash `\`
///
/// ## Parameters
///
/// ### packed
///
/// You can use a filter `packed=true` or `packed=true` to pack frame sequences
fn fwalk(input_path: Json<InputPath>, packed: Option<bool>) -> Json<QuietPaths> {
    let input_path: QuietPaths = QuietPaths::from_walk(input_path);
    if packed.unwrap_or(false) {
        Json(input_path.packed())
    } else {
        Json(input_path)
    }
}
#[openapi(tag = "FileSystem")]
#[post(
    "/listdir?<packed>",
    format = "application/json",
    data = "<input_path>"
)]
/// # listdir
///
/// ## Description
///
/// List the files and directory and return a Json of the paths
///
/// ## Tips
///
/// It is recommanded to use path with slash `/` instead of backslash `\`
///
/// ## Parameters
///
/// ### packed
///
/// You can use a filter `packed=true` or `packed=true` to pack frame sequences
fn flistdir(input_path: Json<InputPath>, packed: Option<bool>) -> Json<QuietPaths> {
    let input_path: QuietPaths = QuietPaths::from_listdir(input_path);
    if packed.unwrap_or(false) {
        Json(input_path.packed())
    } else {
        Json(input_path)
    }
}

#[openapi(tag = "FileSystem")]
#[post("/glob?<packed>", format = "application/json", data = "<input_path>")]
/// # glob
///
/// ## Description
///
/// Use a glob pattern to return a Json of the paths
///
/// ## Tips
///
/// It is recommanded to use path with slash `/` instead of backslash `\`
///
/// ## Parameters
///
/// ### packed
///
/// You can use a filter `packed=true` or `packed=true` to pack frame sequences
fn fglob(input_path: Json<InputPath>, packed: Option<bool>) -> Json<QuietPaths> {
    let quiet_path:QuietPaths = match QuietPaths::from_glob(input_path) {
        Ok(val) =>if packed.unwrap_or(false) {
            val.packed()
        } else {
            val
        },
        Err(err) => QuietPaths::from_string(format!("{}",err))
    };
    Json(quiet_path)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            openapi_get_routes![index, flistdir, fglob, fwalk, coffee],
        )
        .mount(
            "/docs/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
