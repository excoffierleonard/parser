mod common;

use common::test_file_path;

// Note: The endpoint tests require the web module to be exposed publicly.
// For now, we'll keep this as a placeholder. The web functionality can be tested
// via integration tests that start the actual server, or by making the web module public.
#[test]
fn test_file_paths_exist() {
    let file_names = vec![
        "test_pdf_1.pdf",
        "test_docx_1.docx",
        "test_xlsx_1.xlsx",
        "test_pptx_1.pptx",
        "test_txt_1.txt",
        "test_csv_1.csv",
        "test_json_1.json",
        "test_png_1.png",
        "test_jpg_1.jpg",
        "test_webp_1.webp",
    ];

    for name in file_names {
        let path = test_file_path(name);
        assert!(path.exists(), "Test file should exist: {:?}", path);
    }
}
