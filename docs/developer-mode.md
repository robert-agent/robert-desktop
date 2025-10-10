# Developer Mode

Developer Mode is a feature in the Robert Tauri app that provides tools for manual end-to-end testing and debugging of browser automation with CDP (Chrome DevTools Protocol) commands.

## Features

### 1. System Paths Information

Displays important system paths for debugging and configuration:
- **Installation Directory**: Where the app is installed
- **Config Directory**: Application configuration files location
- **Data Directory**: Application data storage location
- **Cache Directory**: Application cache location
- **Temp Directory**: System temporary files location
- **Current Directory**: Working directory of the application
- **Chrome Path**: Location of Chrome executable (if detected)

All paths can be copied to clipboard with a single click.

### 2. Mock Test Server

A local HTTP server for manual e2e testing of browser automation:

#### Features:
- Starts on a random available port for isolation
- Serves an interactive test page with:
  - Input fields for testing form automation
  - Buttons for testing click events
  - Text areas for testing multi-line input
  - Output display for testing result extraction
  - API endpoint (`/api/test`) for testing JSON responses

#### Usage:
1. Click "Start Server" to launch the test server
2. Click "Open in Browser" to navigate to the test page in the automated browser
3. Use the chat interface to test CDP commands against the page
4. Watch the DebugView to see commands being executed in real-time
5. Click "Stop Server" when finished

## How to Use

### Accessing Developer Mode

1. Launch the Robert app
2. Click the **🛠️ Developer Mode** button in the header
3. The developer mode screen will replace the main UI
4. Click the button again (now showing **✖ Developer Mode**) to exit

### Manual Testing Workflow

1. **Start the Test Server**
   - Click "Start Server" in the Mock Test Server section
   - Wait for the server to start and display its URL

2. **Open the Test Page**
   - Click "Open in Browser" to navigate to the test page
   - The browser will automatically launch if not already running

3. **Test CDP Commands**
   - Use the chat interface to interact with the test page
   - Example commands:
     - "Click the test button"
     - "Fill in the input field with 'Hello World'"
     - "Get the text from the output div"
     - "Take a screenshot"

4. **Monitor Execution**
   - Switch back to the main view (exit developer mode)
   - Check the DebugView to see CDP commands being executed
   - Verify the commands produce the expected browser actions

5. **Stop the Server**
   - When finished, return to developer mode
   - Click "Stop Server" to shut down the test server

## Test Page Elements

The mock test page includes the following elements for testing:

| Element ID | Type | Purpose |
|------------|------|---------|
| `test-input` | Text Input | Test text input automation |
| `test-textarea` | Textarea | Test multi-line text input |
| `test-button` | Button | Test click events |
| `alert-button` | Button | Test alert dialogs |
| `output` | Div | Test content extraction |

## API Endpoint

The test server provides a JSON API endpoint:

```
GET /api/test
```

**Response:**
```json
{
  "status": "ok",
  "message": "Test API endpoint",
  "timestamp": "2025-10-10T12:34:56Z"
}
```

## Architecture

### Backend (Rust)

#### Modules:
- `developer_mode/mod.rs`: Main module with system paths functionality
- `developer_mode/test_server.rs`: HTTP test server implementation
- `commands/developer_mode.rs`: Tauri commands for frontend communication

#### Tauri Commands:
- `get_system_paths()`: Retrieve system path information
- `start_dev_test_server()`: Start the mock test server
- `stop_dev_test_server()`: Stop the mock test server
- `get_dev_test_server_status()`: Check server running status

#### State Management:
The `AppState` struct holds:
- `driver`: Chrome driver instance
- `dev_server`: Developer mode test server instance

### Frontend (Svelte)

#### Components:
- `DeveloperMode.svelte`: Main developer mode UI component
  - System paths display section
  - Test server controls section
  - Server status indicators

#### Features:
- Auto-refresh server status every 2 seconds
- One-click clipboard copy for paths and URLs
- Responsive design with clear visual feedback

## Testing

The developer mode includes comprehensive test coverage:

### Unit Tests (8 tests)
Located in `src-tauri/src/developer_mode/tests.rs`:
- System paths functionality
- Test server startup and configuration
- HTTP request handling
- API endpoint responses
- Multiple server instances
- Graceful shutdown

### Integration Tests (3 tests)
Located in `src-tauri/tests/developer_mode_integration.rs`:
- Server lifecycle in app state
- Concurrent access to server state
- Restart scenarios

### E2E Tests (5 tests)
Located in `src-tauri/tests/e2e_developer_mode.rs`:
- Complete developer workflow
- Manual testing workflow simulation
- Chrome automation integration
- System paths structure validation
- Multiple developer sessions

### Running Tests

```bash
# Run all developer mode tests
cargo test developer_mode

# Run specific test suites
cargo test --lib developer_mode              # Unit tests
cargo test --test developer_mode_integration # Integration tests
cargo test --test e2e_developer_mode         # E2E tests
```

## Dependencies

### Backend:
- `warp`: HTTP server framework
- `reqwest`: HTTP client for server health checks
- `tokio`: Async runtime
- `serde`: Serialization/deserialization

### Frontend:
- `@tauri-apps/api`: Tauri API for frontend-backend communication

## Future Enhancements

Potential improvements for developer mode:

1. **CDP Command Builder**
   - Visual builder for CDP commands
   - Command history and favorites
   - Template library

2. **Network Inspector**
   - Monitor network requests
   - Request/response viewer
   - Network throttling controls

3. **Console Logger**
   - Browser console output
   - JavaScript error tracking
   - Custom log filtering

4. **Screenshot Gallery**
   - Automatic screenshot capture
   - Before/after comparisons
   - Annotation tools

5. **Test Recording**
   - Record manual test sessions
   - Export as CDP scripts
   - Replay recorded sessions

6. **Multiple Test Pages**
   - Forms test page
   - JavaScript interaction page
   - File upload test page
   - WebSocket test page

## Troubleshooting

### Server Won't Start
- Check if the port is already in use
- Verify firewall settings allow local connections
- Check application logs for errors

### Browser Won't Navigate
- Ensure Chrome driver is properly installed
- Verify the browser is launched before navigation
- Check that the server URL is accessible

### System Paths Not Displaying
- Verify the app has proper file system permissions
- Check that the Tauri app is properly initialized
- Review console for JavaScript errors

## Security Considerations

- The test server only binds to `127.0.0.1` (localhost)
- Random port assignment prevents conflicts
- Server automatically shuts down when the app closes
- No external network access required
