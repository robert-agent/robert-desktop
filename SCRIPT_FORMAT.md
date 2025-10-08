# Robert Automation Script Format

## Overview

Robert uses **Markdown** files for automation scripts, inspired by Claude's agent format. Scripts are human-readable, easy to understand, and can be created by **talking through your automation** with Robert's voice-driven agent creator.

## Format Specification

### File Structure

```markdown
---
name: script-name
description: What this automation does
created: 2026-01-15
author: Your Name
---

# Script Title

Brief description of what this automation accomplishes.

## Steps

### 1. Navigate to website
Go to https://example.com and wait for page to load

### 2. Take screenshot
Capture full page screenshot
- Filename: homepage.png
- Type: full-page

### 3. Extract content
Get text from the main heading
- Selector: h1
- Save to: title.txt

### 4. Click button
Click the "Get Started" button
- Selector: button#cta
- Wait: 2 seconds after click

### 5. Fill form
Enter information into the contact form
- Email field (input[name="email"]): user@example.com
- Message field (textarea#message): Hello from Robert!
- Clear existing text: yes

### 6. Submit and capture
Submit the form and take a screenshot of results
- Submit selector: button[type="submit"]
- Screenshot filename: results.png
```

### Frontmatter Fields

The YAML frontmatter (between `---` markers) contains metadata:

| Field | Required | Description | Example |
|-------|----------|-------------|---------|
| `name` | Yes | Unique identifier (lowercase, hyphenated) | `monitor-competitor-prices` |
| `description` | Yes | Brief description of what the script does | `Check competitor pricing daily` |
| `created` | Auto | Date script was created | `2026-01-15` |
| `author` | No | Who created the script | `Your Name` |
| `tags` | No | Comma-separated tags | `monitoring, e-commerce, daily` |
| `schedule` | No | When to run (future feature) | `daily at 9am` |

## Action Types

### Navigation Actions

**Navigate to URL:**
```markdown
### Navigate to website
Go to https://example.com
- Wait for: page load complete
- Timeout: 30 seconds
```

**Go back/forward:**
```markdown
### Go back
Navigate back to previous page
```

### Wait Actions

**Wait for time:**
```markdown
### Wait
Pause for 3 seconds
```

**Wait for element:**
```markdown
### Wait for element
Wait until the results appear
- Selector: .search-results
- Timeout: 10 seconds
```

### Click Actions

**Simple click:**
```markdown
### Click button
Click the submit button
- Selector: button#submit
```

**Click with wait:**
```markdown
### Click and wait
Click the "Next" button and wait for page to load
- Selector: a.next-page
- Wait after: 2 seconds
```

### Type/Fill Actions

**Type text:**
```markdown
### Enter search query
Type into the search box
- Selector: input[name="q"]
- Text: automation tools
- Clear first: yes
- Press enter: yes
```

**Fill form:**
```markdown
### Fill contact form
Enter form information
- Name field (input#name): John Doe
- Email field (input#email): john@example.com
- Message (textarea#message): Hello!
```

### Capture Actions

**Screenshot:**
```markdown
### Take screenshot
Capture the current page
- Filename: homepage.png
- Type: full-page
- Format: PNG
```

Options for `Type`:
- `viewport` - Visible area only
- `full-page` - Entire scrollable page
- `element` - Specific element only

**Extract text:**
```markdown
### Extract prices
Get all product prices from the page
- Selector: .product-price
- Save to: prices.txt
- Get all matches: yes
```

**Get page HTML:**
```markdown
### Save page source
Capture the raw HTML
- Save to: page.html
```

### Conditional Actions

**If element exists:**
```markdown
### Check for errors
If error message appears, take screenshot
- Condition: element exists
- Selector: .error-message
- Then:
  - Take screenshot
  - Filename: error.png
  - Stop execution: yes
```

**If text contains:**
```markdown
### Verify success
If page contains "Success", continue
- Condition: text contains
- Text: "Success"
- Then: continue
- Else: take screenshot and stop
```

### Scroll Actions

