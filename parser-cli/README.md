# Parser CLI

A simple command-line interface for the parser-core library, allowing you to parse various document formats from the command line.

## Usage

Parse one or more files and extract their text content to stdout:

```
parser-cli <FILES>...
```

Example:
```
parser-cli document.pdf presentation.pptx report.docx
```

## Supported File Types

- PDF documents
- Microsoft Word documents (.docx)
- Microsoft Excel spreadsheets (.xlsx)
- Microsoft PowerPoint presentations (.pptx)
- Plain text files
- Image files (requires OCR)