# s3bat / s3cat

A command-line tool that combines AWS S3 file retrieval with syntax highlighting capabilities. Think of it as `cat` for S3 objects with beautiful syntax highlighting powered by [bat](https://github.com/sharkdp/bat).

## Features

-   Direct viewing of S3 objects with syntax highlighting
-   Automatic language detection based on file extension and content type
-   Built-in paging support (like `less`)
-   Optional line numbers
-   Custom syntax language specification
-   Detailed AWS error messages

## Installation

### Prerequisites

-   Rust and Cargo installed
-   AWS credentials configured (either through environment variables, AWS CLI configuration, or IAM role)

```bash
cargo install --git https://github.com/daemonp/s3bat
```

## Usage

Basic usage:

```bash
s3bat s3://bucket-name/path/to/file
```

### Options

-   `-l, --language <LANGUAGE>`: Specify the syntax highlighting language explicitly
-   `-n, --numbers`: Enable line numbers (disabled by default)

### Examples

View a file with automatic language detection:

```bash
s3bat s3://my-bucket/code/script.py
```

View multiple files:

```bash
s3bat s3://my-bucket/file1.js s3://my-bucket/file2.py
```

Specify language explicitly:

```bash
s3bat -l JavaScript s3://my-bucket/code.txt
```

Show line numbers:

```bash
s3bat -n s3://my-bucket/src/lib.rs
```
