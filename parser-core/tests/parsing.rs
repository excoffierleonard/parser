use parser_core::{
    errors::ParserError,
    parsers::{parse_any, parse_docx, parse_image, parse_pdf, parse_pptx, parse_text, parse_xlsx},
};

use std::path::{Path, PathBuf};

// Test file case structure
#[derive(Clone)]
struct ParseTestCase {
    file_path: PathBuf,
    expected_text: &'static str,
}

// Helper function to test file parsing with specific parser
fn test_parse_specific(parser: fn(&Path) -> Result<String, ParserError>, test_case: ParseTestCase) {
    let result = parser(&test_case.file_path).unwrap();

    // Assert the results
    assert!(result.len() > 0);
    assert_eq!(result, test_case.expected_text);
}

// Helper function to test generic file parsing
fn test_parse_any(test_case: ParseTestCase) {
    let result = parse_any(&test_case.file_path).unwrap();

    // Assert the results
    assert!(result.len() > 0);
    assert_eq!(result, test_case.expected_text);
}

// Helper function to run parser tests
fn run_parser_tests(
    parser: fn(&Path) -> Result<String, ParserError>,
    test_cases: Vec<ParseTestCase>,
) {
    for test_case in test_cases {
        test_parse_specific(parser, test_case.clone());
        test_parse_any(test_case);
    }
}

// Helper function to get the test input path
fn test_input_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("inputs")
}

// Macro to create test cases
macro_rules! create_test_cases {
    ($($file:expr => $text:expr),* $(,)?) => {
        vec![
            $(
                ParseTestCase {
                    file_path: test_input_path().join($file),
                    expected_text: $text,
                },
            )*
        ]
    };
}

#[test]
fn test_parse_pdf_files() {
    run_parser_tests(
        parse_pdf,
        create_test_cases! {
            "test_pdf_1.pdf" => "Hello, this is a test pdf for the parsing API.",
            "test_pdf_2.pdf" => "Hello, this is another test pdf for the parsing API.",
        },
    );
}

#[test]
fn test_parse_docx_files() {
    run_parser_tests(
        parse_docx,
        create_test_cases! {
            "test_docx_1.docx" => "Hello, this is a test docx for the parsing API.",
            "test_docx_2.docx" => "Hello, this is another test docx for the parsing API.",
        },
    );
}

#[test]
fn test_parse_xlsx_files() {
    run_parser_tests(
        parse_xlsx,
        create_test_cases! {
            "test_xlsx_1.xlsx" => "username,identifier,first_name
johndoe123,4281,John
alice23,8425,Alice",
            "test_xlsx_2.xlsx" => "username,identifier,first_name
alice23,8425,Alice
--- Sheet: Sheet2 ---
username,identifier,first_name
johndoe123,4281,John",
        },
    );
}

#[test]
fn test_parse_pptx_files() {
    run_parser_tests(
        parse_pptx,
        create_test_cases! {
            "test_pptx_1.pptx" => "This is the title
This is the subtitle

--- Slide 2 ---
This is the title of the second slide
This is the text of the second slide",
        },
    );
}

#[test]
fn test_parse_text_files() {
    run_parser_tests(
        parse_text,
        create_test_cases! {
            "test_txt_1.txt" => "Hello, this is a test txt for the parsing API.",
            "test_txt_2.txt" => "Hello, this is another test txt for the parsing API.",
            "test_csv_1.csv" => "Username; Identifier;First name;Last name
booker12;9012;Rachel;Booker
grey07;2070;Laura;Grey",
            "test_json_1.json" => r#"{
    "name": "John Doe",
    "age": 30,
    "email": "john@example.com"
}"#,
        },
    );
}

#[test]
fn test_parse_image_files() {
    let image_text = "Hello World! This is an OCR test.\n123456789\n0.123 | 45.67 | 890";
    run_parser_tests(
        parse_image,
        create_test_cases! {
            "test_png_1.png" => image_text,
            "test_jpg_1.jpg" => image_text,
            "test_webp_1.webp" => image_text,
        },
    );
}
