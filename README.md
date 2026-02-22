# mbox-rdf

`mbox-rdf` is a Proof-of-Concept (PoC) tool that converts local email archives (mbox files) into RDF. This allows users to leverage the power of SPARQL to analyze and query their email history in ways that are typically impossible with standard desktop email clients.

## Motivation

For those familiar with RDF and Graph databases, the search capabilities in tools like Thunderbird or Outlook feel extremely limiting. Email is naturally a graph—subjects, senders, recipients, and threads create a complex web of information. By converting this data to RDF, you can answer complex questions about your communication patterns, social connectivity, and archive consistency using standard Semantic Web technologies.

Currently, the tool operates on local mbox files (such as those used by Thunderbird). In the future, it might be extended to fetch data directly via IMAP or JMAP to act as a live bridge between your mailbox and a triple store.

## Features

- **Extremely Fast**: Capable of converting multi-GB mailboxes in a matter of seconds or minutes.
- **Streaming Architecture**: Designed to handle archives of any size with minimal memory footprint.
- **Gzip & N-Quads**: Native support for compressed output and named graphs for multi-account analysis.
- **Timezone Aware**: Accurate `xsd:dateTime` literals preserving original sender offsets.
- **Automatic Schema**: Generates `mail-schema.ttl` to guide your analysis.

## Usage

### Basic conversion

```bash
cargo run --release -- path/to/INBOX --output mail.nt
```

### Advanced usage (N-Quads + Gzip)

```bash
cargo run --release -- path/to/INBOX \
  --schema-iri https://mail.described.at/ \
  --data-iri https://example.org/data/ \
  --graph-iri urn:email:user@example.com \
  --include-body \
  --output mail.nq.gz
```

### Options and Help

To see all available options, use the help command:

```bash
cargo run --release -- --help
```

The `--include-body` flag allows you to include the textual content of the email in the RDF output. This is useful for full-text search within your triple store, though it will significantly increase the size of the generated RDF file.

## Vocabulary

The tool generates a clean, flat vocabulary defined in `mail-schema.ttl`. Key concepts include:
- `mail:Message`: Individual email messages.
- `mail:Address`: Senders and recipients (associated with stable IRIs).
- `mail:Attachment`: Metadata about files within the messages.

## Analytics with SPARQL

Check `QUERIES.sparqlbook` for advanced query examples. You can run these using the [SPARQL Notebook](https://marketplace.visualstudio.com/items?itemName=Zazuko.sparql-notebook) extension for VS Code.
- Identifying stakeholders via CC analysis.
- Heuristic thread discovery.
- Hourly activity peaks.
- Cross-folder duplicate detection.

## Performance

## License

This project is licensed under the MIT License.

---
*Developed with the assistance of Google Antigravity*
