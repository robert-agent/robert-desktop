# Robert Automation Script Format

## Overview

Robert uses **Claude-generated Chrome DevTools Protocol (CDP) commands** for browser automation. Instead of writing code, you describe what you want in natural language, and Claude generates the CDP command sequence as a JSON file. Scripts are **never compiled** into the application - they're loaded and interpreted at runtime.

**Key Concepts:**
- **Natural language input** - Describe your task to Claude
- **AI generates CDP commands** - Claude outputs structured JSON with CDP protocol commands
- **Runtime interpretation** - Scripts loaded from files and executed dynamically
- **No compilation required** - Change scripts without rebuilding the app

## Script Format

### JSON Structure

Robert automation scripts are JSON files containing Chrome DevTools Protocol commands:

```json
{
  "name": "script-name",
  "description": "What this automation does",
  "created": "2026-01-15",
  "author": "Claude",
  "cdp_commands": [
    {
      "method": "Page.navigate",
      "params": {
        "url": "https://example.com"
      }
    },
    {
      "method": "Page.captureScreenshot",
      "params": {
        "format": "png",
        "captureBeyondViewport": true
      },
      "save_as": "homepage.png"
    },
    {
      "method": "Runtime.evaluate",
      "params": {
        "expression": "document.querySelector('button#cta').click()"
      }
    }
  ]
}
```

### How It Works

1. **User describes task**: "Navigate to example.com, take a screenshot, click the login button"
2. **Claude generates CDP**: AI produces JSON with appropriate CDP commands
3. **Save to file**: Script saved as `my-automation.json`
4. **Runtime execution**: App loads JSON, sends CDP commands to Chrome
5. **No recompilation**: Edit script file anytime, run immediately

### Script Metadata

Top-level JSON fields:

| Field | Required | Description | Example |
|-------|----------|-------------|---------|
| `name` | Yes | Unique identifier (lowercase, hyphenated) | `"monitor-competitor-prices"` |
| `description` | Yes | Brief description of what the script does | `"Check competitor pricing daily"` |
| `created` | Auto | ISO date script was created | `"2026-01-15T10:30:00Z"` |
| `author` | Auto | Always "Claude" for AI-generated | `"Claude"` |
| `tags` | No | Array of tags | `["monitoring", "e-commerce"]` |
| `cdp_commands` | Yes | Array of CDP command objects | See below |

## CDP Command Reference

### Command Object Structure

Each CDP command in the `cdp_commands` array has:

```json
{
  "method": "Domain.method",
  "params": {
    "param1": "value1",
    "param2": "value2"
  },
  "save_as": "output.png"  // Optional: for screenshot/data capture
}
```

### Common CDP Methods

#### Navigation

**Page.navigate** - Navigate to URL:
```json
{
  "method": "Page.navigate",
  "params": {
    "url": "https://example.com"
  }
}
```

**Page.goBack** / **Page.goForward** - Browser history:
```json
{
  "method": "Page.goBack",
  "params": {}
}
```

**Page.reload** - Refresh page:
```json
{
  "method": "Page.reload",
  "params": {
    "ignoreCache": true
  }
}
```

#### Interaction

**Runtime.evaluate** - Execute JavaScript:
```json
{
  "method": "Runtime.evaluate",
  "params": {
    "expression": "document.querySelector('button').click()"
  }
}
```

**Input.dispatchMouseEvent** - Click at coordinates:
```json
{
  "method": "Input.dispatchMouseEvent",
  "params": {
    "type": "mousePressed",
    "x": 100,
    "y": 200,
    "button": "left",
    "clickCount": 1
  }
}
```

**Input.insertText** - Type text:
```json
{
  "method": "Input.insertText",
  "params": {
    "text": "Hello World"
  }
}
```

**Input.dispatchKeyEvent** - Press keys:
```json
{
  "method": "Input.dispatchKeyEvent",
  "params": {
    "type": "keyDown",
    "key": "Enter"
  }
}
```

#### Capture

**Page.captureScreenshot** - Take screenshot:
```json
{
  "method": "Page.captureScreenshot",
  "params": {
    "format": "png",
    "captureBeyondViewport": true,
    "clip": {
      "x": 0,
      "y": 0,
      "width": 800,
      "height": 600,
      "scale": 1
    }
  },
  "save_as": "screenshot.png"
}
```

**Runtime.evaluate** - Extract data:
```json
{
  "method": "Runtime.evaluate",
  "params": {
    "expression": "document.body.innerText",
    "returnByValue": true
  },
  "save_as": "page_text.txt"
}
```

#### DOM & Content

**DOM.getDocument** - Get DOM tree:
```json
{
  "method": "DOM.getDocument",
  "params": {
    "depth": -1,
    "pierce": true
  }
}
```

**DOM.querySelector** - Find element:
```json
{
  "method": "DOM.querySelector",
  "params": {
    "nodeId": 1,
    "selector": "button#submit"
  }
}
```

#### Network

**Network.getCookies** - Get cookies:
```json
{
  "method": "Network.getCookies",
  "params": {}
}
```

