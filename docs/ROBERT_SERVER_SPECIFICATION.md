# Robert Server Specification

## Overview

The Robert Server is a Rust-based web service built with the Warp framework that receives SSL-encrypted streamed content from the Robert desktop application. It acts as a remote execution proxy, forwarding requests to the Claude CLI tool and streaming responses back to the client.

## Architecture

```
┌─────────────────┐         HTTPS/WSS          ┌──────────────────┐
│   Robert App    │ ◄─────────────────────────► │  Robert Server   │
│   (Desktop)     │   (SSL Encrypted Stream)    │   (Rust/Warp)    │
└─────────────────┘                             └──────────────────┘
                                                          │
                                                          │ Spawns/IPC
                                                          ▼
                                                 ┌──────────────────┐
                                                 │   claude-cli     │
                                                 │   (Headless)     │
                                                 └──────────────────┘
```

## Key Requirements

### 1. Network Protocol

- **Transport**: HTTPS with WebSocket upgrade for streaming
- **SSL/TLS**: Mandatory TLS 1.3 encryption
- **Authentication**: Bearer token or API key based authentication
- **Port**: Configurable (default: 8443)

### 2. Request Flow

1. Robert app establishes SSL connection to robert-server
2. Robert app sends CDP automation context and user prompt
3. Server validates authentication and request format
4. Server spawns headless claude-cli process
5. Server streams request data to claude-cli stdin
6. Server captures claude-cli stdout/stderr
7. Server streams responses back to Robert app
8. Connection remains open until completion or timeout

### 3. API Endpoints

#### `POST /api/v1/execute`

Initiates a new Robert execution session with streaming response.

**Request Headers:**
```
Authorization: Bearer <token>
Content-Type: application/json
Accept: text/event-stream
```

**Request Body:**
```json
{
  "session_id": "uuid",
  "context": {
    "screenshots": [
      {
        "timestamp": "2025-10-17T10:30:00Z",
        "image_data": "base64_encoded_png",
        "metadata": {
          "window_title": "string",
          "url": "string",
          "viewport": {"width": 1920, "height": 1080}
        }
      }
    ],
    "dom_state": {
      "accessible_tree": "...",
      "interactive_elements": []
    },
    "user_intent": "string"
  },
  "prompt": "string",
  "options": {
    "timeout_seconds": 300,
    "max_tokens": 100000,
    "stream": true
  }
}
```

**Response (Server-Sent Events):**
```
event: tool_use
data: {"tool": "cdp_command", "params": {...}}

event: content
data: {"type": "text", "text": "Processing..."}

event: error
data: {"code": "EXECUTION_ERROR", "message": "..."}

event: complete
data: {"session_id": "uuid", "status": "success"}
```

#### `GET /api/v1/health`

Health check endpoint.

**Response:**
```json
{
  "status": "healthy",
  "version": "1.0.0",
  "claude_cli_available": true,
  "uptime_seconds": 12345
}
```

#### `GET /api/v1/sessions/:session_id`

Query status of an active or completed session.

**Response:**
```json
{
  "session_id": "uuid",
  "status": "running|completed|failed",
  "started_at": "2025-10-17T10:30:00Z",
  "completed_at": "2025-10-17T10:35:00Z",
  "error": null
}
```

#### `DELETE /api/v1/sessions/:session_id`

Cancel a running session.

**Response:**
```json
{
  "session_id": "uuid",
  "status": "cancelled"
}
```

### 4. Payload Format

The payload structure remains **unchanged** from the current Robert app implementation:

- Screenshot data format (base64 PNG)
- DOM/accessibility tree structure
- CDP command parameters
- Tool use formats
- Response streaming events

The only difference is **transport mechanism** - data is sent over HTTPS instead of local IPC.

### 5. Claude CLI Integration

#### Execution Model

```rust
// Pseudo-code representation
async fn execute_claude_cli(request: RobertRequest) -> Stream<ClaudeEvent> {
    let mut child = Command::new("claude")
        .arg("--headless")
        .arg("--stream")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Write request to stdin
    write_request_to_stdin(&mut child.stdin, request).await?;

    // Stream stdout/stderr back to client
    stream_output(&mut child.stdout, &mut child.stderr).await
}
```

#### Claude CLI Requirements

- Must be installed on server (`claude-cli` in PATH)
- Headless mode support (no interactive prompts)
- JSON input/output mode
- Streaming output support
- Process isolation per request

### 6. Security Considerations

#### Authentication

- API key/bearer token validation on every request
- Rate limiting per token (configurable)
- IP whitelisting (optional)
- Token rotation support

#### Input Validation

- Maximum request size limits (default: 50MB)
- Screenshot count limits (default: 10 per request)
- Prompt length validation
- JSON schema validation

#### Process Isolation

- Each claude-cli execution runs in isolated process
- Resource limits (CPU, memory, file descriptors)
- Timeout enforcement (default: 5 minutes)
- Automatic cleanup of zombie processes

