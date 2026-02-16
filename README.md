# Ark-CLI

Ark is a fast, offline AI assistant built with Rust and Tauri.

It runs a local LLM with RAG (Retrieval-Augmented Generation), memory, file indexing, and PDF ingestion — all without cloud dependency.

---

## Features

-  Local LLM inference (llama3.2:3b)
-  File indexing + document retrieval
-  PDF parsing support
-  RAG pipeline
-  Memory caching layer
-  Offline-first architecture
-  Desktop UI built with Tauri

---

## Architecture

Frontend (Vite + JS)
->
Tauri bridge
->
Rust backend (assistant-core)
->
LLM + RAG + Cache + File Router

---

##  Tech Stack

- Rust
- Tauri
- Vite
- pnpm
- Local LLM (llama3)

---

##  Run Locally

### Backend

```bash
cd assistant-core
cargo build
cargo run
