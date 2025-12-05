# Chat UI - Real-time User Feedback for Web Automation

The Chat UI feature provides a sidebar chat interface that is automatically injected into every webpage navigated to by the webdriver. This enables real-time communication between users and automated agents during web automation tasks.

## Features

- 🚀 **Automatic Injection** - Chat UI is automatically injected on every page navigation
- 💬 **Two-way Communication** - Both user and agent can send messages
- 🎨 **Clean Design** - Non-intrusive sidebar interface with modern styling
- 🔄 **Persistent Across Pages** - Chat UI is re-injected when navigating to new pages
- 🎛️ **Collapsible** - Users can collapse/expand the sidebar as needed
- 🔌 **Optional** - Can be disabled if not needed

## Quick Start

```rust
use robert_webdriver::{ChromeDriver, ConnectionMode};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Launch Chrome - chat UI is enabled by default
    let driver = ChromeDriver::new(ConnectionMode::Sandboxed {
        chrome_path: None,
        no_sandbox: true,
        headless: false,
    }).await?;

    // Navigate to a webpage - chat UI is automatically injected
    driver.navigate("https://example.com").await?;

    // Send a message from the agent to the user
    driver.send_chat_message("Hello! I'm analyzing this page.").await?;

    // Check if the user sent any feedback
    let messages = driver.get_chat_messages().await?;
    for msg in messages {
        if msg.sender == "user" {
            println!("User says: {}", msg.text);
        }
    }

    Ok(())
}
```

## API Reference

### Basic Operations

#### Send Agent Message
Send a message from the agent to the chat UI:

```rust
driver.send_chat_message("Processing your request...").await?;
```

#### Get All Messages
Retrieve all messages from the chat:

```rust
let messages = driver.get_chat_messages().await?;
for msg in messages {
    println!("[{}] {}: {}", msg.timestamp, msg.sender, msg.text);
}
```

#### Clear Messages
Clear all messages from the chat:

```rust
driver.clear_chat_messages().await?;
```

### UI Control

#### Collapse Chat Sidebar
```rust
driver.collapse_chat().await?;
```

#### Expand Chat Sidebar
```rust
driver.expand_chat().await?;
```

### Advanced Control

#### Disable Auto-Injection
```rust
let mut driver = ChromeDriver::new(ConnectionMode::Sandboxed {
    chrome_path: None,
    no_sandbox: true,
    headless: false,
}).await?;

// Disable automatic injection
driver.chat_ui_mut().disable();

// Now navigation won't inject the chat UI
driver.navigate("https://example.com").await?;
```

#### Manual Injection
```rust
// Enable and manually inject
driver.chat_ui_mut().enable();
driver.inject_chat_ui().await?;
```

#### Check if Enabled
```rust
if driver.chat_ui().is_enabled() {
    println!("Chat UI is enabled");
}
```

## ChatMessage Structure

```rust
pub struct ChatMessage {
    pub text: String,      // Message content
    pub sender: String,    // "user" or "agent"
    pub timestamp: u64,    // Unix timestamp in milliseconds
}
```

## UI Appearance

The chat UI appears as a sidebar on the right side of the webpage with:

- **Header** - Blue header with "Agent Chat" title and collapse/expand button
- **Messages Area** - Scrollable message history with user messages (blue) and agent messages (gray)
- **Input Area** - Text area and send button for user feedback
- **Responsive** - Works on pages of any size

### Keyboard Shortcuts

- **Enter** - Send message
- **Shift+Enter** - New line in message

## Examples

### Example 1: Progress Updates

```rust
let driver = ChromeDriver::launch_no_sandbox().await?;

driver.navigate("https://example.com").await?;
driver.send_chat_message("Starting analysis...").await?;

// Do some work
tokio::time::sleep(Duration::from_secs(2)).await;

driver.send_chat_message("Analysis complete! Found 10 items.").await?;
```

### Example 2: User Feedback Loop

```rust
let driver = ChromeDriver::launch_no_sandbox().await?;
driver.navigate("https://example.com").await?;

driver.send_chat_message("Please review this page. Type 'ok' to continue.").await?;

// Poll for user response
loop {
    tokio::time::sleep(Duration::from_secs(1)).await;

    let messages = driver.get_chat_messages().await?;
    let user_messages: Vec<_> = messages.iter()
        .filter(|m| m.sender == "user")
        .collect();

    if let Some(last_msg) = user_messages.last() {
        if last_msg.text.to_lowercase().contains("ok") {
            driver.send_chat_message("Thanks! Continuing...").await?;
            break;
        }
    }
}
```

### Example 3: Multi-Page Workflow

```rust
let driver = ChromeDriver::launch_no_sandbox().await?;

// Page 1
driver.navigate("https://page1.com").await?;
driver.send_chat_message("Checking page 1...").await?;
// Chat UI is automatically injected

// Page 2
driver.navigate("https://page2.com").await?;
// Chat UI is re-injected automatically with message history reset
driver.send_chat_message("Now checking page 2...").await?;
```

## Running the Demo

A complete demo is available:

```bash
cargo run --example chat_ui_demo
```

This will:
1. Launch Chrome with visible UI (non-headless)
2. Navigate to example.com with chat UI injected
3. Send agent messages
4. Wait for user interaction
5. Demonstrate navigation to another page
6. Show collapse/expand functionality

## Testing

Run the chat UI tests:

```bash
cargo test --test chat_ui_test
```

**Note**: All tests use a local test server for complete isolation from external dependencies. No internet connection required and tests are fully offline-capable.

Tests cover:
- Automatic injection on navigation
- Sending agent messages
- Retrieving messages
- Clearing messages
- Persistence across navigation (using local /page2 route)
- Collapse/expand functionality
- Disable/enable functionality
- Manual injection

## Technical Details

### JavaScript Injection

The chat UI is implemented as a self-contained JavaScript module that:
- Prevents duplicate injections
- Uses shadow DOM isolation (optional future enhancement)
- Exposes a global API (`window.__ROBERT_CHAT_API__`)
- Stores messages in `window.__ROBERT_CHAT_MESSAGES__`

### Styling

- Fixed positioning on the right side
- High z-index (2147483647) to stay on top
- Modern, clean design with CSS variables
- Smooth animations for messages and collapse/expand

### Communication

- Agent to UI: JavaScript execution via CDP Runtime.evaluate
- UI to Rust: Messages stored in window object, retrieved via JavaScript evaluation
- Event-based: Custom events fired on user messages (can be extended for real-time polling)

## Future Enhancements

Potential improvements:
- [ ] WebSocket connection for real-time bidirectional communication
- [ ] Voice input/output
- [ ] File uploads from user
- [ ] Screenshot capture from UI
- [ ] Chat history persistence across sessions
- [ ] Theming support (dark mode, custom colors)
- [ ] Markdown rendering in messages
- [ ] Typing indicators

## Troubleshooting

### Chat UI Not Appearing

**Problem**: Navigate to a page but don't see the chat UI

**Solutions**:
1. Check if chat UI is enabled: `driver.chat_ui().is_enabled()`
2. Manually inject: `driver.inject_chat_ui().await?`
3. Check browser console for JavaScript errors

### Messages Not Sending

**Problem**: `send_chat_message()` doesn't show messages in UI

**Solutions**:
1. Verify chat UI is injected: Check for `#robert-chat-container` in DOM
2. Check for JavaScript errors in browser console
3. Try re-injecting: `driver.inject_chat_ui().await?`

### Headless Mode

The chat UI works in headless mode (for testing) but obviously won't be visible. Messages can still be sent and retrieved programmatically.

## License

Part of robert-webdriver project.
