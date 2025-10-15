# Step-by-Step Frame Format

## Overview
A standardized JSON format for capturing detailed interaction frames during web workflows. Each frame represents a single moment in time with a screenshot, DOM state, and transcript of what the user/agent is doing.

## File Naming Convention
```
{domain}_{workflow_name}_{session_id}_v{version}.frames.json
```

Examples:
- `github.com_create_repository_session_a1b2c3_v1.frames.json`
- `gmail.com_compose_email_session_x9y8z7_v2.frames.json`

## Schema

### Root Structure
```json
{
  "metadata": {
    "domain": "string",
    "workflow_name": "string",
    "session_id": "string",
    "version": "string",
    "created": "ISO 8601 timestamp",
    "agent_id": "string",
    "success": boolean,
    "total_frames": number,
    "total_duration_ms": number,
    "starting_url": "string",
    "ending_url": "string",
    "user_agent": "string",
    "viewport": {
      "width": number,
      "height": number
    },
    "tags": ["string"]
  },
  "frames": [
    // Array of frame objects (see Frame Schema below)
  ],
  "summary": {
    "goal": "string",
    "outcome": "string",
    "errors_encountered": [],
    "recovery_actions": [],
    "learned_insights": []
  }
}
```

### Frame Schema
```json
{
  "frame_id": number,
  "timestamp": "ISO 8601 timestamp",
  "elapsed_ms": number,
  "

  // Visual State
  "screenshot": {
    "path": "string (relative path to PNG file)",
    "format": "png | jpeg | webp",
    "size_bytes": number,
    "dimensions": {
      "width": number,
      "height": number
    },
    "hash": "string (SHA-256 for deduplication)"
  },

  // DOM State
  "dom": {
    "url": "string",
    "title": "string",
    "html_path": "string (relative path to HTML file)",
    "html_hash": "string (SHA-256)",
    "interactive_elements": [
      {
        "selector": "string (CSS selector)",
        "tag": "string",
        "type": "string (input type, if applicable)",
        "text": "string (visible text)",
        "attributes": {
          "id": "string",
          "class": "string",
          "aria-label": "string",
          "data-*": "string"
        },
        "bbox": {
          "x": number,
          "y": number,
          "width": number,
          "height": number
        },
        "is_visible": boolean,
        "is_enabled": boolean
      }
    ],
    "forms": [
      {
        "selector": "string",
        "action": "string (form action URL)",
        "method": "string (GET/POST)",
        "fields": [
          {
            "name": "string",
            "type": "string",
            "selector": "string",
            "value": "string (current value)",
            "required": boolean
          }
        ]
      }
    ],
    "modals": [
      {
        "selector": "string",
        "is_open": boolean,
        "content_preview": "string"
      }
    ]
  },

  // User/Agent Action
  "action": {
    "type": "navigate | click | type | select | hover | scroll | wait | keystroke",
    "target": {
      "selector": "string (CSS selector)",
      "xpath": "string (alternative locator)",
      "text": "string (visible text on element)",
      "coordinates": {
        "x": number,
        "y": number
      }
    },
    "input": {
      "value": "string (for type/select actions)",
      "masked": boolean,
      "key": "string (for keystroke actions)"
    },
    "intent": "string (high-level description)",
    "confidence": number,
    "alternatives_considered": [
      {
        "selector": "string",
        "reason_rejected": "string",
        "confidence": number
      }
    ]
  },

  // Transcript (Natural Language)
  "transcript": {
    "action_description": "string (what is happening)",
    "reasoning": "string (why this action was chosen)",
    "expected_outcome": "string (what should happen next)",
    "observations": ["string (any notable observations)"]
  },

  // State Changes
  "state_changes": {
    "url_changed": boolean,
    "dom_mutations": number,
    "network_requests": [
      {
        "url": "string",
        "method": "string",
        "status": number,
        "type": "xhr | fetch | navigation | document",
        "duration_ms": number
      }
    ],
    "console_messages": [
      {
        "level": "log | warn | error",
        "message": "string",
        "timestamp": "ISO 8601"
      }
    ],
    "cookies_changed": boolean,
    "storage_changed": boolean
  },

  // Verification
  "verification": {
    "action_succeeded": boolean,
    "success_indicators": ["string"],
    "failure_indicators": ["string"],
    "wait_conditions_met": boolean,
    "unexpected_changes": ["string"]
  },

  // Learning Data
  "learning": {
    "selector_stability": number,
    "action_reliability": number,
    "alternative_selectors": ["string"],
    "notes": "string (agent's notes for future improvement)"
  }
}
```

## Complete Example

