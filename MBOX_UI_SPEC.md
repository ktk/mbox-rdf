# Specification: mbox-rdf-ui (Refined)

A modern frontend for managing and querying email archives converted via `mbox-rdf`.

## Core Features

### 1. Mbox Management & Pipeline
- **Auto-Discovery**: Support for scanning local Thunderbird profiles.
- **Selection UI**:
    - Hierarchical tree view. Smart defaults: Auto-exclude `Trash`, `Junk`, and `Archives`. Include `INBOX` and `Sent`.
    - **Configuration Persistence**: Save all user selections and folder mappings as a **local Turtle (.ttl) file**.
- **Automated Backend**: 
    - Integrates with the [ktk/mbox-rdf](https://github.com/ktk/mbox-rdf) repository.
    - Automates binary management (download or `cargo build`) so the user avoids manual Rust configuration.

### 2. Search & Query Interface
- **Modern Search**: Modern UI mimicking Thunderbird's advanced search but powered by QLever.
- **SPARQL Mode (YASGUI)**:
    - **RDF Query Storage**: Enable saving custom queries.
    - **Initial Seed**: Pre-load with the 12 sample queries from the `mbox-rdf` project.
    - **Storage**: Query configurations stored in a simple RDF format.
- **LLM Question Answering**:
    - Natural language to SPARQL bridge.
    - **Context Awareness**: Injects schema and few-shot examples from `mail-schema.ttl` and `QUERIES.sparqlbook` (referenced from the `ktk/mbox-rdf` repo).

### 3. Synchronization & Metadata
- **Smart Sync**: File system watcher with configurable intervals (e.g., check every X minutes) or triggered QLever inserts to avoid constant re-indexing.
- **Thunderbird Integration**: Links (`mid:`) open directly in the desktop client.
- **Schema Extensions (Future)**: 
    > [!NOTE]
    > While "tagging" is a goal, reading Thunderbird's internal tags is complex and will be explored as a secondary phase.

## Technical Stack
- **Framework**: Angular or Vanilla Web Components. **No React.**
- **Shell**: Node.js/Electron.
- **Backend**: [QLever](https://github.com/ad-freiburg/qlever).

---
*Refined based on user feedback - Feb 2026*