#### Data Privacy

- No persistent storage of screenshots or prompts
- Temporary files cleaned immediately after use
- Optional request/response logging (sanitized)
- Configurable log retention

### 7. Performance Requirements

- **Concurrent Sessions**: Support 10-50 concurrent executions
- **Latency**: <100ms overhead for stream forwarding
- **Throughput**: Handle 10MB/s per connection
- **Memory**: <100MB base + <200MB per active session
- **CPU**: Efficient async I/O, minimal CPU overhead

### 8. Configuration

```toml
[server]
host = "0.0.0.0"
port = 8443
tls_cert = "/path/to/cert.pem"
tls_key = "/path/to/key.pem"

[auth]
tokens = ["token1", "token2"]
rate_limit_per_minute = 10

[claude]
binary_path = "claude"
default_timeout_seconds = 300
max_concurrent_sessions = 20

[limits]
max_request_size_mb = 50
max_screenshot_count = 10
max_prompt_length = 50000

[logging]
level = "info"
format = "json"
sanitize_sensitive_data = true
```

### 9. Error Handling

#### Error Types

- `AUTH_FAILED`: Invalid or missing authentication
- `RATE_LIMITED`: Too many requests
- `INVALID_REQUEST`: Malformed request body
- `CLAUDE_UNAVAILABLE`: claude-cli not found or not responding
- `EXECUTION_ERROR`: claude-cli process failed
- `TIMEOUT`: Request exceeded timeout limit
- `INTERNAL_ERROR`: Server-side error

#### Error Response Format

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable message",
    "session_id": "uuid",
    "timestamp": "2025-10-17T10:30:00Z",
    "retry_after_seconds": 60
  }
}
```

### 10. Implementation Dependencies

```toml
[dependencies]
warp = "0.3"
tokio = { version = "1.0", features = ["full"] }
tokio-stream = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio-tungstenite = "0.21" # WebSocket support
tokio-rustls = "0.25"
uuid = { version = "1.0", features = ["v4"] }
tracing = "0.1"
tracing-subscriber = "0.3"
tower = "0.4" # Middleware support
tower-http = { version = "0.5", features = ["cors", "compression", "trace"] }
bytes = "1.0"
futures = "0.3"
base64 = "0.21"
```

### 11. Deployment Considerations

#### Docker Support

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates curl
# Install claude-cli
RUN curl -O https://... && install claude /usr/local/bin/
COPY --from=builder /app/target/release/robert-server /usr/local/bin/
EXPOSE 8443
CMD ["robert-server"]
```

#### Monitoring

- Prometheus metrics endpoint (`/metrics`)
- Health check endpoint for load balancers
- Structured logging (JSON format)
- Request tracing with correlation IDs

#### Scaling

- Stateless design enables horizontal scaling
- Load balancer distributes requests across instances
- Session affinity not required
- Shared rate limiting via Redis (optional)

### 12. Local Development & Testing

#### Development Server

For local development and testing, robert-server supports running on localhost with relaxed security:

```bash
# Start local development server
cargo run --bin robert-server -- --dev

# Or with explicit configuration
cargo run --bin robert-server -- \
  --host localhost \
  --port 8443 \
  --dev-mode \
  --no-tls
```

**Development Mode Features:**
- HTTP support (no TLS required for localhost)
- Simplified authentication (single static token or disabled)
- Verbose logging enabled by default
- CORS enabled for all origins
- Auto-reload on configuration changes
- Mock mode for claude-cli (optional)

#### Configuration: `config.dev.toml`

```toml
[server]
host = "127.0.0.1"
port = 8443
dev_mode = true
enable_tls = false  # Use HTTP for localhost testing

[auth]
dev_token = "dev-token-12345"
require_auth = false  # Optional: disable auth for local testing

[claude]
binary_path = "claude"
mock_mode = false  # Set to true to use mock responses
default_timeout_seconds = 300

[logging]
level = "debug"
pretty_print = true
```

#### Integration with Tauri App

The Robert Tauri app should include a command to start/stop local robert-server:

```bash
# Tauri app commands
npm run dev                    # Start Tauri app (points to remote server)
npm run dev:local              # Start Tauri app with local server
npm run server:start           # Start local robert-server only
npm run server:stop            # Stop local robert-server

# Or using cargo directly from project root
cargo run --bin robert-server -- --dev
```

**Tauri Configuration:**

```json
// package.json
{
  "scripts": {
    "dev": "tauri dev",
    "dev:local": "concurrently \"npm run server:start\" \"tauri dev --config tauri.local.json\"",
    "server:start": "cd ../robert-server && cargo run -- --dev",
    "server:stop": "pkill -f robert-server",
    "test:e2e": "npm run server:start && playwright test && npm run server:stop",
    "test:integration": "cd ../robert-server && cargo test --features integration"
  }
}
```

