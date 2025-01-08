use actix_web::{
    http::header::{HeaderName, HeaderValue},
    test, App,
};
use parser_web::routes::{greet, parse_file};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
struct HelloResponse {
    message: String,
}

#[derive(Deserialize)]
struct ParseResponse {
    texts: Vec<String>,
}

// Since Actix Test does not support native multipart payload, we have to build our own.
pub fn build_multipart_payload(file_paths: Vec<PathBuf>) -> (Vec<u8>, (HeaderName, HeaderValue)) {
    let boundary = "-----------------------------202022185716362916172375148227";
    let mut payload = Vec::new();

    for file_path in file_paths {
        let file_bytes = std::fs::read(&file_path).unwrap();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        // Add form field boundary and headers
        payload.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
        payload.extend_from_slice(
            format!(
                "Content-Disposition: form-data; name=\"file\"; filename=\"{file_name}\"\r\n\r\n"
            )
            .as_bytes(),
        );

        // Add file contents as raw bytes
        payload.extend_from_slice(&file_bytes);
        payload.extend_from_slice(b"\r\n");
    }

    // Add closing boundary
    payload.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());

    let header = (
        actix_web::http::header::CONTENT_TYPE,
        HeaderValue::from_str(&format!("multipart/form-data; boundary={boundary}")).unwrap(),
    );

    (payload, header)
}

// Helper function to get the test input path
fn build_input_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("inputs")
}

// Test the greet endpoint
#[actix_web::test]
async fn request_hello_success() {
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
async fn request_parse_success() {
    let file_paths: Vec<PathBuf> = vec![
        "test_pdf_1.pdf",
        "test_pdf_2.pdf",
        "test_docx_1.docx",
        "test_docx_2.docx",
        "test_xlsx_1.xlsx",
        "test_xlsx_2.xlsx",
        "test_pptx_1.pptx",
        "test_txt_1.txt",
        "test_txt_2.txt",
        "test_csv_1.csv",
        "test_json_1.json",
        "test_png_1.png",
        "test_jpg_1.jpg",
        "test_webp_1.webp",
    ]
    .iter()
    .map(|x| build_input_path().join(x))
    .collect();

    let expected_texts = vec![
        "Hello, this is a test pdf for the parsing API.",
        "Hello, this is another test pdf for the parsing API.",
        "Hello, this is a test docx for the parsing API.",
        "Hello, this is another test docx for the parsing API.",
        "username,identifier,first_name
johndoe123,4281,John
alice23,8425,Alice",
        "username,identifier,first_name
alice23,8425,Alice
--- Sheet: Sheet2 ---
username,identifier,first_name
johndoe123,4281,John",
        "This is the title
This is the subtitle

--- Slide 2 ---
This is the title of the second slide
This is the text of the second slide",
        "Hello, this is a test txt for the parsing API.",
        "Hello, this is another test txt for the parsing API.",
        "Username; Identifier;First name;Last name
booker12;9012;Rachel;Booker
grey07;2070;Laura;Grey",
        r#"{
    "name": "John Doe",
    "age": 30,
    "email": "john@example.com"
}"#,
        "Hello World! This is an OCR test.\n123456789\n0.123 | 45.67 | 890",
        "Hello World! This is an OCR test.\n123456789\n0.123 | 45.67 | 890",
        "Hello World! This is an OCR test.\n123456789\n0.123 | 45.67 | 890",
    ];

    // Setup
    let app = test::init_service(App::new().service(parse_file)).await;

    // Build multipart payload
    let (payload, content_type_header) = build_multipart_payload(file_paths.clone());

    // Create request
    let req = test::TestRequest::post()
        .uri("/parse")
        .insert_header(content_type_header)
        .set_payload(payload)
        .to_request();

    // Get response
    let resp = test::call_service(&app, req).await;

    // Assert the results
    let status = resp.status();
    assert!(status.is_success());

    let body: ParseResponse = test::read_body_json(resp).await;
    assert_eq!(body.texts, expected_texts);

    // Assert the results
    assert_eq!(body.texts.len(), file_paths.len());
}
