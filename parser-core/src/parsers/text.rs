use crate::errors::ParserError;
use std::fs::read_to_string;

// Parses all that can be coerced to text
pub fn parse_text(file_path: &str) -> Result<String, ParserError> {
    Ok(read_to_string(file_path)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_txt_success() {
        let file_path = "tests/inputs/test_txt_1.txt";
        let result = parse_text(file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "Hello, this is a test txt for the parsing API.".to_string()
        );
    }

    #[test]
    fn parse_csv_success() {
        let file_path = "tests/inputs/test_csv_1.csv";
        let result = parse_text(file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "Username; Identifier;First name;Last name
booker12;9012;Rachel;Booker
grey07;2070;Laura;Grey"
                .to_string()
        );
    }

    #[test]
    fn parse_json_success() {
        let file_path = "tests/inputs/test_json_1.json";
        let result = parse_text(file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            r#"{
    "name": "John Doe",
    "age": 30,
    "email": "john@example.com"
}"#
            .to_string()
        );
    }
}
