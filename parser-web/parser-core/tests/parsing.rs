use parser_core::InputFiles;

use std::path::PathBuf;

// Helper function to get the test input path
fn build_input_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("inputs")
}

#[test]
fn parse_success() {
    let inputs: Vec<PathBuf> = vec![
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

    let result = InputFiles::new(inputs.clone()).parse().unwrap();

    // Assert the results
    assert_eq!(result.len(), inputs.len());
    assert_eq!(result, expected_texts);
}
