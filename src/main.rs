#![feature(proc_macro_hygiene, decl_macro)] // language features needed by Rocket

// Importing the rocket macros
#[macro_use]
extern crate rocket;

// Importing the OpenAPI macros
#[macro_use]
extern crate rocket_okapi;

use serde::*;
use rocket_contrib::json::Json;
use rocket_okapi::JsonSchema;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

/// Host information structure returned at /hostinfo
#[derive(Serialize, JsonSchema, Debug)]
struct HostInfo {
    hostname: String,
    pid: u32,
    uptime: u64,
}

// Create route / that returns "Hello, world!"
#[openapi]
#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

/// Create route /hostinfo that returns information about the host serving 
/// this page.
#[openapi]
#[get("/hostinfo")]
fn hostinfo() -> Json<HostInfo> {
    // gets the current machine hostname or "unknown" if the hostname
    // doesn't parse into UTF-8 (very unlikely)
    let hostname = gethostname::gethostname()
        .into_string()
        //.or(|_| "unknown".to_string())
        .unwrap();
    
    Json(HostInfo{
        hostname: hostname,
        pid: std::process::id(),
        uptime: psutil::host::uptime()
            .unwrap() // normally this is a bad idea, but this code is
                      // very unlikely to fail.
            .as_secs(),
    })
}

fn main() {
    rocket::ignite()
        .mount("/", routes_with_openapi![index, hostinfo])
        .mount(
            "/swagger_ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: Some("../openapi.json".to_owned()),
                urls: None,
                }),
            )
        .launch();
}

#[cfg(test)] // Only compile this when unit testing is requested
mod tests {
  use super::*; // Modules are their own scope
                // So, you need to explictly use the stuff in
                // the parent module.
  use rocket::http::Status;
  use rocket::local::*;

  #[test]
  fn test_index() {
    // Create the rocket instance to test
    let rkt = rocket::ignite().mount("/", routes![index]);

    // Create a HTTP client bound to this rocket instance
    let client = Client::new(rkt).expect("valid rocket");

    // get a HTTP response
    let mut response = client.get("/").dispatch();

    // Ensure it returns HTTP 200
    assert_eq!(response.status(), Status::Ok);

    // Ensure the body is what we expect it to be
    assert_eq!(response.body_string(), Some("Hello, World!".into()));
  }
}
