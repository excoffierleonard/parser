# Parser CLI

Command-line interface for the parser-core library, enabling text extraction from various document formats.

## Features

- Extract text from multiple files in a single command
- Support for all formats handled by parser-core
- Stream results to stdout for piping to other tools

## Installation

```bash
# From source
cargo install --path .

# Or within the workspace
cargo build -p parser-cli
```

## Usage

Parse one or more files and extract their text content to stdout:

```bash
parser-cli <FILES>...
```

Example:

```bash
parser-cli document.pdf presentation.pptx report.docx
```

## Integration

Useful in shell pipelines:

```bash
# Count words in a document
parser-cli document.pdf | wc -w

# Search for text in multiple documents
parser-cli *.pdf | grep "search term"
```