# Parser CLI

A command-line interface for the parser-core library, allowing you to parse various document formats from the command line.

## Usage

### Parse Files

Parse one or more files and extract their text content:

```
parser-cli parse <FILES>...
```

Example:
```
parser-cli parse document.pdf presentation.pptx report.docx
```

### Output to Files

You can also output the parsed content to files in a specified directory:

```
parser-cli parse --output <OUTPUT_DIR> <FILES>...
```

Example:
```
parser-cli parse --output ./parsed_output document.pdf presentation.pptx
```

## Supported File Types

- PDF documents
- Microsoft Word documents (.docx)
- Microsoft Excel spreadsheets (.xlsx)
- Microsoft PowerPoint presentations (.pptx)
- Plain text files
- Image files (requires OCR)