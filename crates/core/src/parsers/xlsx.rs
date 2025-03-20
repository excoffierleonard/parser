//! XLSX parser module.
//!
//! This module provides functionality for extracting text from Microsoft Excel
//! XLSX spreadsheet files using the calamine library. It converts spreadsheet
//! content to a CSV-like text format.

use crate::errors::ParserError;
use calamine::{Reader, Xlsx};
use std::io::Cursor;

/// Parses an XLSX file and extracts text content as CSV.
///
/// This function takes raw bytes of an XLSX spreadsheet and extracts all cell
/// values as a comma-separated text representation, with support for multiple
/// sheets.
///
/// # Arguments
///
/// * `data` - A byte slice containing the XLSX data
///
/// # Returns
///
/// * `Ok(String)` - The extracted text from the spreadsheet in CSV format
/// * `Err(ParserError)` - If an error occurs during XLSX parsing
///
/// # Implementation Notes
///
/// * Uses the calamine library for XLSX parsing
/// * Converts each sheet to CSV format with comma-separated values
/// * Adds sheet headers for multi-sheet workbooks
/// * Memory-efficient implementation using cursors instead of temporary files
/// * TODO: Need proper logic to escape commas and quotes
/// * TODO: Consider using the csv crate to convert each sheet and pass it through the parse_text function
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

#[cfg(test)]
pub mod benchmarks {
    use super::*;
    use criterion::{black_box, Criterion};
    use parser_test_utils::read_test_file;

    pub fn benchmark_parse_xlsx(c: &mut Criterion) {
        let xlsx_data_1 = read_test_file("test_xlsx_1.xlsx");
        let xlsx_data_2 = read_test_file("test_xlsx_2.xlsx");

        let mut group = c.benchmark_group("XLSX Parser");
        
        group.bench_function("parse_xlsx (single sheet)", |b| {
            b.iter(|| parse_xlsx(black_box(&xlsx_data_1)))
        });
        
        group.bench_function("parse_xlsx (multiple sheets)", |b| {
            b.iter(|| parse_xlsx(black_box(&xlsx_data_2)))
        });
        
        group.finish();
    }
}
