# Implementation Plan - Robert Alpha (v0.5)

**Goal**: Deliver "ContextOS Core" - a local-first AI memory layer with GraphRAG, context control, and basic reasoning on macOS.

## User Review Required
> [!IMPORTANT]
> This plan overwrites the previous browser-automation focused plan. Please confirm this pivot is final.

## Proposed Changes

### Phase 1: Foundation & Project Setup
#### [MODIFY] [Project Structure](file:///Users/lucas/code/rust/robert/Cargo.toml)
- [x] Initialize Tauri 2.0 workspace (Exists as `robert-app`)
- [ ] Create new crate: `robert-graph` (GraphRAG engine)
- [ ] Create new crate: `robert-core` (Business logic & ContextOS)
- [ ] Register new crates in workspace `Cargo.toml`

#### [MODIFY] [Basic UI Shell](file:///Users/lucas/code/rust/robert/crates/robert-app/src/App.svelte)
- [x] Setup Svelte + Tailwind frontend (Exists)
- [ ] Refactor main layout: Add Context Sidebar (left) and Chat Area (center)
- [ ] Remove legacy browser automation UI elements


### Phase 2: Core Backend (The Memory Layer)
#### [NEW] [GraphRAG Engine](file:///Users/lucas/code/rust/robert/crates/robert-graph/src/lib.rs)
- [ ] **Dependencies**: Add `surrealdb` (embedded, kv-rocksdb), `fastembed`, `tokio`, `serde`.
- [ ] **GraphStore**: Implement `SurrealStore` struct.
    - [ ] Initialize embedded SurrealDB instance.
    - [ ] Define schema: `Node` (chunk), `Edge` (relation).
- [ ] **IngestionPipeline**: Text -> Chunks -> Embeddings -> SurrealDB.
- [ ] **GraphQuery**: Vector + Edge Traversal.
    - [ ] Vector Search (SurrealDB) -> Top K.
    - [ ] Filtered Traversal (e.g., "exclude archived").
    - [ ] Context Assembly.



#### [NEW] [File Ingestion](file:///Users/lucas/code/rust/robert/crates/robert-core/src/ingest.rs)
- [ ] File watcher for local folders
- [ ] Parsers for PDF, Markdown, TXT
- [ ] Basic entity extraction (using local LLM or simple heuristics for Alpha)

### Phase 3: ContextOS Features
#### [NEW] [Context Control](file:///Users/lucas/code/rust/robert/crates/robert-core/src/context.rs)
- [ ] Implement `Context` struct (id, name, rules)
- [ ] Implement `ContextManager`: Create/Delete/Switch contexts
- [ ] Add "Personal" and "Work" default contexts

#### [NEW] [Reactive Pruning](file:///Users/lucas/code/rust/robert/src/components/ContextSidebar.svelte)
- [ ] UI: "Mark as Outdated" button on sidebar items
- [ ] Backend: Update graph edge weights/status on prune action
- [ ] UI: Visual feedback for archived/outdated items

### Phase 4: Reasoning & Chat
#### [NEW] [Basic Reasoning](file:///Users/lucas/code/rust/robert/crates/robert-core/src/llm.rs)
- [ ] Integrate OpenAI API client (user provides key)
- [ ] Implement RAG loop: Query -> Graph Search -> Context Assembly -> Prompt -> LLM
- [ ] Chat UI: Message history, streaming responses

### Phase 5: Polish & Packaging
- [ ] E2E Testing of ingestion -> query loop
- [ ] macOS Bundle signing & notarization setup
- [ ] DMG creation

## Verification Plan

### Automated Tests
- Unit tests for `robert-graph` (graph construction, traversal)
- Integration tests for `IngestionPipeline` (file to graph)

### Manual Verification
- **The Sarah Test**: Ingest old vs new pricing docs, query, verify answer, mark old as outdated, verify answer changes.
- **Ingestion Test**: Import 50+ local files, verify graph structure.
