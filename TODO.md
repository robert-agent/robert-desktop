# Robert Implementation TODO

This document tracks the implementation gaps between the current codebase and the architecture defined in the ADRs.

## Status Legend
- ❌ Not Started
- 🚧 Partially Implemented
- ✅ Complete

---

## Layer 1: robert-graph (Database Layer)

### Status: 🚧 Partially Implemented

#### Implemented
- ✅ GraphStore and VectorStore traits defined
- ✅ Mock implementations for testing
- ✅ Basic Node and Edge types
- ✅ Ephemeral graph module (petgraph integration)
- ✅ SurrealDB store module structure

#### Missing/Incomplete
- ❌ **SurrealDB GraphStore implementation**
  - File: `crates/robert-graph/src/surreal_store.rs`
  - Need: Full implementation of GraphStore trait for SurrealDB
  - Need: Connection management, database initialization

- ❌ **SurrealDB VectorStore implementation**
  - File: `crates/robert-graph/src/surreal_store.rs`
  - Need: Vector similarity search using SurrealDB
  - Need: FastEmbed integration for embedding generation

- ❌ **Graph Partitioning**
  - Need: Support for `graph_id` field (Personal/Work/Business partitions)
  - Need: Partition-aware queries (filter by graph_id)
  - Need: Update Node and Edge types to include partition info

- ❌ **Ingestion Pipeline**
  - File: `crates/robert-graph/src/ingest.rs`
  - Need: Document chunking logic
  - Need: Entity extraction from chunks
  - Need: Relationship inference

- ❌ **Query Module**
  - File: `crates/robert-graph/src/query.rs`
  - Need: GraphRAG query implementation
  - Need: Vector + Graph hybrid search
  - Need: Metadata filtering ("exclude archived")

---

## Layer 2: robert-core (Business Logic)

### Status: 🚧 Partially Implemented

#### Implemented
- ✅ ContextManager for Personal/Work contexts
- ✅ ClaudeClient wrapper
- ✅ LlmClient abstraction (currently Claude-only)
- ✅ Basic module structure

#### Missing/Incomplete

### Split-Brain Architecture (ADR-004)

- ❌ **Local "Little Brain" LLM**
  - File: `crates/robert-core/src/llm/local.rs`
  - Need: Candle integration for local model loading (Phi-4/Gemma 2)
  - Need: Model download/caching logic
  - Need: `synthesize(chunks) -> summary` function
  - Need: `extract_pii(text) -> (redacted_text, pii_map)` function
  - Need: `optimize_prompt(query, context) -> compressed_prompt` function

- ❌ **Cloud "Big Brain" Integration**
  - File: `crates/robert-core/src/llm/cloud.rs` (new file)
  - Need: Multi-provider support (OpenAI, Anthropic)
  - Need: Provider switching logic
  - Need: Token counting and cost tracking

### Agentic RAG Loop (ADR-007)

- ❌ **Custom Agent Implementation**
  - File: `crates/robert-core/src/agent.rs` (new file)
  - Need: Agent struct with tool support
  - Need: Tool trait definition
  - Need: `search_graph(query, partition)` tool
  - Need: `read_file(path)` tool
  - Need: Planning logic (which partitions to search)
  - Need: Synthesis and redaction pipeline
  - Need: Decision logic (local vs cloud generation)

### Document Ingestion

- ❌ **File Ingestion**
  - File: `crates/robert-core/src/ingest.rs` (new file)
  - Need: File watcher for local folders
  - Need: PDF parser integration
  - Need: Markdown parser
  - Need: Text file parser
  - Need: Local LLM entity extraction during ingestion

### Memory Management

- ❌ **Hierarchical Memory (Hot/Warm/Cold)**
  - File: `crates/robert-core/src/memory.rs` (new file)
  - Need: Tier definitions and policies
  - Need: Automatic tier transitions
  - Need: Recency tracking

- ❌ **Reactive Pruning**
  - File: `crates/robert-core/src/pruning.rs`
  - Need: "Mark as Outdated" functionality
  - Need: Update graph edge weights
  - Need: Status flags (active/archived)

### Search and Retrieval

- ❌ **GraphRAG Search**
  - File: `crates/robert-core/src/search.rs`
  - Need: Integration with robert-graph query module
  - Need: Context-aware search (partition filtering)
  - Need: Ranking and scoring logic

---

## Layer 3: robert-server (REST API)

### Status: 🚧 Partially Implemented

#### Implemented
- ✅ Warp server setup
- ✅ Authentication middleware
- ✅ Session management
- ✅ ClaudeExecutor for claude-cli integration
- ✅ Health endpoints

#### Missing/Incomplete

- ❌ **robert-core Integration**
  - Current: Server only wraps claude-cli
  - Need: Instantiate and manage robert-core instances
  - Need: Route API calls to robert-core methods

