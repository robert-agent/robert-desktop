# Architecture Decisions Log

## ADR-001: Core Crate Structure
**Date**: 2025-12-02
**Status**: Accepted

### Context
We are pivoting from a browser automation tool to a "Memory Layer" (ContextOS). We need a modular architecture to support GraphRAG, Context Management, and the Tauri App.

### Decision
We will split the backend into focused crates:
1.  `robert-graph`: Dedicated to GraphRAG implementation (Vector store + Graph store).
2.  `robert-core`: Contains the business logic, ContextOS state management, and ingestion pipelines.
3.  `robert-webdriver`: Retained for future agentic capabilities (browser control).
4.  `robert-app`: The Tauri application shell (UI + IPC layer).

### Consequences
- **Pros**: Clear separation of concerns. `robert-graph` can be tested in isolation (TDD). `robert-core` orchestrates logic without being tied to UI.
- **Cons**: Slightly more boilerplate in `Cargo.toml` workspace management.

## ADR-002: SurrealDB-Only Graph Architecture (Alpha)
**Date**: 2025-12-02
**Status**: Accepted

### Context
We evaluated using `petgraph` (in-memory) vs `surrealdb` (persistent) for the GraphRAG engine. The core Alpha use case ("The Sarah Test") relies on **Local Search** (filtering edges by metadata, e.g., "exclude archived") rather than **Global Search** (community detection/clustering).

### Decision
We will use **SurrealDB exclusively** for the Alpha release.
- We will NOT include `petgraph` or maintain a separate in-memory graph structure.
- All graph traversals will be executed via SurrealDB queries (or simple client-side logic on retrieved nodes).

### Consequences
- **Pros**: Significantly simpler architecture (single source of truth, no sync logic). Lower RAM usage. Faster implementation.
- **Cons**: Cannot perform efficient global graph algorithms (PageRank, Leiden) without pulling data into memory. This is acceptable for Alpha scope.

## ADR-003: Deprecate Robert Server in favor of Embedded Core
**Date**: 2025-12-02
**Status**: Accepted

### Context
`robert-server` was designed as a remote execution proxy to isolate `claude` CLI execution. However, the pivot to "ContextOS" (local-first memory layer) prioritizes local execution and simplicity. Running a separate server process adds unnecessary complexity for a local-first desktop app.

### Decision
We will **deprecate `robert-server`** and **port the Claude execution logic directly into `robert-core`**.
- `robert-core` will manage the `claude` CLI process directly (embedded).
- `robert-server` will be marked as deprecated and eventually removed.

### Consequences
- **Pros**: Simplified architecture (single process/binary), easier distribution, lower latency (no local network hop), better alignment with "local-first" vision.
- **Cons**: Loss of remote execution capability (can be re-added later if needed).

## ADR-004: Split-Brain Generation Architecture
**Date**: 2025-12-03
**Status**: Accepted

### Context
We need to balance privacy (PII protection), cost (token usage), and intelligence (reasoning capability). Sending all raw retrieved chunks to a cloud LLM is expensive and risky. Relying solely on a local LLM limits reasoning quality.

### Decision
We will adopt a **Split-Brain Architecture**:
1.  **Local "Little Brain"**: A small, efficient local model (Phi-4/Gemma 2) handles **Memory Access**, **Synthesis**, **PII Redaction**, and **Prompt Engineering**.
2.  **Cloud "Big Brain"**: A large remote model (GPT-4o/Claude 3.5) handles **Heavy Reasoning** and **Final Generation**, receiving only sanitized and optimized prompts.

### Consequences
- **Pros**: Maximizes privacy (PII stays local), minimizes cost (compressed prompts), maximizes quality (cloud reasoning).
- **Cons**: Requires local hardware acceleration (Metal on Mac). Adds complexity to the RAG loop.

## ADR-005: Adoption of Rig for Agentic RAG
**Date**: 2025-12-03
**Status**: Rejected (Superseded by ADR-007)

### Context
Building an Agentic RAG loop (planning, tool use, multi-step retrieval) from scratch is error-prone. We need a robust abstraction layer for LLM interactions and agent orchestration in Rust.

### Decision
We will use **`rig-core`** (the "LangChain for Rust") as our agent framework.
- It provides unified APIs for LLM providers (OpenAI, Cohere, Local).
- It has built-in support for RAG primitives and vector stores.
- It is type-safe and Rust-native.

### Consequences
- **Pros**: Accelerates development. Type safety reduces runtime errors. Unified interface for switching models.
- **Cons**: New dependency to learn and manage.
- **Update (2025-12-03)**: Rejected after evaluation. Rig's traits are overly restrictive for local-only loops using `candle` and `fastembed`, requiring excessive boilerplate.

## ADR-006: Local LLM as a Hard Requirement
**Date**: 2025-12-03
**Status**: Accepted

### Context
To support the "Split-Brain" architecture and "The Sarah Test" (privacy-first context control), we cannot rely purely on cloud APIs for the internal loop.

### Decision
A **Local LLM is now a hard requirement** for Robert.
- The app must bundle or download a small model (e.g., via Candle or by managing an Ollama instance) on first launch.
- Core features (Ingestion, PII stripping) will fail if local inference is unavailable.

### Consequences
- **Pros**: Enables true offline functionality and privacy guarantees.
- **Cons**: Increases app binary size (or initial download size). Higher system requirements (RAM/GPU).

## ADR-007: Custom Rust Agent for Local Loop
**Date**: 2025-12-03
**Status**: Accepted

### Context
We evaluated `rig-core` (ADR-005) for the local RAG loop. The evaluation revealed that adapting local Rust libraries (`candle`, `fastembed`) to Rig's REST-centric traits (`CompletionModel`, `EmbeddingModel`) requires significant boilerplate and introduces unnecessary friction.

### Decision
We will **DROP Rig** for the core local loop and implement a **Custom Rust Agent**.
- We will write a lightweight, type-safe loop directly using `candle-core` and `surrealdb`.
- We will prioritize direct control over the inference loop (for optimizations like KV-caching with Phi-4) over generic abstractions.

### Consequences
- **Pros**:
    - **Simplicity**: Less boilerplate, no "fighting the framework".
    - **Performance**: Direct control over `candle` tensors and generation loop.
    - **Flexibility**: Easier to integrate bleeding-edge models (Phi-4) without waiting for upstream library support.
- **Cons**: We must implement our own "Tool" traits and planning logic (which is acceptable for the scoped "Split-Brain" use case).