```json
{
  "metadata": {
    "domain": "github.com",
    "workflow_name": "create_repository",
    "session_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "version": "1.0.0",
    "created": "2025-10-09T14:32:15.123Z",
    "agent_id": "agent-claude-001",
    "success": true,
    "total_frames": 8,
    "total_duration_ms": 12450,
    "starting_url": "https://github.com",
    "ending_url": "https://github.com/username/new-repo",
    "user_agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36",
    "viewport": {
      "width": 1920,
      "height": 1080
    },
    "tags": ["create", "repository", "successful"]
  },

  "frames": [
    {
      "frame_id": 0,
      "timestamp": "2025-10-09T14:32:15.123Z",
      "elapsed_ms": 0,

      "screenshot": {
        "path": "./screenshots/frame_0000.png",
        "format": "png",
        "size_bytes": 245678,
        "dimensions": {
          "width": 1920,
          "height": 1080
        },
        "hash": "a1b2c3d4e5f6789..."
      },

      "dom": {
        "url": "https://github.com",
        "title": "GitHub: Let's build from here",
        "html_path": "./dom/frame_0000.html",
        "html_hash": "x9y8z7w6v5u4...",
        "interactive_elements": [
          {
            "selector": "[data-test-selector='global-create-menu-button']",
            "tag": "button",
            "type": null,
            "text": "+",
            "attributes": {
              "id": "create-menu-button",
              "class": "btn btn-primary",
              "aria-label": "Create new...",
              "data-test-selector": "global-create-menu-button"
            },
            "bbox": {
              "x": 1745,
              "y": 22,
              "width": 32,
              "height": 32
            },
            "is_visible": true,
            "is_enabled": true
          }
        ],
        "forms": [],
        "modals": []
      },

      "action": {
        "type": "click",
        "target": {
          "selector": "[data-test-selector='global-create-menu-button']",
          "xpath": "//button[@data-test-selector='global-create-menu-button']",
          "text": "+",
          "coordinates": {
            "x": 1761,
            "y": 38
          }
        },
        "input": null,
        "intent": "Open the create menu to access repository creation option",
        "confidence": 0.95,
        "alternatives_considered": [
          {
            "selector": "a[href='/new']",
            "reason_rejected": "Direct link not visible on homepage",
            "confidence": 0.45
          }
        ]
      },

      "transcript": {
        "action_description": "Clicking the '+' button in the top navigation to open the create menu",
        "reasoning": "The create menu provides access to creating new repositories, which is the first step in the workflow. The button is clearly visible and has a stable data-test-selector attribute.",
        "expected_outcome": "A dropdown menu should appear with options including 'New repository'",
        "observations": [
          "User is already logged in (avatar visible in top right)",
          "Page loaded successfully with no errors",
          "Create button is in expected location"
        ]
      },

      "state_changes": {
        "url_changed": false,
        "dom_mutations": 0,
        "network_requests": [],
        "console_messages": [],
        "cookies_changed": false,
        "storage_changed": false
      },

      "verification": {
        "action_succeeded": null,
        "success_indicators": [],
        "failure_indicators": [],
        "wait_conditions_met": true,
        "unexpected_changes": []
      },

      "learning": {
        "selector_stability": 0.98,
        "action_reliability": 0.96,
        "alternative_selectors": [
          "#create-menu-button",
          "button[aria-label='Create new...']"
        ],
        "notes": "data-test-selector appears to be most stable. ID may change."
      }
    },

    {
      "frame_id": 1,
      "timestamp": "2025-10-09T14:32:15.456Z",
      "elapsed_ms": 333,

      "screenshot": {
        "path": "./screenshots/frame_0001.png",
        "format": "png",
        "size_bytes": 248123,
        "dimensions": {
          "width": 1920,
          "height": 1080
        },
        "hash": "b2c3d4e5f6g7890..."
      },

      "dom": {
        "url": "https://github.com",
        "title": "GitHub: Let's build from here",
        "html_path": "./dom/frame_0001.html",
        "html_hash": "y8z7w6v5u4t3...",
        "interactive_elements": [
          {
            "selector": "[data-test-selector='global-create-menu'] a[href='/new']",
            "tag": "a",
            "type": null,
            "text": "New repository",
            "attributes": {
              "href": "/new",
              "class": "dropdown-item",
              "data-test-selector": "create-repo-menu-item"
            },
            "bbox": {
              "x": 1645,
              "y": 58,
              "width": 180,
              "height": 36
            },
            "is_visible": true,
            "is_enabled": true
          }
        ],
        "forms": [],
        "modals": []
      },

      "action": {
        "type": "click",
        "target": {
          "selector": "[data-test-selector='global-create-menu'] a[href='/new']",
          "xpath": "//div[@data-test-selector='global-create-menu']//a[@href='/new']",
          "text": "New repository",
          "coordinates": {
            "x": 1735,
            "y": 76
          }
        },
        "input": null,
        "intent": "Navigate to the repository creation form",
        "confidence": 0.98,
        "alternatives_considered": []
      },

      "transcript": {
        "action_description": "Clicking 'New repository' option from the dropdown menu",
        "reasoning": "The dropdown menu has appeared as expected, and the 'New repository' link is the correct option to create a new repository. This is a standard navigation action.",
        "expected_outcome": "Browser should navigate to /new page with repository creation form",
        "observations": [
          "Dropdown menu appeared within 333ms",
          "Menu contains multiple options (New repository, New gist, etc.)",
          "Target element is clearly visible and clickable"
        ]
      },

      "state_changes": {
        "url_changed": false,
        "dom_mutations": 15,
        "network_requests": [],
        "console_messages": [],
        "cookies_changed": false,
        "storage_changed": false
      },

      "verification": {
        "action_succeeded": true,
        "success_indicators": [
          "Dropdown menu visible",
          "Target element present and enabled"
        ],
        "failure_indicators": [],
        "wait_conditions_met": true,
        "unexpected_changes": []
      },

      "learning": {
        "selector_stability": 0.94,
        "action_reliability": 0.97,
        "alternative_selectors": [
          "a[href='/new']",
          ".dropdown-item:contains('New repository')"
        ],
        "notes": "Menu structure is stable. Simple href selector works but combining with menu selector is more robust."
      }
    },

    {
      "frame_id": 2,
      "timestamp": "2025-10-09T14:32:16.892Z",
      "elapsed_ms": 1769,

      "screenshot": {
        "path": "./screenshots/frame_0002.png",
        "format": "png",
        "size_bytes": 312456,
        "dimensions": {
          "width": 1920,
          "height": 1080
        },
        "hash": "c3d4e5f6g7h8901..."
      },

      "dom": {
        "url": "https://github.com/new",
        "title": "Create a new repository",
        "html_path": "./dom/frame_0002.html",
        "html_hash": "z7w6v5u4t3s2...",
        "interactive_elements": [
          {
            "selector": "input[data-testid='repository-name-input']",
            "tag": "input",
            "type": "text",
            "text": "",
            "attributes": {
              "id": "repository-name",
              "name": "repository[name]",
              "data-testid": "repository-name-input",
              "aria-label": "Repository name",
              "autocomplete": "off"
            },
            "bbox": {
              "x": 412,
              "y": 284,
              "width": 568,
              "height": 40
            },
            "is_visible": true,
            "is_enabled": true
          }
        ],
        "forms": [
          {
            "selector": "form[action='/repositories']",
            "action": "/repositories",
            "method": "POST",
            "fields": [
              {
                "name": "repository[name]",
                "type": "text",
                "selector": "input[data-testid='repository-name-input']",
                "value": "",
                "required": true
              },
              {
                "name": "repository[description]",
                "type": "text",
                "selector": "input#repository-description",
                "value": "",
                "required": false
              }
            ]
          }
        ],
        "modals": []
      },

      "action": {
        "type": "type",
        "target": {
          "selector": "input[data-testid='repository-name-input']",
          "xpath": "//input[@data-testid='repository-name-input']",
          "text": "",
          "coordinates": {
            "x": 696,
            "y": 304
          }
        },
        "input": {
          "value": "my-awesome-project",
          "masked": false,
          "key": null
        },
        "intent": "Enter the desired repository name",
        "confidence": 0.99,
        "alternatives_considered": []
      },

      "transcript": {
        "action_description": "Typing 'my-awesome-project' into the repository name field",
        "reasoning": "The form has loaded successfully and the repository name input is the first required field. The name follows GitHub's naming conventions (lowercase, hyphens allowed).",
        "expected_outcome": "Text should appear in the input field, and GitHub will check name availability",
        "observations": [
          "Navigation to /new was successful (took 1.4s)",
          "Form is fully loaded and interactive",
          "Repository name field is focused automatically",
          "No validation errors present"
        ]
      },

      "state_changes": {
        "url_changed": true,
        "dom_mutations": 45,
        "network_requests": [
          {
            "url": "https://github.com/new",
            "method": "GET",
            "status": 200,
            "type": "navigation",
            "duration_ms": 1436
          }
        ],
        "console_messages": [],
        "cookies_changed": false,
        "storage_changed": false
      },

      "verification": {
        "action_succeeded": true,
        "success_indicators": [
          "URL changed to /new",
          "Form loaded successfully",
          "Input field is present and enabled"
        ],
        "failure_indicators": [],
        "wait_conditions_met": true,
        "unexpected_changes": []
      },

      "learning": {
        "selector_stability": 0.99,
        "action_reliability": 0.98,
        "alternative_selectors": [
          "#repository-name",
          "input[name='repository[name]']"
        ],
        "notes": "data-testid selector is very stable. Name attribute also reliable but less specific."
      }
    }
  ],

  "summary": {
    "goal": "Create a new GitHub repository named 'my-awesome-project'",
    "outcome": "Successfully created repository at https://github.com/username/my-awesome-project",
    "errors_encountered": [],
    "recovery_actions": [],
    "learned_insights": [
      "data-test-selector attributes are most stable for GitHub navigation",
      "Repository creation form auto-focuses name input",
      "Network idle wait of 2s is sufficient for page transitions",
      "No rate limiting encountered during this session"
    ]
  }
}
```