**Network.setCookie** - Set cookie:
```json
{
  "method": "Network.setCookie",
  "params": {
    "name": "session",
    "value": "abc123",
    "domain": "example.com"
  }
}
```

#### Emulation

**Emulation.setGeolocationOverride** - Set location:
```json
{
  "method": "Emulation.setGeolocationOverride",
  "params": {
    "latitude": 37.7749,
    "longitude": -122.4194,
    "accuracy": 100
  }
}
```

**Emulation.setDeviceMetricsOverride** - Mobile emulation:
```json
{
  "method": "Emulation.setDeviceMetricsOverride",
  "params": {
    "width": 375,
    "height": 667,
    "deviceScaleFactor": 2,
    "mobile": true
  }
}
```

## Creating Scripts with Claude

Instead of writing CDP JSON manually, describe your task in natural language:

### Example Interaction

**You:** "Check product prices on competitor.com/products and save them"

**Claude Generates:**

```json
{
  "name": "competitor-price-check",
  "description": "Monitor competitor product prices",
  "created": "2026-01-15T10:30:00Z",
  "author": "Claude",
  "tags": ["monitoring", "e-commerce", "prices"],
  "cdp_commands": [
    {
      "method": "Page.navigate",
      "params": {
        "url": "https://competitor.com/products"
      }
    },
    {
      "method": "Page.captureScreenshot",
      "params": {
        "format": "png",
        "captureBeyondViewport": true
      },
      "save_as": "products_page.png"
    },
    {
      "method": "Runtime.evaluate",
      "params": {
        "expression": "Array.from(document.querySelectorAll('.product-price')).map(el => el.textContent).join('\\n')",
        "returnByValue": true
      },
      "save_as": "prices.txt"
    }
  ]
}
```

**Robert saves this to:** `~/Robert/scripts/competitor-price-check.json`

**You can then:** Run it immediately, edit it, or schedule it

## Claude Integration Features

The AI-powered script generator:

1. **Natural language** - Describe task in plain English
2. **CDP expertise** - Claude knows Chrome DevTools Protocol
3. **Instant generation** - Outputs complete JSON script
4. **Editable** - Modify generated scripts before running
5. **Learn by example** - See how tasks translate to CDP

### Script Generation Flow

```
┌─────────────────────────────────────────────┐
│  User: "Navigate to X and click Y"         │
│  (Natural language description)              │
└────────────────┬────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────┐
│  Claude analyzes request                    │
│  - Identifies required CDP commands         │
│  - Sequences operations correctly           │
│  - Adds appropriate parameters              │
└────────────────┬────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────┐
│  Generates CDP JSON script                  │
│  Shows preview of commands                  │
└────────────────┬────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────┐
│  User reviews and can:                      │
│  - Run immediately                          │
│  - Edit JSON directly                       │
│  - Save for reuse                           │
│  - Share with others                        │
└─────────────────────────────────────────────┘
```

## Why CDP Instead of High-Level Abstraction?

### Direct CDP Access Advantages

**Full Chrome capabilities:**
- Any browser feature available via protocol
- No abstraction layer limitations
- Access to advanced features (emulation, performance, etc.)

**AI-friendly:**
- Well-documented protocol Claude understands
- Structured JSON format perfect for LLM generation
- Predictable command structure

**Power user flexibility:**
- Hand-edit scripts for advanced use cases
- Combine commands in creative ways
- Access cutting-edge Chrome features

**Future-proof:**
- CDP evolves with Chrome
- New features automatically available
- No wrapper library to maintain

## Advanced Features

### Variables

Store and reuse values:
```markdown
### Store page title
Get the page title and save it
- Extract from: title element
- Store as: page_title

### Use in filename
Take screenshot with dynamic name
- Filename: {page_title}_screenshot.png
```

### Loops (Future)

Repeat actions:
```markdown
### For each product
Repeat for all products on page
- Selector: .product-card
- Actions:
  - Extract product name
  - Extract product price
  - Save to: products.csv
```

### Conditionals

Branch based on conditions:
```markdown
### Check login status
If not logged in, log in first
- Condition: element missing
- Selector: .user-avatar
- Then:
  - Click login button
  - Fill login form
  - Submit
```

## File Organization

Store scripts in your Robert folder:

```
~/Robert/
├── scripts/
│   ├── daily/
│   │   ├── competitor-prices.md
│   │   └── website-monitor.md
│   ├── weekly/
│   │   └── backup-data.md
│   └── one-time/
│       └── data-extraction.md
└── outputs/
    ├── 2026-01-15_competitor-prices/
    │   ├── homepage.png
    │   └── prices.txt
    └── 2026-01-16_competitor-prices/
        ├── homepage.png
        └── prices.txt
```

## Community Scripts

Share and discover automations in the community library:

- **E-commerce**: Price monitoring, inventory tracking
- **Social Media**: Content scheduling, analytics collection
- **Research**: Data gathering, article archiving
- **Monitoring**: Website changes, uptime checking
- **Productivity**: Form filling, report generation

All scripts are Markdown files you can read, understand, and customize!

---

**Remember:** You never have to write Markdown manually. Just talk to Robert's Agent Creator and it writes the script for you!