```json
// tauri.local.json
{
  "build": {
    "beforeDevCommand": "",
    "beforeBuildCommand": ""
  },
  "robert": {
    "server_url": "http://localhost:8443",
    "api_token": "dev-token-12345"
  }
}
```

#### Mock Mode for Testing

For rapid testing without claude-cli dependency:

```rust
// robert-server/src/mock.rs
pub struct MockClaudeExecutor;

impl MockClaudeExecutor {
    pub async fn execute(&self, request: RobertRequest) -> impl Stream<Item = ClaudeEvent> {
        // Return predefined responses for testing
        stream! {
            yield ClaudeEvent::Content {
                text: "Mock response: Analyzing screenshot...".to_string()
            };

            yield ClaudeEvent::ToolUse {
                tool: "cdp_command".to_string(),
                params: json!({"command": "click", "selector": "#button"})
            };

            yield ClaudeEvent::Complete {
                session_id: request.session_id,
                status: "success".to_string()
            };
        }
    }
}
```

Enable mock mode:
```bash
cargo run --bin robert-server -- --dev --mock
```

#### Testing Setup

**Unit Tests:**
```bash
cargo test --lib
```

**Integration Tests** (requires robert-server running):
```bash
# Terminal 1: Start server
cargo run --bin robert-server -- --dev

# Terminal 2: Run integration tests
cargo test --test integration_tests -- --test-threads=1
```

**E2E Tests** (requires both server and Tauri app):
```bash
# Automated via npm script
npm run test:e2e

# Or manually
# Terminal 1: Start robert-server
cargo run --bin robert-server -- --dev

# Terminal 2: Run Playwright/Tauri tests
npm run tauri test
```

#### Test Environment Variables

```bash
# .env.test
ROBERT_SERVER_URL=http://localhost:8443
ROBERT_API_TOKEN=dev-token-12345
CLAUDE_CLI_PATH=/usr/local/bin/claude
MOCK_MODE=false
LOG_LEVEL=debug
```

#### Docker Compose for Local Testing

```yaml
# docker-compose.dev.yml
version: '3.8'

services:
  robert-server:
    build:
      context: ./robert-server
      dockerfile: Dockerfile.dev
    ports:
      - "8443:8443"
    environment:
      - DEV_MODE=true
      - LOG_LEVEL=debug
    volumes:
      - ./robert-server:/app
      - ~/.config/claude:/root/.config/claude  # Mount claude-cli config
    command: cargo watch -x 'run -- --dev'

  # Optional: Mock claude-cli for isolated testing
  mock-claude:
    build: ./test/mock-claude
    volumes:
      - mock-responses:/data
```

Start with:
```bash
docker-compose -f docker-compose.dev.yml up
```

#### VS Code Launch Configuration

```json
// .vscode/launch.json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug robert-server",
      "cargo": {
        "args": ["run", "--bin", "robert-server", "--", "--dev"],
        "filter": {
          "name": "robert-server",
          "kind": "bin"
        }
      },
      "args": ["--dev"],
      "cwd": "${workspaceFolder}/robert-server"
    },
    {
      "type": "node",
      "request": "launch",
      "name": "Debug Tauri App (Local Server)",
      "runtimeExecutable": "npm",
      "runtimeArgs": ["run", "dev:local"],
      "cwd": "${workspaceFolder}/robert-app"
    }
  ],
  "compounds": [
    {
      "name": "Debug Full Stack",
      "configurations": ["Debug robert-server", "Debug Tauri App (Local Server)"],
      "stopAll": true
    }
  ]
}
```

### 13. Development Phases

#### Phase 1: MVP
- Basic HTTP/HTTPS endpoint
- Single request/response (no streaming)
- Simple token auth
- Synchronous claude-cli execution
- **Local development mode**

#### Phase 2: Streaming
- WebSocket/SSE streaming support
- Async claude-cli process handling
- Concurrent session management
- **Integration test suite**

#### Phase 3: Production Hardening
- Advanced rate limiting
- Metrics and monitoring
- Comprehensive error handling
- Performance optimization
- **E2E test automation**

#### Phase 4: Enterprise Features
- Multi-tenancy support
- Advanced authentication (OAuth2, mTLS)
- Audit logging
- HA deployment patterns

## Testing Strategy

### Unit Tests
- Request parsing and validation
- Authentication logic
- Error handling paths

### Integration Tests
- End-to-end request flow with mock claude-cli
- Streaming behavior
- Timeout handling
- Concurrent session management

### Load Tests
- Sustained concurrent connections
- Large payload handling
- Memory leak detection
- Resource cleanup verification

## References

- Warp documentation: https://docs.rs/warp
- Tokio async runtime: https://tokio.rs
- Server-Sent Events spec: https://html.spec.whatwg.org/multipage/server-sent-events.html
- WebSocket protocol: RFC 6455
