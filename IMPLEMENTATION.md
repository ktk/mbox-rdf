# Implementation Details

This tool is built in Rust to maximize performance and safety when processing large email archives.

## Core Components

- **mbox-reader**: Used for memory-mapped or streaming access to mbox files. This allows the tool to process files larger than available RAM without significant overhead.
- **mail-parser**: A high-performance, RFC-compliant email parser that handles multi-part messages, MIME encodings, and complex headers.
- **flate2**: Provides native Gzip compression for output files.
- **Vocab Module (`src/vocab.rs`)**: Manages IRI generation logic, ensuring deterministic and URL-safe identifiers for messages and addresses.
- **Schema Module (`src/schema.rs`)**: Handles the generation of the Turtle schema file.

## Performance Considerations

The tool minimizes allocations and uses buffered I/O throughout the pipeline:
1.  **Iterative Parsing**: Each message is read and parsed individually.
2.  **On-the-fly Serialization**: Triples/Quads are formatted and written to the output stream (potentially gzipped) immediately after a message is processed.
3.  **Zero-copy**: Where possible, data references the underlying buffers to avoid unnecessary string copying.
