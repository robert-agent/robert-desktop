# TODO

## In Progress
- [ ] Create robert-server project structure and Cargo.toml configuration

## Planned
- [ ] Phase 1: Foundation - Define core data models (RobertRequest, ClaudeEvent, etc.)
- [ ] Phase 1: Foundation - Write tests for request parsing/validation
- [ ] Phase 1: Foundation - Implement configuration loading
- [ ] Phase 1: Foundation - Write tests for config validation
- [ ] Phase 1: Foundation - Implement basic health endpoint
- [ ] Phase 1: Foundation - Write integration test for health endpoint
- [ ] Phase 2: Auth & Middleware - Write tests for token authentication
- [ ] Phase 2: Auth & Middleware - Implement auth middleware
- [ ] Phase 2: Auth & Middleware - Write tests for rate limiting
- [ ] Phase 2: Auth & Middleware - Implement rate limiting middleware
- [ ] Phase 2: Auth & Middleware - Integration tests for auth flow
- [ ] Phase 3: Claude CLI Integration - Write tests for mock executor
- [ ] Phase 3: Claude CLI Integration - Implement mock executor
- [ ] Phase 3: Claude CLI Integration - Write tests for real executor interface
- [ ] Phase 3: Claude CLI Integration - Implement claude-cli process spawning
- [ ] Phase 3: Claude CLI Integration - Implement stdout/stderr streaming
- [ ] Phase 3: Claude CLI Integration - Add timeout and cleanup logic
- [ ] Phase 4: Execute Endpoint - Write tests for execute endpoint with mock executor
- [ ] Phase 4: Execute Endpoint - Implement execute endpoint with SSE streaming
- [ ] Phase 4: Execute Endpoint - Write tests for session management
- [ ] Phase 4: Execute Endpoint - Implement session tracking and cancellation
- [ ] Phase 4: Execute Endpoint - Integration tests for full execute flow
- [ ] Phase 5: Production Readiness - Add TLS support
- [ ] Phase 5: Production Readiness - Add metrics endpoint
- [ ] Phase 5: Production Readiness - Performance benchmarks
- [ ] Phase 5: Production Readiness - Error handling polish
- [ ] Phase 5: Production Readiness - Documentation and README

## Completed
- [x] Read specification document
- [x] Set up TODO tracking

## Triage
- [ ] Docker Compose setup for local testing (optional for now)
- [ ] VS Code launch configuration (optional for now)
- [ ] Enterprise features (Phase 4 - future scope)

## Won't Fix
- [ ] OAuth2/mTLS advanced authentication (out of scope for MVP)
- [ ] Multi-tenancy support (future feature)
- [ ] HA deployment patterns (deployment concern, not implementation)
