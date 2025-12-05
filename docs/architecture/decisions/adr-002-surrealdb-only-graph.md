# ADR-002: SurrealDB-Only Graph Architecture (Alpha)

**Date**: 2025-12-02
**Status**: Accepted

## Context

We evaluated using `petgraph` (in-memory) vs `surrealdb` (persistent) for the GraphRAG engine. The core Alpha use case ("The Sarah Test") relies on **Local Search** (filtering edges by metadata, e.g., "exclude archived") rather than **Global Search** (community detection/clustering).

## Decision

We will use **SurrealDB exclusively** for the Alpha release:
- We will NOT include `petgraph` or maintain a separate in-memory graph structure
- All graph traversals will be executed via SurrealDB queries (or simple client-side logic on retrieved nodes)

## Consequences

### Pros
- Significantly simpler architecture (single source of truth, no sync logic)
- Lower RAM usage
- Faster implementation

### Cons
- Cannot perform efficient global graph algorithms (PageRank, Leiden) without pulling data into memory
- This is acceptable for Alpha scope
