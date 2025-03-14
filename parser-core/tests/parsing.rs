use parser_core::InputFiles;

use std::path::PathBuf;

// Helper function to get the test input path
fn build_input_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("inputs")
}

fn get_test_data() -> (Vec<PathBuf>, Vec<String>) {
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
        "Hello, this is a test pdf for the parsing API.".to_string(),
        "Hello, this is another test pdf for the parsing API.".to_string(),
        "Hello, this is a test docx for the parsing API.".to_string(),
        "Hello, this is another test docx for the parsing API.".to_string(),
        "username,identifier,first_name
johndoe123,4281,John
alice23,8425,Alice"
            .to_string(),
        "username,identifier,first_name
alice23,8425,Alice
--- Sheet: Sheet2 ---
username,identifier,first_name
johndoe123,4281,John"
            .to_string(),
        "This is the title
This is the subtitle

--- Slide 2 ---
This is the title of the second slide
This is the text of the second slide"
            .to_string(),
        "Hello, this is a test txt for the parsing API.".to_string(),
        "Hello, this is another test txt for the parsing API.".to_string(),
        "Username; Identifier;First name;Last name
booker12;9012;Rachel;Booker
grey07;2070;Laura;Grey"
            .to_string(),
        r#"{
    "name": "John Doe",
    "age": 30,
    "email": "john@example.com"
}"#
        .to_string(),
        "Hello World! This is an OCR test.\n123456789\n0.123 | 45.67 | 890".to_string(),
        "Hello World! This is an OCR test.\n123456789\n0.123 | 45.67 | 890".to_string(),
        "Hello World! This is an OCR test.\n123456789\n0.123 | 45.67 | 890".to_string(),
    ];

    (inputs, expected_texts)
}

fn prepare_test_input(inputs: &[PathBuf]) -> Vec<Vec<u8>> {
    inputs
        .iter()
        .map(|path| std::fs::read(path).unwrap())
        .collect()
}

#[test]
fn parse_success() {
    let (inputs, expected_texts) = get_test_data();
    let data = prepare_test_input(&inputs);

    let result = InputFiles::new(data).parse().unwrap();

    // Assert the results
    assert_eq!(result.len(), inputs.len());
    assert_eq!(result, expected_texts);
}
