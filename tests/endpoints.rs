use actix_web::{test, App};
use parser::routes::{greet, parse_file};

use serde::Deserialize;

#[derive(Deserialize)]
struct HelloResponse {
    message: String,
}

#[derive(Deserialize)]
struct ParseResponse {
    text: String,
}

// Tests the default route
#[actix_web::test]
async fn get_hello() {
    // Setup
    let app = test::init_service(App::new().service(greet)).await;

    // Create request
    let req = test::TestRequest::get()
        .uri("/hello/test_name")
        .to_request();

    // Get response
    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body: HelloResponse = test::read_body_json(resp).await;

    // Assert the results
    assert!(status.is_success());
    assert_eq!(body.message, "Hello test_name!");
}

// This is the final integration test, it supposed to fail a lot before passing to comfirm the final implementation of the parsing endpoint
#[actix_web::test]
async fn post_parse_pdf() {
    // Setup
    let app = test::init_service(App::new().service(parse_file)).await;

    // NOTE: Maybe create a temporary pdf rather than having stored one in inputs
    // Read file
    let file_bytes = std::fs::read("tests/inputs/test_pdf.pdf").unwrap();

    // Create request
    let req = test::TestRequest::post()
        .uri("/parse")
        .set_payload(file_bytes)
        .insert_header(("content-type", "multipart/form-data"))
        .to_request();

    // Get response
    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body: ParseResponse = test::read_body_json(resp).await;

    // Assert the results
    assert!(status.is_success());
    assert_eq!(body.text, "Hello, this is a test pdf for the parsing API.");
}
