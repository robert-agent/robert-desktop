# Robert - Local AI with Web Browser

**Privacy-first AI automation that keeps your data local and your options open.**

Robert is a web browser and agent platform that keeps your personal knowledge base local. Build your library of workflows, configure your agents, and store your browsing history—all on your machine. Connect to any AI provider for inference without giving them access to your accumulated knowledge and context.

### The Problem with Traditional AI Tools

| Issue | Traditional AI Assistants | Robert |
|-------|--------------------------|--------|
| **Your Data** | Stored on vendor servers | Always local, encrypted |
| **AI Provider** | Locked to one vendor | Switch freely anytime |
| **Privacy** | Builds profile on you | Stateless, no profiling |
| **Visibility** | Black box operations | Watch everything happen |
| **Control** | Can't stop mid-task | Abort anytime |
| **Vendor Lock-in** | Migrate = lose history | Portable, vendor-neutral |
| **Background Tasks** | Cloud daemons 24/7 | Off means OFF |

### Your Personal Knowledge Base Stays Local

**Build context and configure agents without sharing it with AI companies.**

Robert keeps your accumulated knowledge, agent configurations, and browsing data local:

- **Knowledge Base** - Your command library, agent configurations, and workflow templates never leave your machine
- **Agent Context** - Each agent's learned preferences and refined instructions stay with you
- **Browsing History** - Your cookies, sessions, and history stored locally, encrypted
- **Switch AI Vendors Freely** - Use Claude today, GPT tomorrow, local models next week—your knowledge base stays intact
- **No Vendor Profiling** - AI providers only see individual task descriptions, not your accumulated context
- **Multi-User Encrypted** - Each user gets their own isolated, encrypted knowledge base

### Project-Driven Workflows

**Build repeatable automation that runs in the background.**

- **Command Scripts** - Create reusable tasks and agent teams through natural conversation
- **Visual Monitoring** - Watch your projects and agents working in real-time
- **Background Execution** - Set tasks running and come back to results
- **Self-Improving** - Commands learn from feedback and get better over time

### Off Means Off

**When you close Robert, everything stops. No cloud daemons, no persistent agents.**

Unlike cloud automation platforms that run 24/7 with your credentials, Robert respects your boundaries. Close the app, and all agents, profiles, and sessions immediately stop. Your data stays on your machine, under your control.

## Key Features

### Full-Featured Web Browser

**A real browser with AI agent capabilities.**

- **Browse the Web** - Use it like any browser: navigate, search, shop, research
- **Direct Browser Control** - Agents can interact with any website
- **Native Interactions** - Click, type, scroll, navigate, extract—everything a browser can do
- **Manual + Automated** - Browse manually, automate tasks, or blend both
- **Visual Debugging** - See exactly what agents are doing in real-time

### Visual-First Design

**See your automation work, don't just hope it works.**

- **Real-time Browser View** - Watch automation happen in a live browser window
- **Color-coded Debug Logs** - Understand what's happening at every step
- **Screenshot Capture** - Document states and outcomes automatically
- **Event Tracking** - Complete audit trail of every action taken

### Local Data Storage

**Your browsing data belongs to you, always.**

- **Local Browser Profiles** - All cookies, history, and sessions stay on your machine
- **Encrypted Storage** - User profiles protected with strong encryption
- **Multi-User Support** - Family-friendly with isolated workspaces per person
- **Ephemeral Sessions** - Privacy mode that leaves no trace when closed
- **Import/Export** - Move your commands and workflows between machines

### Personal Knowledge Base & Agent Configuration

**Build your own library of workflows and agent configurations.**

- **Your Agent Library** - Configure specialized agents for different tasks (research, shopping, monitoring)
- **Command Repository** - Build a personal library of reusable workflows
- **Agent Context** - Each agent remembers your preferences, style, and requirements
- **Parameterized Workflows** - Save templates with variables you fill in each time
- **Self-Improving** - Agents learn from your feedback (👍 or 👎) and refine their approach
- **Local Storage** - All knowledge, configurations, and history stay on your machine
- **Watch & Learn** - See agents browse, click, and complete tasks in real-time

### Vendor Independence

**Never locked in, always in control.**

- **Pure Inference Providers** - AI vendors only see task descriptions, not your full context
- **Swap Providers Anytime** - Change from Claude to GPT to local models without losing data
- **No Training on Your Data** - Stateless interactions mean no persistent profiling
- **Open Architecture** - Community can add new AI provider integrations
- **Local-First Option** - Run completely offline with local inference models

## Current Status

**Active Development** - Core platform complete, advanced features in progress

