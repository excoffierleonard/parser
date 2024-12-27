use actix_web::{
    http::header::{HeaderName, HeaderValue},
    test, App,
};
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

// Since Actix Test does not support native multipart payload, we have to build our own.
pub fn build_multipart_payload(
    file_name: &str,
    file_contents: &[u8],
) -> (Vec<u8>, (HeaderName, HeaderValue)) {
    let boundary = "-----------------------------202022185716362916172375148227";

    let mut payload = Vec::new();

    // Add form field boundary and headers
    payload.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    payload.extend_from_slice(
        format!("Content-Disposition: form-data; name=\"file\"; filename=\"{file_name}\"\r\n\r\n")
            .as_bytes(),
    );

    // Add file contents as raw bytes
    payload.extend_from_slice(file_contents);

    // Add closing boundary
    payload.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());

    let header = (
        actix_web::http::header::CONTENT_TYPE,
        HeaderValue::from_str(&format!("multipart/form-data; boundary={boundary}")).unwrap(),
    );

    (payload, header)
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

    // Assert the results
    let status = resp.status();
    assert!(status.is_success());

    let body: HelloResponse = test::read_body_json(resp).await;
    assert_eq!(body.message, "Hello test_name!");
}

#[actix_web::test]
async fn post_parse_pdf_1() {
    // Setup
    let app = test::init_service(App::new().service(parse_file)).await;

    // Read file
    let file_bytes = std::fs::read("tests/inputs/test_pdf_1.pdf").unwrap();
    let (payload, content_type_header) = build_multipart_payload("test_pdf_1.pdf", &file_bytes);

    // Create request
    let req = test::TestRequest::post()
        .uri("/parse")
        .set_payload(payload)
        .insert_header(content_type_header)
        .to_request();

    // Get response
    let resp = test::call_service(&app, req).await;

    // Assert the results
    let status = resp.status();
    assert!(status.is_success());

    let body: ParseResponse = test::read_body_json(resp).await;
    assert_eq!(body.text, "Hello, this is a test pdf for the parsing API.");
}

#[actix_web::test]
async fn post_parse_pdf_2() {
    // Setup
    let app = test::init_service(App::new().service(parse_file)).await;

    // Read file
    let file_bytes = std::fs::read("tests/inputs/test_pdf_2.pdf").unwrap();
    let (payload, content_type_header) = build_multipart_payload("test_pdf_1.pdf", &file_bytes);

    // Create request
    let req = test::TestRequest::post()
        .uri("/parse")
        .set_payload(payload)
        .insert_header(content_type_header)
        .to_request();

    // Get response
    let resp = test::call_service(&app, req).await;

    // Assert the results
    let status = resp.status();
    assert!(status.is_success());

    let body: ParseResponse = test::read_body_json(resp).await;
    assert_eq!(
        body.text,
        "Hello, this is another test pdf for the parsing API."
    );
}
