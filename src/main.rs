use quiet_stroll;
#[macro_use]
extern crate rocket;
use quiet_stroll::{InputPath, Paths};
use rocket::http::Status;
use rocket::response::{content, status};
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};

#[openapi(tag = "Default")]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[openapi(tag = "Fun")]
#[get("/coffee")]
fn coffee() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(Status::ImATeapot, content::RawJson("{ \"hi\": \"world\" }"))
}
#[openapi(tag = "FileSystem")]
#[post("/listdir", format = "application/json", data = "<input_path>")]
fn flistdir(input_path: Json<InputPath>) -> Json<Paths> {
    Json(Paths::from_listdir(input_path))
}
#[openapi(tag = "FileSystem")]
#[post("/glob", format = "application/json", data = "<input_path>")]
fn fglob(input_path: Json<InputPath>) -> Json<Paths> {
    Json(Paths::from_glob(input_path))
}
#[openapi(tag = "FileSystem")]
#[post("/walk", format = "application/json", data = "<input_path>")]
fn fwalk(input_path: Json<InputPath>) -> Json<Paths> {
    Json(Paths::from_walk(input_path))
}

#[rocket::main]
async fn main() {
    let launch_result = rocket::build()
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
        .launch()
        .await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}
