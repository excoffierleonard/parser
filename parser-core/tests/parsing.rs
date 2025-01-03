use parser_core::{
    errors::ParserError,
    parsers::{parse_any, parse_docx, parse_image, parse_pdf, parse_pptx, parse_text, parse_xlsx},
};

// Test file case structure
#[derive(Clone)]
struct ParseTestCase {
    file_path: &'static str,
    expected_text: &'static str,
}

// Helper function to test file parsing
fn test_parse_any(test_case: ParseTestCase) {
    let result = parse_any(test_case.file_path).unwrap();

    // Assert the results
    assert!(result.len() > 0);
    assert_eq!(result, test_case.expected_text);
}

// Helper function to test file parsing with specific parser
fn test_parse_specific(parser: fn(&str) -> Result<String, ParserError>, test_case: ParseTestCase) {
    let result = parser(test_case.file_path).unwrap();

    // Assert the results
    assert!(result.len() > 0);
    assert_eq!(result, test_case.expected_text);
}

#[test]
fn test_parse_pdf_files() {
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
        test_parse_specific(parse_pdf, test_case.clone());
        test_parse_any(test_case);
    }
}

#[test]
fn test_parse_docx_files() {
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
        test_parse_specific(parse_docx, test_case.clone());
        test_parse_any(test_case);
    }
}

#[test]
fn test_parse_xlsx_files() {
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
        test_parse_specific(parse_xlsx, test_case.clone());
        test_parse_any(test_case);
    }
}

#[test]
fn test_parse_pptx_files() {
    let test_cases = vec![ParseTestCase {
        file_path: "tests/inputs/test_pptx_1.pptx",
        expected_text: "This is the title
This is the subtitle

--- Slide 2 ---
This is the title of the second slide
This is the text of the second slide",
    }];

    for test_case in test_cases {
        test_parse_specific(parse_pptx, test_case.clone());
        test_parse_any(test_case);
    }
}

#[test]
fn test_parse_txt_files() {
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
        test_parse_specific(parse_text, test_case.clone());
        test_parse_any(test_case);
    }
}

#[test]
fn test_parse_csv_files() {
    let test_cases = vec![ParseTestCase {
        file_path: "tests/inputs/test_csv_1.csv",
        expected_text: "Username; Identifier;First name;Last name
booker12;9012;Rachel;Booker
grey07;2070;Laura;Grey",
    }];

    for test_case in test_cases {
        test_parse_specific(parse_text, test_case.clone());
        test_parse_any(test_case);
    }
}

#[test]
fn test_parse_json_files() {
    let test_cases = vec![ParseTestCase {
        file_path: "tests/inputs/test_json_1.json",
        expected_text: r#"{
    "name": "John Doe",
    "age": 30,
    "email": "john@example.com"
}"#,
    }];

    for test_case in test_cases {
        test_parse_specific(parse_text, test_case.clone());
        test_parse_any(test_case);
    }
}

#[test]
fn test_parse_png_files() {
    let test_cases = vec![ParseTestCase {
        file_path: "tests/inputs/test_png_1.png",
        expected_text: "Hello World! This is an OCR test.
123456789
0.123 | 45.67 | 890",
    }];

    for test_case in test_cases {
        test_parse_specific(parse_image, test_case.clone());
        test_parse_any(test_case);
    }
}

#[test]
fn test_parse_jpg_files() {
    let test_cases = vec![ParseTestCase {
        file_path: "tests/inputs/test_jpg_1.jpg",
        expected_text: "Hello World! This is an OCR test.
123456789
0.123 | 45.67 | 890",
    }];

    for test_case in test_cases {
        test_parse_specific(parse_image, test_case.clone());
        test_parse_any(test_case);
    }
}

#[test]
fn test_parse_webp_files() {
    let test_cases = vec![ParseTestCase {
        file_path: "tests/inputs/test_webp_1.webp",
        expected_text: "Hello World! This is an OCR test.
123456789
0.123 | 45.67 | 890",
    }];

    for test_case in test_cases {
        test_parse_specific(parse_image, test_case.clone());
        test_parse_any(test_case);
    }
}