**Scroll page:**
```markdown
### Scroll down
Scroll down the page
- Amount: 500 pixels
- Direction: down
```

**Scroll to element:**
```markdown
### Scroll to footer
Scroll until footer is visible
- Selector: footer
```

## Creating Scripts with Voice

Instead of writing Markdown manually, use Robert's **Agent Creator** to talk through your automation:

### Example Voice Session

**You:** "I want to check product prices on a competitor's website"

**Robert:** "Great! Let me walk you through creating this automation. What's the website URL?"

**You:** "competitor.com/products"

**Robert:** "Got it. What do you want to do on that page?"

**You:** "Take a screenshot of the whole page, then extract all the prices"

**Robert:** "Perfect. Where do you want to save the prices?"

**You:** "Save them to a file called prices.txt"

**Robert:** "Excellent! I've created your automation. Here's what it will do:
1. Go to competitor.com/products
2. Take a full-page screenshot (homepage.png)
3. Extract all prices and save to prices.txt

Would you like to run it now, or edit anything?"

### The Generated Script

```markdown
---
name: competitor-price-check
description: Monitor competitor product prices
created: 2026-01-15
author: Voice Creator
tags: monitoring, e-commerce, prices
---

# Competitor Price Check

Monitors product prices on competitor.com/products to track pricing changes.

## Steps

### 1. Navigate to products page
Go to https://competitor.com/products
- Wait for: page load complete

### 2. Take screenshot
Capture full page screenshot
- Filename: homepage.png
- Type: full-page

### 3. Extract all prices
Get text from all price elements
- Selector: .product-price
- Save to: prices.txt
- Get all matches: yes
```

## Voice Creator Features

The voice-driven Agent Creator:

1. **Conversational** - Talk naturally, no special commands
2. **Guided** - Robert asks clarifying questions
3. **Previews** - Shows what it understood before creating
4. **Iterative** - Easily edit and refine
5. **Educational** - Learn automation by describing what you want

### Voice Creator Flow

```
┌─────────────────────────────────────────────┐
│  "I want to automate..."                    │
│  (User describes task in natural language)  │
└────────────────┬────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────┐
│  Robert asks clarifying questions           │
│  - What website?                            │
│  - What actions?                            │
│  - Where to save outputs?                   │
└────────────────┬────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────┐
│  Robert generates Markdown script           │
│  Shows preview with step-by-step breakdown  │
└────────────────┬────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────┐
│  User reviews and can:                      │
│  - Run immediately                          │
│  - Edit in visual editor                    │
│  - Refine with voice                        │
│  - Save for later                           │
└─────────────────────────────────────────────┘
```

## Markdown Advantages Over YAML

### Why Markdown Instead of YAML?

**YAML is scary for non-technical users:**
```yaml
# This looks intimidating!
steps:
  - action: navigate
    url: "https://example.com"
    wait_for: "dom_content_loaded"
  - action: screenshot
    type: "full_page"
    filename: "homepage.png"
```

**Markdown is readable by everyone:**
```markdown
### Navigate to website
Go to https://example.com and wait for page to load

### Take screenshot
Capture full page screenshot
- Filename: homepage.png
- Type: full-page
```

**Key Advantages:**

1. **Readable as documentation** - Scripts double as human-readable docs
2. **No syntax errors** - More forgiving format
3. **Familiar format** - Everyone has seen Markdown
4. **Easy to edit** - Text editor or Robert's visual editor
5. **Natural structure** - Headings = steps, bullets = options
6. **Community friendly** - Easy to share and understand

### Inspired by Claude Agents

Robert's format is inspired by Claude's agent definitions:

**Claude Agent:**
```markdown
---
name: code-reviewer
description: Expert code review specialist
---
You are a senior code reviewer ensuring high standards...
```

**Robert Automation:**
```markdown
---
name: competitor-monitor
description: Track competitor website changes
---
# Competitor Monitor

Automatically check competitor prices daily...

### 1. Navigate to site
Go to competitor.com...
```

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
