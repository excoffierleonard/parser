use actix_web::{
    http::header::{HeaderName, HeaderValue},
    test, App,
};
use parser_web::routes::{greet, parse_file};
use serde::Deserialize;

#[derive(Deserialize)]
struct HelloResponse {
    message: String,
}

#[derive(Deserialize)]
struct ParseResponse {
    text: String,
}

// Test file case structure
struct ParseTestCase {
    file_path: &'static str,
    expected_text: &'static str,
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

// Helper function to test file parsing
async fn test_parse_file(test_case: ParseTestCase) {
    // Setup
    let app = test::init_service(App::new().service(parse_file)).await;

    // Read file
    let file_bytes = std::fs::read(test_case.file_path).unwrap();
    let file_name = std::path::Path::new(test_case.file_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let (payload, content_type_header) = build_multipart_payload(file_name, &file_bytes);

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
    assert_eq!(body.text, test_case.expected_text);
}

#[actix_web::test]
async fn test_parse_pdf_files() {
    let test_cases = vec![
        ParseTestCase {
            file_path: "tests/inputs/test_pdf_1.pdf",
            expected_text: "Hello, this is a test pdf for the parsing API.",
        },
        ParseTestCase {
            file_path: "tests/inputs/test_pdf_2.pdf",
            expected_text: "Hello, this is another test pdf for the parsing API.",
        },
    ];

    for test_case in test_cases {
        test_parse_file(test_case).await;
    }
}

#[actix_web::test]
async fn test_parse_docx_files() {
    let test_cases = vec![
        ParseTestCase {
            file_path: "tests/inputs/test_docx_1.docx",
            expected_text: "Hello, this is a test docx for the parsing API.",
        },
        ParseTestCase {
            file_path: "tests/inputs/test_docx_2.docx",
            expected_text: "Hello, this is another test docx for the parsing API.",
        },
    ];

    for test_case in test_cases {
        test_parse_file(test_case).await;
    }
}

#[actix_web::test]
async fn test_parse_xlsx_files() {
    let test_cases = vec![
        ParseTestCase {
            file_path: "tests/inputs/test_xlsx_1.xlsx",
            expected_text: "username,identifier,first_name
johndoe123,4281,John
alice23,8425,Alice",
        },
        ParseTestCase {
            file_path: "tests/inputs/test_xlsx_2.xlsx",
            expected_text: "username,identifier,first_name
alice23,8425,Alice
--- Sheet: Sheet2 ---
username,identifier,first_name
johndoe123,4281,John",
        },
    ];

    for test_case in test_cases {
        test_parse_file(test_case).await;
    }
}

#[actix_web::test]
async fn test_parse_pptx_files() {
    let test_cases = vec![ParseTestCase {
        file_path: "tests/inputs/test_pptx_1.pptx",
        expected_text: "This is the title
This is the subtitle

--- Slide 2 ---
This is the title of the second slide
This is the text of the second slide",
    }];

    for test_case in test_cases {
        test_parse_file(test_case).await;
    }
}

#[actix_web::test]
async fn test_parse_txt_files() {
    let test_cases = vec![
        ParseTestCase {
            file_path: "tests/inputs/test_txt_1.txt",
            expected_text: "Hello, this is a test txt for the parsing API.",
        },
        ParseTestCase {
            file_path: "tests/inputs/test_txt_2.txt",
            expected_text: "Hello, this is another test txt for the parsing API.",
        },
    ];

    for test_case in test_cases {
        test_parse_file(test_case).await;
    }
}

#[actix_web::test]
async fn test_parse_csv_files() {
    let test_cases = vec![ParseTestCase {
        file_path: "tests/inputs/test_csv_1.csv",
        expected_text: "Username; Identifier;First name;Last name
booker12;9012;Rachel;Booker
grey07;2070;Laura;Grey",
    }];

    for test_case in test_cases {
        test_parse_file(test_case).await;
    }
}

#[actix_web::test]
async fn test_parse_json_files() {
    let test_cases = vec![ParseTestCase {
        file_path: "tests/inputs/test_json_1.json",
        expected_text: r#"{
    "name": "John Doe",
    "age": 30,
    "email": "john@example.com"
}"#,
    }];

    for test_case in test_cases {
        test_parse_file(test_case).await;
    }
}

#[actix_web::test]
async fn test_parse_png_files() {
    let test_cases = vec![ParseTestCase {
        file_path: "tests/inputs/test_png_1.png",
        expected_text: "Hello World! This is an OCR test.
123456789
0.123 | 45.67 | 890",
    }];

    for test_case in test_cases {
        test_parse_file(test_case).await;
    }
}

#[actix_web::test]
async fn test_parse_jpg_files() {
    let test_cases = vec![ParseTestCase {
        file_path: "tests/inputs/test_jpg_1.jpg",
        expected_text: "Hello World! This is an OCR test.
123456789
0.123 | 45.67 | 890",
    }];

    for test_case in test_cases {
        test_parse_file(test_case).await;
    }
}

#[actix_web::test]
async fn test_parse_webp_files() {
    let test_cases = vec![ParseTestCase {
        file_path: "tests/inputs/test_webp_1.webp",
        expected_text: "Hello World! This is an OCR test.
123456789
0.123 | 45.67 | 890",
    }];

    for test_case in test_cases {
        test_parse_file(test_case).await;
    }
}