- ❌ **GraphRAG API Endpoints**
  - File: `crates/robert-server/src/api/graphrag.rs` (new file)
  - Need: `POST /ingest` - Ingest documents
  - Need: `POST /query` - Query knowledge graph
  - Need: `GET /contexts` - List available contexts
  - Need: `POST /contexts/:id/activate` - Switch context

- ❌ **Memory Management Endpoints**
  - File: `crates/robert-server/src/api/memory.rs` (new file)
  - Need: `POST /nodes/:id/prune` - Mark as outdated
  - Need: `GET /nodes/:id/attribution` - Get source documents
  - Need: `GET /memory/status` - Memory tier statistics

- ❌ **Agent Execution Endpoints**
  - File: `crates/robert-server/src/api/agent.rs` (new file)
  - Need: `POST /agent/execute` - Run agentic RAG loop
  - Need: `GET /agent/status/:session_id` - Check execution status
  - Need: Streaming response support for agent steps

- ❌ **Server Configuration**
  - Need: Add robert-core initialization to server startup
  - Need: Pass GraphStore instance to core
  - Need: Configure local LLM models path

---

## Layer 4: robert-app (Desktop UI)

### Status: 🚧 Partially Implemented

#### Implemented
- ✅ Tauri application structure
- ✅ Claude health checking
- ✅ Browser/webdriver integration
- ✅ User profile management
- ✅ Feedback dialog

#### Missing/Incomplete

- ❌ **robert-server Lifecycle Management**
  - File: `crates/robert-app/src-tauri/src/server.rs` (new file)
  - Need: Spawn local robert-server on app start
  - Need: Monitor server health
  - Need: Graceful shutdown on app exit
  - Need: Server restart logic

- ❌ **GraphRAG Chat Interface**
  - Current: Chat uses standalone webdriver workflows
  - Need: Commands to call robert-server GraphRAG endpoints
  - Need: Context switching UI
  - Need: Transparent attribution display

- ❌ **Context Management UI**
  - File: `crates/robert-app/src-tauri/src/commands/context.rs` (new file)
  - Need: `get_contexts` command
  - Need: `set_active_context` command
  - Need: `create_context` command

- ❌ **Document Ingestion UI**
  - File: `crates/robert-app/src-tauri/src/commands/ingest.rs` (new file)
  - Need: `ingest_file` command
  - Need: `ingest_folder` command
  - Need: Ingestion progress events

- ❌ **Memory Control UI**
  - File: `crates/robert-app/src-tauri/src/commands/memory.rs` (new file)
  - Need: `mark_as_outdated` command
  - Need: `get_attribution` command
  - Need: Graph visualization commands

- ❌ **Frontend Components** (Svelte)
  - File: `crates/robert-app/src/components/ContextSelector.svelte` (new)
  - File: `crates/robert-app/src/components/GraphViewer.svelte` (new)
  - File: `crates/robert-app/src/components/AttributionSidebar.svelte` (new)
  - Need: Update ChatInterface to use GraphRAG endpoints

---

## Layer 5: robert-cli (Command Line Interface)

### Status: ❌ Not Started

Current: Only has placeholder code

#### All To Be Implemented

- ❌ **Server Management Commands**
  - File: `crates/robert-cli/src/server.rs` (new file)
  - Need: `robert server start` - Start robert-server
  - Need: `robert server stop` - Stop robert-server
  - Need: `robert server status` - Check server health
  - Need: `robert server logs` - View server logs

- ❌ **Ingestion Commands**
  - File: `crates/robert-cli/src/ingest.rs` (new file)
  - Need: `robert ingest <path>` - Ingest file or folder
  - Need: `robert ingest --watch <path>` - Watch folder for changes
  - Need: `robert ingest list` - List ingested documents

- ❌ **Query Commands**
  - File: `crates/robert-cli/src/query.rs` (new file)
  - Need: `robert query <question>` - Query knowledge base
  - Need: `robert query --context <context_id>` - Query specific context
  - Need: `robert search <term>` - Search documents

- ❌ **Context Commands**
  - File: `crates/robert-cli/src/context.rs` (new file)
  - Need: `robert context list` - List contexts
  - Need: `robert context create <name>` - Create context
  - Need: `robert context switch <id>` - Switch active context
  - Need: `robert context delete <id>` - Delete context

- ❌ **Memory Commands**
  - File: `crates/robert-cli/src/memory.rs` (new file)
  - Need: `robert memory status` - View memory statistics
  - Need: `robert memory prune <node_id>` - Mark as outdated
  - Need: `robert memory export <path>` - Export knowledge graph

- ❌ **Model Management**
  - File: `crates/robert-cli/src/models.rs` (new file)
  - Need: `robert models list` - List downloaded models
  - Need: `robert models download <model_name>` - Download local model
  - Need: `robert models delete <model_name>` - Delete model

- ❌ **Configuration**
  - File: `crates/robert-cli/src/config.rs` (new file)
  - Need: Config file management (~/.robert/config.toml)
  - Need: API token configuration
  - Need: Server URL configuration (local vs remote)

