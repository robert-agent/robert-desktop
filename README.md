# Robert - Automation for Everyone

**Watch automation happen. Learn by seeing. Control everything. Own your data.**

Robert is an open-source, local-first browser automation tool that brings automation to everyone—not just programmers. While Zapier requires API knowledge, Claude agents are black boxes, and GPT provides no visibility, Robert lets you **watch automation work in real-time** with beautiful visual feedback.

## Why Robert?

### The Problem
- 🔌 **Zapier/IFTTT** require understanding APIs and webhooks
- 🤖 **Claude/GPT agents** are black boxes—you can't see what's happening
- 💻 **Selenium/Playwright** require programming and test-runner knowledge
- 🏢 **Herd/Monitoro** are centralized, proprietary, and require programming

### Our Solution
- 🎤 **Voice-driven** - Talk through your automation, AI writes it for you
- 👁️ **Visual feedback** - Watch browser automation happen in real-time
- 🎮 **Full control** - Pause, abort, inspect state at any moment
- 🏠 **Local-first** - 100% on-device AI, zero data sent to cloud by default
- 🔒 **Privacy protected** - Auto-obfuscates passwords/sensitive data if you opt for cloud AI
- 🆓 **Open source** - Free, auditable, community-owned
- 📝 **Markdown scripts** - Human-readable format, not scary YAML or code
- 🎨 **Eye-candy UI** - Beautiful interface that makes automation delightful

## Project Status

✅ **Phase 0 Complete** - CLI Prototype with Chrome Auto-download
🔄 **Phase 1 Starting** - Native Desktop App with Visual Feedback

## What Robert Provides

- **Talk, Don't Type** - Voice-driven agent creator writes scripts for you
- **Watch It Work** - Real-time visual feedback as automation runs
- **100% Local AI** - All inference runs on your device by default (zero cloud)
- **Privacy First** - If you opt for cloud AI, we auto-obfuscate passwords & sensitive data
- **Markdown Scripts** - Human-readable format anyone can understand
- **Full Control** - Pause, abort, and inspect automation at any moment
- **Content Capture** - Screenshots and text extraction
- **Beautiful UI** - Eye-candy interface that builds confidence

## Technology Stack

- **Desktop Framework**: Tauri 2.0 (planned)
- **Frontend**: Svelte + TypeScript + Tailwind CSS (planned)
- **Backend**: Rust 1.70+
- **Browser Automation**: spider_chrome (Chrome DevTools Protocol)
- **Chrome Management**: Auto-download via spider_chromiumoxide_fetcher
- **Target Platform**: macOS (v1.0), Linux (supported), Windows (planned)

## Project Structure

```
robert/
├── PRD.md                    # Product Requirements Document
├── IMPLEMENTATION_PLAN.md   # 7+ week development roadmap
├── PHASE_0_TASKS.md         # CLI prototype task breakdown
├── README.md                # This file
├── CHANGELOG.md             # Version history
├── Cargo.toml               # Workspace configuration
├── archive/                 # Deprecated documents
└── crates/                  # Rust workspace
    ├── robert-webdriver/    # Browser automation library
    │   ├── src/
    │   │   ├── browser/     # Chrome CDP implementation
    │   │   ├── error.rs     # Error types
    │   │   └── lib.rs
    │   └── tests/           # E2E tests (auto-download Chrome)
    ├── robert-cli/          # CLI tool (implemented)
    │   └── src/main.rs
    └── robert-app/          # Tauri desktop app (placeholder)
```

## Documentation

- **[PRD](PRD.md)** - Complete product requirements document
- **[Implementation Plan](IMPLEMENTATION_PLAN.md)** - 7+ week development roadmap with Phase 0 CLI prototype
- **[Phase 0 Tasks](PHASE_0_TASKS.md)** - Detailed task breakdown for CLI prototype (2-3 days)

## Roadmap

### Phase 0 (Days 1-3) - CLI Prototype ✅ COMPLETE
- ✅ Cargo workspace with 3 crates
- ✅ Browser automation via Chrome DevTools Protocol (CDP)
- ✅ Auto-download Chrome for Testing (~150MB)
- ✅ CLI tool: navigate and fetch content
- ✅ Headless mode and CI/CD support
- ✅ E2E tests with visible window (5 second delay)

### Version 1.0 (Weeks 1-7) - macOS Desktop App
- 🔄 Tauri desktop application
- 🔄 Chrome automation via CDP
- 🔄 Real-time execution UI
- 🔄 Screenshot and text capture
- 🔄 Voice-driven Markdown script creation
- 🔄 Output management

### Version 1.5 (Months 3-4) - Linux & Advanced Features
- ✅ Linux CLI support (already working)
- ✅ Headless Chrome support (already working)
- 🔄 Docker container
- 🔄 REST API

### Version 2.0+ - Multi-Browser & Beyond
- 🔄 Firefox, Edge, Safari support
- 🔄 Windows desktop app
- 🔄 Visual script builder
- 🔄 Cloud execution

## Example: Talk, Don't Type

Instead of writing code or YAML, just **talk to Robert**:

**You:** "I want to check product prices on competitor.com"

**Robert:** "Great! I'll help you create that. What do you want to do on the page?"

**You:** "Take a screenshot and extract all the prices"

**Robert creates this for you:**

```markdown
---
name: competitor-price-check
description: Monitor competitor product prices
---

# Competitor Price Monitor

Automatically check competitor.com for pricing changes.

## Steps

### 1. Navigate to products page
Go to https://competitor.com/products
- Wait for: page load complete

### 2. Take screenshot
Capture full page screenshot
- Filename: products_page.png
- Type: full-page

### 3. Extract all prices
Get text from price elements
- Selector: .price
- Save to: prices.txt
- Get all matches: yes
```

