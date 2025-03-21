# Parser Web API

REST API and web interface for the parser-core library, enabling document parsing through HTTP requests.

## Features

- RESTful API for document parsing
- Optional static file serving for web interface
- Multipart file upload support
- Containerized deployment ready

## Installation

```bash
# Build from source
cargo build -p parser-web

# Run with default settings
cargo run -p parser-web
```

## Configuration

Environment variables:

- `PARSER_APP_PORT`: API server port (default: 8080)
- `ENABLE_FILE_SERVING`: Enable static file serving (default: false)
- `RUST_LOG`: Logging level (default: info)

## API Endpoints

### Parse Documents

```http
POST /parse
```

#### Request Body

Multipart form with one or more files using the key `file`.

#### Response

```json
{
    "texts": [
        "Parsed text of first document.",
        "Parsed text of second document."
    ]
}
```

#### Status Codes

- `200 OK`: Successfully parsed documents
- `400 Bad Request`: Invalid request format
- `500 Internal Server Error`: Parsing failed

## Example Usage

```bash
# Upload and parse a single file
curl -X POST \
     -F "file=@document.pdf" \
     http://localhost:8080/parse

# Upload and parse multiple files
curl -X POST \
     -F "file=@document1.pdf" \
     -F "file=@document2.docx" \
     http://localhost:8080/parse
```

## Web Interface

When `ENABLE_FILE_SERVING=true`, the server provides a simple web interface at the root URL for testing the API.