### Implemented
- ✅ Desktop browser application with native performance
- ✅ Full browser automation capabilities
- ✅ User profile management with strong encryption
- ✅ Ephemeral and named browser profile support
- ✅ AI agent workflow system with multiple agent types
- ✅ Real-time visual feedback and event tracking
- ✅ Screenshot and content extraction
- ✅ Command creation and execution framework
- ✅ Multi-user isolation with encrypted workspaces
- ✅ Automated releases and updates

### In Progress
- 🔄 Generative UI for dynamic command forms
- 🔄 Command refinement and versioning system
- 🔄 Advanced browser interactions (click, type, scroll, forms)
- 🔄 Project management and background task execution
- 🔄 Local inference model integration

## Supported Platforms

- **macOS** - Primary development platform, fully supported
- **Linux** - Supported with native performance
- **Windows** - Planned support

## Project Structure

```
robert/
├── README.md                # This file
├── BUILD.md                 # Build instructions
├── Cargo.toml               # Workspace configuration
└── crates/                  # Rust workspace
    ├── robert-webdriver/    # Browser automation library
    │   ├── src/
    │   │   ├── browser/     # Chrome CDP implementation
    │   │   ├── error.rs     # Error types
    │   │   └── lib.rs
    │   └── tests/           # E2E tests
    ├── robert-cli/          # CLI tool
    │   └── src/main.rs
    └── robert-app/          # Tauri desktop application
        ├── src/             # Svelte frontend
        ├── src-tauri/       # Rust backend
        └── package.json     # Frontend dependencies
```

## Quick Start

### Prerequisites
- **Rust** 1.70 or later
- **Bun** (for frontend development)
- **System libraries** (macOS: Xcode, Linux: GTK/WebKit)

### Running the Desktop App

```bash
# Clone the repository
git clone https://github.com/yourusername/robert.git
cd robert

# Install frontend dependencies
cd crates/robert-app
bun install

# Run in development mode
bun run dev
```

### Running the CLI

```bash
# Navigate to robert directory
cd robert

# Run the CLI (Chrome auto-downloads on first run)
cargo run --bin robert -- example.com

# Extract text only
cargo run --bin robert -- example.com --format text

# Run headless
cargo run --bin robert -- example.com --headless
```

See [BUILD.md](BUILD.md) for detailed build instructions.

## How It Works

### 1. Start the Browser

Launch Robert as your web browser. Everything starts local and stays local—your browsing history, cookies, and sessions are protected by industry-standard encryption.

```
Your Password → Encrypted User Profile
                ↓
                Browser data (cookies, history, bookmarks)
                Commands & workflows
                Agent configurations
```

### 2. Browse Manually or Automate with AI

Use Robert like any browser, or ask AI agents to help:

> "Search for winter jackets under $200 on REI and Patagonia, but skip Amazon. Show me the top 5 with free returns."

AI generates browser automation that you can reuse and refine.

### 3. Watch Agents Work

When agents run tasks, you see them browse in real-time:
- Pages load in the actual browser window
- Color-coded logs explain each step
- Screenshots document the journey
- Abort anytime if something looks wrong

### 4. Refine & Reuse

Give feedback (👍 or 👎) and commands improve. Build a library of automations that get smarter over time.

### 5. Your Browser, Your Choice of AI

Your browser data and commands stay local. Connect to any AI provider for inference:
- Anthropic Claude
- OpenAI GPT
- Local models (Llama, Mistral)
- Custom inference endpoints

No vendor lock-in. Your browsing data stays yours.


### Browser + Agent Platform Architecture

Robert separates your local browser data from remote AI inference:

```
┌─────────────────────────────────────────────────┐
│  Your Machine (Robert Browser)                  │
│                                                 │
│  ┌──────────────────────────────────────┐      │
│  │  Local Browser Data (Never Leaves)   │      │
│  │  • User profiles (encrypted)         │      │
│  │  • Browsing history & cookies        │      │
│  │  • Bookmarks & sessions              │      │
│  │  • Command definitions               │      │
│  │  • Screenshots & captures            │      │
│  └──────────────────────────────────────┘      │
│                    ↕                            │
│  ┌──────────────────────────────────────┐      │
│  │  Browser Engine                      │      │
│  │  • Manual browsing                   │      │
│  │  • Agent automation                  │      │
│  │  • Workflow execution                │      │
│  │  • Event tracking                    │      │
│  └──────────────────────────────────────┘      │
│                    ↕                            │
│           Only task descriptions                │
│           sent to inference ↓                   │
└─────────────────────────────────────────────────┘
                      ↓
         ┌────────────────────────┐
         │  AI Inference Provider │
         │  (Your Choice)         │
         │  • Claude API          │
         │  • OpenAI API          │
         │  • Local Models        │
         │  • Custom Endpoints    │
         └────────────────────────┘
```
