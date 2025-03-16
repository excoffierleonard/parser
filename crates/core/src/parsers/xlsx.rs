//! XLSX parser module.

use crate::errors::ParserError;
use calamine::{Reader, Xlsx};
use std::io::Cursor;

// TODO: Need proper logic to escape commas and quotes
// TODO: Consider using the csv crate to simply convert to csv each sheet and pass it throught the parse text function
/// Parse an XLSX file and extract text from it.
pub(crate) fn parse_xlsx(data: &[u8]) -> Result<String, ParserError> {
    // Create a cursor from the bytes for memory-based reading
    let cursor = Cursor::new(data);

    // Open the workbook directly from the cursor
    // This uses the standard Read trait and avoids temporary files
    let mut excel = Xlsx::new(cursor)?;

    let mut csv_data = String::new();

    // Copy the sheet names to avoid borrowing issues
    let sheet_names = excel.sheet_names().to_vec();

    for name in sheet_names {
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
    use parser_test_utils::read_test_file;

    #[test]
    fn parse_xlsx_single_sheet_success() {
        let data = read_test_file("test_xlsx_1.xlsx");
        let result = parse_xlsx(&data).unwrap();

        assert!(!result.is_empty());
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
        let data = read_test_file("test_xlsx_2.xlsx");
        let result = parse_xlsx(&data).unwrap();

        assert!(!result.is_empty());
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