---

## Shared: robert-types

### Status: 🚧 Partially Implemented

#### Implemented
- ✅ User profile types
- ✅ Authentication types
- ✅ Cryptography utilities

#### Missing/Incomplete

- ❌ **GraphRAG Types**
  - File: `crates/types/src/graphrag.rs` (new file)
  - Need: Shared request/response types for GraphRAG API
  - Need: Query types
  - Need: Ingestion types

- ❌ **Context Types**
  - File: `crates/types/src/context.rs` (new file)
  - Need: Context definition types
  - Need: Partition types

- ❌ **Memory Types**
  - File: `crates/types/src/memory.rs` (new file)
  - Need: Memory tier types
  - Need: Attribution types

---

## Critical Path for Alpha (v0.5)

Based on ADRs and implementation plan, the critical path is:

### Phase 1: Foundation (Week 1-2)
1. ✅ Core Crate Structure (ADR-001) - DONE
2. 🚧 SurrealDB GraphStore implementation - **PRIORITY 1**
3. 🚧 SurrealDB VectorStore with FastEmbed - **PRIORITY 1**
4. ❌ Graph partitioning (Personal/Work) - **PRIORITY 2**

### Phase 2: Local Intelligence (Week 3-4)
5. ❌ Candle integration for local LLM (Phi-4) - **PRIORITY 1**
6. ❌ Local model download/caching - **PRIORITY 1**
7. ❌ PII redaction implementation - **PRIORITY 1**
8. ❌ Prompt synthesis and optimization - **PRIORITY 2**

### Phase 3: RAG Loop (Week 5-6)
9. ❌ Custom Agent implementation - **PRIORITY 1**
10. ❌ Document ingestion pipeline - **PRIORITY 1**
11. ❌ GraphRAG query logic - **PRIORITY 1**
12. ❌ Split-brain execution flow (local → cloud) - **PRIORITY 2**

### Phase 4: Integration (Week 7-8)
13. ❌ robert-server GraphRAG endpoints - **PRIORITY 1**
14. ❌ robert-app server lifecycle management - **PRIORITY 1**
15. ❌ robert-app GraphRAG commands - **PRIORITY 2**
16. ❌ Reactive pruning UI ("Mark as Outdated") - **PRIORITY 2**

### Phase 5: CLI & Polish (Week 9-10)
17. ❌ robert-cli basic commands (ingest, query) - **PRIORITY 2**
18. ❌ Context switching UI - **PRIORITY 2**
19. ❌ Graph visualization - **PRIORITY 3**
20. ❌ Attribution sidebar - **PRIORITY 3**

---

## Known Issues / Blockers

### Dependency Conflicts
- ❌ async-openai: Currently commented out in robert-core/src/llm/mod.rs
  - Need to add `_api` feature flag or fix dependency version

### Architecture Misalignments
- ✅ robert-server now depends on robert-core (FIXED)
- ✅ robert-cli now has necessary dependencies (FIXED)
- ❌ robert-app still connects to standalone webdriver server, needs to use robert-server
- ❌ robert-server still only wraps claude-cli, doesn't use robert-core

### Missing Infrastructure
- ❌ No model download mechanism yet
- ❌ No SurrealDB initialization/migration scripts
- ❌ No local storage path conventions (~/.robert/data, ~/.robert/models, etc.)

---

## Testing Requirements

### Unit Tests Needed
- ❌ robert-graph: SurrealDB GraphStore/VectorStore
- ❌ robert-core: Local LLM inference
- ❌ robert-core: PII redaction
- ❌ robert-core: Agent tool execution
- ❌ robert-server: GraphRAG endpoints

### Integration Tests Needed
- ❌ End-to-end ingestion → query flow
- ❌ Context switching with partition isolation
- ❌ Split-brain execution (local + cloud)
- ❌ "The Sarah Test" (mark document as outdated, verify not used)

### Manual Verification Needed
- ❌ Local model download and initialization
- ❌ Desktop app server lifecycle
- ❌ CLI commands
- ❌ Graph visualization

---

## Documentation Needed

- ❌ API documentation for robert-server endpoints
- ❌ CLI command reference
- ❌ Local model setup guide
- ❌ Database schema documentation
- ❌ Agent workflow documentation
- ❌ Developer setup guide (building from source)

---

## Notes

This TODO represents a comprehensive Alpha (v0.5) implementation. Some features may be deferred to v1.0:
- Advanced graph algorithms (PageRank, community detection)
- Team knowledge graphs and sharing
- E2E encrypted cloud sync
- Mobile apps (iOS/Android)
- Enterprise audit trails

The focus is on proving the core ContextOS concept:
1. Local-first GraphRAG with partitioning
2. Split-brain privacy architecture
3. Transparent attribution and reactive pruning
4. Provider-agnostic reasoning