**No programming. No YAML. No syntax.** Just talk through what you want, Robert writes it in readable Markdown, and you watch it work.

## Quick Start

### Desktop App (Tauri)

**See [BUILD.md](BUILD.md) for complete build instructions.**

Quick start:
```bash
cd crates/robert-app
bun install
bun run dev
```

**Prerequisites:** Rust 1.70+, Bun, system GTK libraries (Linux). See [BUILD.md](BUILD.md) for details.

### CLI Prototype

**Prerequisites:** Just Rust 1.70+. Chrome downloads automatically!

### Try It Now

```bash
# Clone the repository
git clone https://github.com/yourusername/robert.git
cd robert

# Run Robert - Chrome auto-downloads on first run (~150MB, one-time)
cargo run --bin robert -- example.com
```

**That's it!** Chrome downloads automatically, opens visibly, navigates to the site, and prints the page content.

### More Examples

```bash
# Extract visible text only
cargo run --bin robert -- example.com --format text

# Extract specific element (like page title)
cargo run --bin robert -- example.com --selector "h1"

# Run headless (no visible window)
cargo run --bin robert -- example.com --headless

# Advanced: Connect to your existing Chrome session
google-chrome --remote-debugging-port=9222 &
cargo run --bin robert -- example.com --debug-port 9222
```

### What You'll See

1. **Chrome downloads automatically** (first run only, cached at `~/.cache/robert/chrome`)
2. **Browser opens visibly** showing the automation in action
3. **Content is extracted** and printed to console
4. **Browser closes** when done

This is just the CLI prototype. The full desktop app will have:
- 🎤 Voice-driven agent creator - Just talk, don't type
- 🎨 Beautiful visual dashboard
- ▶️ Play/pause/stop controls
- 📊 Real-time step-by-step progress
- 🖼️ Screenshot previews
- 📝 Markdown script viewer (editable if you want)

**Running Tests:**
```bash
# Run all tests (auto-downloads Chrome, visible window, 5 second delay)
cargo test --package robert-webdriver

# Run in CI mode (headless, no delay)
CI=true cargo test --package robert-webdriver

# Run with output visible
cargo test --package robert-webdriver -- --nocapture

# Run all workspace checks
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

### Phase 1+: Desktop App (Future)

**Additional Prerequisites:**
- Node.js 18+
- macOS 11+ (for v1.0)

Implementation begins after Phase 0 validation.

## Development Status

**Current Phase**: Phase 1 Complete ✅ → Tauri Desktop App Ready for Testing

### Completed:
1. ✅ Planning documents finalized
2. ✅ **Phase 0 implementation complete** (see [PHASE_0_TASKS.md](PHASE_0_TASKS.md))
3. ✅ Browser automation via Chrome DevTools Protocol (CDP)
4. ✅ Workspace with 3 crates created
5. ✅ robert-webdriver library with auto-download Chrome
6. ✅ robert-cli with headless mode and CI support
7. ✅ E2E tests with 5-second visible window
8. ✅ **Tauri desktop app implemented** (see [crates/robert-app/README.md](crates/robert-app/README.md))
9. ✅ Real-time debug view with color-coded logs
10. ✅ URL input with auto-browser launch
11. ✅ Event system with 11 event types
12. ✅ Modern UI with TypeScript/Svelte

### Next Steps:
1. 🔄 Test on desktop environment with GUI
2. 🔄 Implement voice-driven agent creator
3. 🔄 Implement Markdown script parser
4. 🔄 Add screenshot capture functionality
5. 🔄 Add navigation history

### Phase 0 Summary:
- **Workspace**: Cargo multi-crate workspace with shared dependencies
- **robert-webdriver**: Core browser automation library using spider_chrome (CDP)
- **Auto-download**: Chrome for Testing downloads automatically (~150MB, cached)
- **robert-cli**: CLI tool with headless mode, CI support, and URL auto-fixing
- **robert-app**: Placeholder for future Tauri application
- **Tests**: E2E tests with visible window (5 second delay) and CI mode

## Philosophy

**Automation should be for everyone, not just developers.**

We believe:
- **Voice beats typing** - Talk through your automation naturally
- **Visual feedback builds confidence** - See it work, trust it works
- **Local-first protects privacy** - 100% on-device AI, your data never leaves
- **Privacy by design** - Auto-obfuscation if you choose cloud AI
- **Open source ensures freedom** - No vendor lock-in, auditable code
- **Beautiful UI matters** - Eye-candy makes automation delightful
- **Readable > code** - Markdown over YAML over programming
- **Control > convenience** - Pause/abort beats fire-and-forget

## Inspiration

Tools like **Herd** and **Monitoro** are pioneering visual browser automation with gorgeous UIs. We're inspired by their approach but believe automation should be:
- **Free and open source** (not proprietary)
- **Local-first** (not centralized)
- **Accessible to non-programmers** (not requiring test-runner knowledge)
- **Owned by users** (not locked into platforms)

## Contributing

**Phase 0 complete!** Phase 1 (desktop app) starting soon.

Interested in contributing? Watch this space for:
- Desktop app architecture discussions
- UI/UX feedback sessions
- Community script library
- Documentation improvements

## License

MIT OR Apache-2.0 (dual-licensed for maximum freedom)

## Links

- **PRD**: [Full Product Requirements](PRD.md)
- **Implementation Plan**: [7-week roadmap](IMPLEMENTATION_PLAN.md)
- **Phase 0 Tasks**: [CLI prototype details](PHASE_0_TASKS.md)

---

**"Watch automation happen. Learn by seeing. Control everything."**
