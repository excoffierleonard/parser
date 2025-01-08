# Parser API Documentation

## Endpoints

1. [Parse the texts of documents](#parse-the-texts-of-documents)

## Parse the texts of documents

### Request

#### Endpoint

```http
POST /parse
```

#### Body (multipart/form-data)

| Key | Value |
|-|-|
| file | *One or more documents to parse* |

### Response

#### Status Codes

| Code | Name |  Description |
|-|-|-|
| `200` | `OK` | Successfully parsed the text |
| `400` | `Bad Request` | Invalid request |
| `500` | `Internal Server Error` | Parsing failed |

#### Body

##### Success

```json
{
    "texts": [
        "*Parsed text of first document.*",
        "*Parsed text of second document.*"
    ]
}
```

##### Error

```json
{
    "message": "*Description of the error.*"
}
```

### Examples

#### Request

```bash
curl --request POST \
     --url "http://localhost:8080/parse" \
     --header "Content-Type: multipart/form-data" \
     --form "file=@parser-web/tests/inputs/test_pdf_1.pdf" \
     --form "file=@parser-web/tests/inputs/test_pdf_2.pdf"
```

#### Response

```json
{
    "texts": [
        "Hello, this is a test pdf for the parsing API.",
        "Hello, this is another test pdf for the parsing API."
    ]
}
```