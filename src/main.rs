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

#[cfg(test)]
mod test {
    use super::rocket;
    use quiet_stroll::InputPath;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::index)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }
    #[test]
    fn test_walk() {
        let message = InputPath::new("./samples".to_string());
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/walk").json(&message).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "{\"paths_list\":[\".\\\\samples\",\"./samples\\\\aaa.001.tif\",\"./samples\\\\aaa.002.tif\",\"./samples\\\\aaa.003.tif\",\"./samples\\\\aaa.004.tif\",\"./samples\\\\aaa.005.tif\",\"./samples\\\\bbb.001.exr\",\"./samples\\\\subfolder\",\"./samples\\\\subfolder\\\\ccc.050.exr\"]}");
    }
    #[test]
    fn test_glob() {
        let message = InputPath::new("./samples/*.tif".to_string());
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/glob").json(&message).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "{\"paths_list\":[\"samples\\\\aaa.001.tif\",\"samples\\\\aaa.002.tif\",\"samples\\\\aaa.003.tif\",\"samples\\\\aaa.004.tif\",\"samples\\\\aaa.005.tif\"]}");
    }
}
