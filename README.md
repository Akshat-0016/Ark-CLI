# Ark CLI

> An Offline Intelligent Knowledge Retrieval Assistant built in Rust.

---

## Overview

Ark CLI is an offline-first command-line assistant that retrieves contextual information from locally stored documents.

Unlike cloud-based AI assistants, Ark uses Information Retrieval techniques such as lexical analysis, inverted indexing, BM25 ranking, and passage retrieval to provide accurate answers while keeping all data on the user's machine.

---

# Features

- Offline document indexing
- PDF parsing
- Natural Language Processing pipeline
- Tokenization
- Stemming
- Stopword removal
- Inverted Index
- Boolean Query Processing
- BM25 Ranking
- Passage Retrieval
- Query Cache
- Conversation Memory
- Fast CLI Interface

---

# Project Structure

```
Ark-CLI/
│
├── assistant-cli/
├── assistant-core/
│
├── docs/
│
├── Dockerfile
├── docker-compose.yml
├── README.md
└── .gitignore
```

---

# Architecture

```
                User
                  │
                  ▼
              Ark CLI
                  │
                  ▼
          Command Router
                  │
                  ▼
            Search Engine
                  │
     ┌────────────┴────────────┐
     ▼                         ▼
 NLP Pipeline           Inverted Index
     │                         │
     └────────────┬────────────┘
                  ▼
              BM25 Ranker
                  │
                  ▼
            Passage Ranking
                  │
                  ▼
          Context Generation
                  │
                  ▼
               CLI Output
```

---

# Vision

Ark aims to become an intelligent offline assistant capable of understanding and retrieving information from personal knowledge bases while maintaining complete user privacy.

---

# Target Users

- Students
- Developers
- Researchers
- Linux Power Users
- Technical Writers

---

# Current Features

- Offline Search
- Document Ranking
- Contextual Retrieval
- Query Caching
- Memory Support

---

# Future Roadmap

- Chunk-Level Indexing
- Semantic Search
- Knowledge Graph
- Plugin System
- Local Language Models
- Desktop GUI
- Voice Interface

---

# Success Metrics

- Search latency under 500ms
- Accurate top-3 document retrieval
- Offline functionality
- Low memory footprint
- Scalable indexing

---

# Assumptions

- Documents are stored locally.
- PDFs contain extractable text.
- Internet access is optional.

---

# Constraints

- Offline-first design
- No cloud dependency
- OCR not currently supported

---

# Quick Start – Local Development

## Clone

```bash
git clone https://github.com/Akshat-0016/Ark-CLI.git

cd Ark-CLI
```

## Build

```bash
cargo build
```

## Run

```bash
cargo run
```

## Docker

```bash
docker build -t ark .
```

```bash
docker run -it ark
```

or

```bash
docker compose up --build
```

---

# Development Tools

- Rust
- Cargo
- Docker
- Git
- GitHub
- VS Code / Cursor
- Draw.io
- Figma

---

# Branching Strategy

Ark follows **GitHub Flow**.

```
main
│
├── feature/search-engine
├── feature/rag
├── feature/cache
├── feature/memory
└── feature/indexing
```

Workflow:

```
Create Feature Branch
        │
        ▼
Develop
        │
        ▼
Commit
        │
        ▼
Pull Request
        │
        ▼
Review
        │
        ▼
Merge into main
```

---

# License

MIT License

---

## Author

Akshat