---

## Directory Structure for Frames

```
workflows/
├── github.com/
│   ├── create_repository/
│   │   ├── session_a1b2c3/
│   │   │   ├── github.com_create_repository_session_a1b2c3_v1.frames.json
│   │   │   ├── screenshots/
│   │   │   │   ├── frame_0000.png
│   │   │   │   ├── frame_0001.png
│   │   │   │   └── frame_0002.png
│   │   │   └── dom/
│   │   │       ├── frame_0000.html
│   │   │       ├── frame_0001.html
│   │   │       └── frame_0002.html
│   │   ├── session_x9y8z7/
│   │   │   └── ...
│   │   └── github.com_create_repository_v1.workflow.md
│   └── create_issue/
│       └── ...
└── gmail.com/
    └── ...
```

---

## Usage Guidelines for Agents

### Recording a Session

1. **Initialize session metadata** before starting workflow
2. **Capture frame before each action**:
   - Take screenshot
   - Save DOM snapshot
   - Extract interactive elements
   - Record current state
3. **Execute action and record details**:
   - Action type and target
   - Input data (mask sensitive info)
   - Reasoning and intent
4. **Capture frame after action**:
   - New screenshot
   - DOM changes
   - Network activity
   - Verification results
5. **Update metadata** when session completes:
   - Total frames
   - Duration
   - Success status

