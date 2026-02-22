# QLever for mbox-rdf

This directory contains the configuration for QLever to index and query your email RDF data.

## Prerequisites

Install QLever according to the [Quickstart Guide](https://docs.qlever.dev/quickstart/).

> [!TIP]
> **macOS Users**: It is highly recommended to use the **native** binary for the best performance. Docker is also available as an alternative option.

## Usage

### 1. Generate RDF Data
It is recommended to write your RDF data directly to `.gz` format to save significant disk space. You should automate the generation using a script.

**Example Generation Script:**
```bash
# Example for a single folder
cargo run --release -- path/to/INBOX \
  --data-iri https://data.zazuko.com/mbox/user/ \
  --graph-iri urn:email:user@example.com \
  --output qlever/mail.nq.gz
```

### 2. Index and Start
Once you have your `.nq.gz` files in this directory (or have updated the `INPUT_FILES` in `Qleverfile`), run the following commands:

```bash
qlever index
qlever start
qlever ui  # Optional: opens the QLever UI in your browser
```

## Configuration
The `Qleverfile` is configured to use `gzip -dc` for gzipped files, which is platform-independent and works on both macOS and Linux.
