# Parser API Documentation

## Endpoints

1. [Parse the text of a document](#parse-the-text-of-a-document)

## Parse the text of a document

### Request

#### Endpoint

```http
POST /parse
```

#### Body (form-data)

| Key | Value |
|-|-|
| file | *The document to parse* |

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
    "text": "*Parsed text*"
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
     --form "file=@tests/inputs/test_pdf_1.pdf"
```

#### Response

```json
{
    "text": "Hello, this is a test pdf for the parsing API."
}
```