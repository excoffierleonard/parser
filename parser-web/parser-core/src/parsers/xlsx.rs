//! XLSX parser module.

use crate::errors::ParserError;
use calamine::{open_workbook, Reader, Xlsx};
use std::path::Path;

// TODO: Need proper logic to escape commas and quotes
// TODO: Consider using the csv crate to simply convert to csv each sheet and pass it throught the parse text function
/// Parse an XLSX file and extract text from it.
pub(crate) fn parse_xlsx(file_path: &Path) -> Result<String, ParserError> {
    let mut excel: Xlsx<_> = open_workbook(file_path)?;

    let mut csv_data = String::new();

    for name in excel.sheet_names() {
        if let Ok(range) = excel.worksheet_range(&name) {
            if !csv_data.is_empty() {
                csv_data.push_str("\n--- Sheet: ");
                csv_data.push_str(&name);
                csv_data.push_str(" ---\n");
            }
            let sheet_csv = range
                .rows()
                .map(|row| {
                    row.iter()
                        .map(|cell| cell.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                })
                .collect::<Vec<String>>()
                .join("\n");
            csv_data.push_str(&sheet_csv);
        }
    }

    Ok(csv_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parse_xlsx_single_sheet_success() {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("inputs")
            .join("test_xlsx_1.xlsx");
        let result = parse_xlsx(&file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "username,identifier,first_name
johndoe123,4281,John
alice23,8425,Alice"
                .to_string()
        );
    }

    #[test]
    fn parse_xlsx_multiple_sheets_success() {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("inputs")
            .join("test_xlsx_2.xlsx");
        let result = parse_xlsx(&file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "username,identifier,first_name
alice23,8425,Alice
--- Sheet: Sheet2 ---
username,identifier,first_name
johndoe123,4281,John"
                .to_string()
        );
    }
}
