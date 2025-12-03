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

