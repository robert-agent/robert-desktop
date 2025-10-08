# Robert Tauri App

A simple Tauri desktop application for browser automation with real-time debug visibility.

## Features

- **Simple URL Navigation**: Enter any URL and click "Go" to navigate
- **Auto-Browser Launch**: Automatically launches Chrome in headless mode when needed
- **Real-Time Debug View**: See everything happening under the hood:
  - Chrome downloading/launching
  - Page navigation
  - Content loading and parsing
  - Error messages with details
- **Color-Coded Logs**: Info (blue), Success (green), Error (red)
- **Modern UI**: Clean interface with gradient header and terminal-style debug view

## Architecture

### Backend (Rust)
- **Tauri 2.0** - Desktop app framework
- **robert-webdriver** - Browser automation library using Chrome DevTools Protocol
- **Async/Await** - Tokio runtime for non-blocking operations
- **Event System** - Real-time communication with frontend

### Frontend (TypeScript/Svelte)
- **SvelteKit** - Reactive UI framework
- **TypeScript** - Type-safe development
- **Bun** - Fast package manager and runtime

## Project Structure

```
robert-app/
├── src-tauri/                  # Rust backend
│   ├── src/
│   │   ├── commands/          # Tauri command handlers
│   │   │   └── mod.rs         # Browser commands (launch, navigate, etc.)
│   │   ├── events/            # Event system
│   │   │   └── mod.rs         # Debug event types and emitters
│   │   ├── state/             # Application state
│   │   │   └── mod.rs         # Browser driver state management
│   │   ├── lib.rs             # Module orchestration
│   │   └── main.rs            # Entry point
│   ├── Cargo.toml             # Rust dependencies
│   └── tauri.conf.json        # Tauri configuration
│
├── src/                        # Frontend
│   ├── components/
│   │   ├── UrlInput.svelte    # URL input + Go button
│   │   └── DebugView.svelte   # Debug log viewer
│   ├── lib/
│   │   ├── types.ts           # TypeScript types
│   │   ├── stores.ts          # Svelte stores
│   │   ├── tauri.ts           # Tauri command wrappers
│   │   └── events.ts          # Event listeners
│   └── routes/
│       └── +page.svelte       # Main app layout
│
├── package.json               # Node dependencies
└── README.md                  # This file
```

## Commands (Backend → Frontend)

### `launch_browser()`
Launches Chrome browser in headless mode. Auto-downloads if needed.

**Emits:**
- `ChromeDownloading` - Download started
- `ChromeLaunching` - Browser launching
- `ChromeLaunched` - Browser ready

### `navigate_to_url(url: string)`
Navigates to specified URL.

**Returns:** `NavigationResult` with success, url, title, message

**Emits:**
- `PageNavigating` - Navigation started
- `PageLoaded` - Page loaded with title

### `get_page_content()`
Extracts visible text from current page.

**Returns:** `string` - Page text content

### `close_browser()`
Closes the browser and cleans up resources.

## Events (Backend → Frontend)

All events are sent through the `debug-event` channel:

```typescript
type DebugEventType =
  | { type: "ChromeDownloading"; data: { message: string } }
  | { type: "ChromeLaunching"; data: { message: string } }
  | { type: "ChromeLaunched"; data: { message: string } }
  | { type: "PageNavigating"; data: { url: string } }
  | { type: "PageLoaded"; data: { url: string; title: string } }
  | { type: "Info"; data: { message: string } }
  | { type: "Success"; data: { message: string } }
  | { type: "Error"; data: { message: string; details?: string } }
```

## Development

### Prerequisites

- **Rust 1.70+**: `rustc --version`
- **Bun**: `curl -fsSL https://bun.sh/install | bash`
- **System Dependencies (Linux)**:
  ```bash
  # Ubuntu/Debian
  sudo apt-get update
  sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    libxdo-dev \
    libappindicator3-dev \
    librsvg2-dev \
    patchelf
  ```

### Install Dependencies

```bash
cd crates/robert-app
bun install
```

### Run in Development Mode

```bash
bun run dev
```

This runs `bunx tauri dev` which:
1. Starts the Vite dev server (http://localhost:1420)
2. Compiles the Rust backend
3. Launches the Tauri app window
4. Enables hot-reload for frontend changes
5. Watches for backend changes

### Build for Production

```bash
bun run build
```

Outputs:
- **Linux**: `.deb`, `.AppImage` in `src-tauri/target/release/bundle/`
- **macOS**: `.app`, `.dmg` in `src-tauri/target/release/bundle/`
- **Windows**: `.msi`, `.exe` in `src-tauri/target/release/bundle/`

## Usage

1. **Launch the app**
2. **Enter a URL** in the input field (e.g., `https://example.com`)
3. **Click "Go"** or press Enter
4. **Watch the debug log** to see:
   - Browser initialization
   - Chrome download (first time only)
   - Navigation progress
   - Page loading
   - Success or error messages

## Configuration

### Window Size
Edit `src-tauri/tauri.conf.json`:

```json
"windows": [{
  "width": 1280,
  "height": 800,
  "resizable": true
}]
```

### Browser Mode
The app uses **auto-detection** mode:
- **CI environments**: Launches headless automatically
- **Desktop**: Launches with visible window

To change behavior, edit `src-tauri/src/commands/mod.rs`:

```rust
// Auto-detection (current - detects CI environment)
ChromeDriver::launch_auto().await

// Always visible window
ChromeDriver::launch_sandboxed().await

// Always headless
ChromeDriver::launch_with_path(path, false, true).await

// No sandbox (Linux AppArmor workaround)
ChromeDriver::launch_no_sandbox().await
```

## Troubleshooting

### "webkit2gtk not found" (Linux)
Install system dependencies:
```bash
sudo apt-get install libwebkit2gtk-4.1-dev librsvg2-dev
```

### Browser fails to launch
Check debug log for details. Common issues:
- Chrome download failed (check internet connection)
- Insufficient permissions
- Missing system libraries

### Debug log not updating
Check browser console (F12) for JavaScript errors. Event listeners should be set up in `onMount()`.

## Tech Stack Summary

| Component | Technology |
|-----------|-----------|
| Desktop Framework | Tauri 2.0 |
| Backend Language | Rust 1.70+ |
| Frontend Framework | SvelteKit |
| Language | TypeScript |
| Package Manager | Bun |
| Browser Automation | Chrome DevTools Protocol |
| Chrome Driver | spider_chrome |
| Async Runtime | tokio |
| State Management | Svelte stores |

## Next Steps

- [ ] Add screenshot capture
- [ ] Support multiple tabs
- [ ] Add script recorder
- [ ] Export debug logs
- [ ] Add filters to debug view
- [ ] Save navigation history

## License

MIT OR Apache-2.0
