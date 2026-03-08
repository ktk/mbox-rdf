# mbox-rdf

`mbox-rdf` is a Proof-of-Concept (PoC) tool that converts local email archives (mbox files) into RDF. This allows users to leverage the power of SPARQL to analyze and query their email history in ways that are typically impossible with standard desktop email clients.

## Motivation

For those familiar with RDF and Graph databases, the search capabilities in tools like Thunderbird or Outlook feel extremely limiting. Email is naturally a graph—subjects, senders, recipients, and threads create a complex web of information. By converting this data to RDF, you can answer complex questions about your communication patterns, social connectivity, and archive consistency using standard Semantic Web technologies.

Currently, the tool operates on local mbox files (such as those used by Thunderbird). In the future, it might be extended to fetch data directly via IMAP or JMAP to act as a live bridge between your mailbox and a triple store.

## Features

- **Extremely Fast**: Capable of converting multi-GB mailboxes in a matter of seconds or minutes.
- **Streaming Architecture**: Designed to handle archives of any size with minimal memory footprint.
- **Gzip & N-Quads**: Native support for compressed output and named graphs for multi-account analysis.
- **Attachment Extraction**: Text attachments stored inline, binary attachments written to content-addressed files.
- **Timezone Aware**: Accurate `xsd:dateTime` literals preserving original sender offsets.
- **Schema.org Aligned**: Uses `schema:CreativeWork`, `schema:MediaObject`, `schema:dateCreated`, and `schema:name` for interoperability.
- **SHACL Shapes**: Includes `mail-shapes.ttl` for validation and documentation.

## Usage

### 🆕 Recommended Workflow: Config-Driven Conversion

The easiest way to map a full Thunderbird profile is through the configuration-driven workflow.

**1. Discover Accounts**
```bash
cargo run --release -- discover
```
This scans your local Thunderbird profile, reads `prefs.js` to map local paths to email identities, and generates an `mbox-config.toml` file.

**2. Configure Extraction**
Open `mbox-config.toml` to customize your extraction. You can enable full-text bodies or file attachments per-account:
```toml
[accounts."alice@example.com"]
email = "alice@example.com"
graph = "urn:email:alice@example.com"
data_iri = "https://data.example.com/"
include_body = true
include_attachments = true
attachment_dir = "attachments"

[[accounts."alice@example.com".folders]]
name = "INBOX"
path = "/path/to/thunderbird/ImapMail/mail.example.com/INBOX"
include = true
```

**3. Convert**
```bash
cargo run --release -- convert
```
This processes all `include = true` folders in parallel, creating isolated `.nq.gz` files per folder. By default, if a `qlever_dir` is defined in settings, it will automatically restart and re-index your QLever SPARQL endpoint with the new data.

---

### Legacy Workflow: Single File Conversion

If you want to manually pipe a single `.mbox` file to RDF, you can use the legacy conversion mode:

```bash
cargo run --release -- path/to/INBOX --output mail.nt
```

**Advanced usage (N-Quads + Gzip):**
```bash
cargo run --release -- path/to/INBOX \
  --data-iri https://example.org/data/ \
  --graph-iri urn:email:user@example.com \
  --include-body \
  --output mail.nq.gz
```

**With attachment extraction:**
```bash
cargo run --release -- path/to/INBOX \
  --include-attachments \
  --max-attachment-size 1048576 \
  --attachment-dir ./attachments \
  --output mail.nq.gz
```

Text attachments (`text/*`) are stored inline as `schema:text` literals. Binary attachments are written to `{attachment-dir}/{sha256}.{ext}` and referenced via `schema:contentUrl`.

**Single-file Options:**
```
-o, --output              Output path (.nt or .nq, optionally .gz) [default: mail.nt]
    --data-iri            Base IRI for instance data [default: urn:mbox:]
    --graph-iri           Optional Graph IRI (enables N-Quads)
    --gzip                Force Gzip compression
    --include-body        Include body text
    --include-attachments Include attachment content (text inline, binary to files)
    --max-attachment-size  Max attachment size in bytes to include
    --attachment-dir      Directory for extracted attachments [default: attachments]
    --folder-name         Folder name override (default: derived from filename)
    --limit               Limit number of messages to process
```

## Vocabulary

The schema namespace is `https://mail.described.at/`. Key concepts:

- **`mail:Message`** (also `schema:CreativeWork`): Individual email messages.
- **`mail:Account`**: Email accounts, identified by `mailto:` URIs (e.g., `<mailto:alice@example.com>`).
- **`mail:Thread`**: Conversation threads, derived from the `References` header chain.
- **`mail:MailingList`**: Mailing lists with `mail:listId`.
- **`schema:MediaObject`**: File attachments with `schema:sha256`, `schema:encodingFormat`, `schema:name`, `schema:text`, `schema:contentUrl`.

Dates use `schema:dateCreated` and `schema:dateReceived`. Display names use `schema:name`.

Schema files:
- `mail-schema.ttl` — RDFS class and property definitions
- `mail-shapes.ttl` — SHACL validation shapes with cardinality constraints
- `SPARQL_INSTRUCTIONS.md` — Complete reference for LLMs generating SPARQL queries

## LLM Integration

Point your LLM agent at [`SPARQL_INSTRUCTIONS.md`](SPARQL_INSTRUCTIONS.md) — it contains the complete schema reference with all classes, properties, cardinalities, and example query patterns. Enough for an LLM to generate correct SPARQL queries without needing to read the source code.

## SPARQL Queries

Queries live as standalone `.rq` files in `sparql/`. The `QUERIES.sparqlbook` references them for use with the [SPARQL Notebook](https://marketplace.visualstudio.com/items?itemName=Zazuko.sparql-notebook) VS Code extension.

Run the smoke test to verify all queries return results:

```bash
SPARQL_ENDPOINT=http://localhost:7029 ./sparql/smoke-test.sh
```

## Indexing and Querying with QLever

For large archives, we recommend using [QLever](https://github.com/ad-freiburg/qlever). It provides a high-performance SPARQL engine that can handle millions of triples with ease.

Detailed instructions and a pre-configured `Qleverfile` can be found in the [qlever/](qlever/) directory.

### Quick Start with QLever
```bash
cd qlever
qlever index  # Indexes the generated .nq.gz files
qlever start  # Starts the SPARQL endpoint
```

## Roadmap

- [ ] **Incremental sync** — track the last processed byte offset per mbox file (stored in QLever as RDF), process only new messages, and INSERT DATA via SPARQL UPDATE. Full reindex is fast enough as a fallback.
- [ ] **Text search** — implement QLever materialized views with `ql:has-word` for ranked keyword search over subjects (weight 5) and body text (weight 1). Waiting on [QLever PR #2579](https://github.com/ad-freiburg/qlever/pull/2579).
- [ ] **URL extraction** — re-enable `mail:linksTo` by parsing `<a href>` from HTML parts instead of regex on plain text.

## License

This project is licensed under the MIT License.

---
*Developed with the assistance of Google Antigravity*