### Deduplication

- Use screenshot hashes to detect duplicate frames
- Use DOM hashes to detect unchanged pages
- Skip frames where hash matches previous (unless action occurred)

### Compression

For storage efficiency:
- Store screenshots as PNG with compression level 6
- Only store full HTML for frames where significant DOM changes occurred
- For other frames, store DOM diff from previous frame
- Consider storing screenshots at lower quality after 30 days

### Privacy & Security

- **Always mask sensitive input**: passwords, API keys, credit cards, SSN
- Set `masked: true` for any sensitive input fields
- Don't store actual sensitive values in `input.value`
- Use placeholder like `"***MASKED***"` for sensitive fields
- Redact sensitive info from screenshots (overlay black boxes)

### Learning from Frames

Agents should analyze frames to:
1. **Identify selector stability**: Track which selectors break vs. which are stable
2. **Measure action reliability**: Calculate success rates for each action type
3. **Discover alternative paths**: Find multiple ways to accomplish same goal
4. **Detect patterns**: Learn common sequences and transitions
5. **Build confidence scores**: Update based on empirical success/failure
6. **Generate workflow graphs**: Extract nodes and edges from successful sessions

### Merging Frame Data

When combining insights from multiple sessions:
1. Aggregate selector stability scores across all sessions
2. Identify most reliable selectors (highest stability + success rate)
3. Discover edge cases and error scenarios
4. Build comprehensive alternative path library
5. Update workflow graph with learned optimizations

### Frame Analysis Queries

Example queries agents can run on frame data:

**Q: What's the most reliable selector for the login button?**
```
Filter frames where action.type = "click" AND action.intent contains "login"
Group by action.target.selector
Calculate: avg(verification.action_succeeded), avg(learning.selector_stability)
Return: selector with highest combined score
```

**Q: What are common failure modes for form submission?**
```
Filter frames where action.type = "click" AND action.intent contains "submit"
  AND verification.action_succeeded = false
Group by verification.failure_indicators
Count occurrences
Return: top failure indicators with recovery actions
```

**Q: How long does page X typically take to load?**
```
Filter frames where url changed to X
Calculate: avg(state_changes.network_requests[type="navigation"].duration_ms)
Return: average load time
```
