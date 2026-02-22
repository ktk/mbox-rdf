# TODO - Future Improvements

This list captures ideas for evolving `mbox-rdf` from a record-based converter to a fully interconnected graph model.

## Graph Model Enhancements

- **Proper Threading**: Map `In-Reply-To` and `References` headers to create explicit RDF relationships (e.g., `mail:inReplyTo`) between message objects. This replaces the current subject-line heuristic.
- **Mailing List Awareness**: Map `List-ID` and `List-Archive` headers to identify and group mailing list traffic.
- **Attachment Deduplication**: Hash attachment content (SHA-256) and use the hash in the IRI (e.g., `urn:hash:sha256:...`) to link identical files across different messages.
- **Hop Analysis**: Map the `Received` header chain to analyze mail delivery infrastructure and delays.
- **IMAP/JMAP Integration**: Extend the tool to fetch data directly from live mail servers rather than relying on local mbox files.

## Future / Out of Scope

- **Person Reconciliation**: Implement a layer to link multiple `mail:Address` nodes (different emails) to a single `mail:Person` node (planned to be done outside this converter).

## Technical Optimizations

- **QLever Full-Text**: Optimize the emission of `mail:bodyText` to play even better with QLever's text index capabilities.
- **Enhanced Address Parsing**: Handle grouped addresses and complex RFC 5322 edge cases in address fields more granularly.
