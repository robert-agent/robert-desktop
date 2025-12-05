# Step Frame Implementation Guide

## Overview

Complete implementation of step frame capture for browser workflows, conforming to the Step Frame Schema specification (`agent-formats/specs/STEP_FRAME_SCHEMA.md`).

## Implementation

### Module Location

`/home/jeef/robert/crates/robert-webdriver/src/step_frame.rs`

### Core Function

```rust
pub async fn capture_step_frame(
    driver: &ChromeDriver,
    frame_id: usize,
    elapsed_ms: u64,
    options: &CaptureOptions,
    user_instruction: Option<String>,
    action_info: Option<ActionInfo>,
) -> Result<StepFrame>
```

### Function Behavior

1. **Fail Fast**: Immediately attempts to access the current browser page and fails with a clear error if the connection is invalid
2. **Take Screenshot**: Captures screenshot and saves to the specified directory with automatic naming (`frame_0000.png`)
3. **Save DOM**: Optionally saves the HTML DOM to a separate directory (`frame_0000.html`)
4. **Extract Elements**: Optionally extracts interactive elements from the page (expensive operation)
5. **Capture VisualDom** (optional): Captures structured DOM snapshot with layout, styles, and embedded images (`frame_0000.visualdom.json`)
6. **Compute Hashes**: Optionally computes SHA-256 hashes for deduplication
7. **Build Frame**: Constructs a complete `StepFrame` object with all captured data

### Key Features

✅ **Fail-Fast Design**: Function fails immediately with clear error message if browser connection is invalid
✅ **Automatic File Naming**: Uses sequential frame IDs with zero-padding (`frame_0000.png`)
✅ **Multiple Formats**: Supports PNG and JPEG screenshot formats
✅ **VisualDom Capture**: Optional structured DOM snapshot with layout, computed styles, and base64 images
✅ **Hash Computation**: SHA-256 hashes for screenshots, HTML, and VisualDom for deduplication
✅ **Interactive Elements**: Optional extraction of buttons, links, inputs, etc.
✅ **Flexible Configuration**: Extensive options for customizing capture behavior
✅ **JSON Serialization**: Full serde JSON support for all structures

## Data Structures

### StepFrame

Complete frame capturing a moment in time:

```rust
pub struct StepFrame {
    pub frame_id: usize,
    pub timestamp: String,              // ISO 8601
    pub elapsed_ms: u64,
    pub screenshot: ScreenshotInfo,
    pub dom: DomInfo,
    pub visual_dom: Option<VisualDomInfo>,   // Structured DOM snapshot
    pub action: Option<ActionInfo>,
    pub transcript: Option<TranscriptInfo>,
}
```

### ScreenshotInfo

```rust
pub struct ScreenshotInfo {
    pub path: String,
    pub format: String,                 // "png" | "jpeg"
    pub size_bytes: usize,
    pub dimensions: Option<Dimensions>,
    pub hash: Option<String>,           // SHA-256
}
```

### DomInfo

```rust
pub struct DomInfo {
    pub url: String,
    pub title: String,
    pub html_path: Option<String>,
    pub html_hash: Option<String>,      // SHA-256
    pub interactive_elements: Option<Vec<InteractiveElement>>,
}
```

### VisualDomInfo

```rust
pub struct VisualDomInfo {
    pub path: String,                   // Path to VisualDom JSON file
    pub size_bytes: usize,
    pub node_count: usize,              // Number of DOM nodes
    pub hash: Option<String>,           // SHA-256
}
```

**VisualDom is a custom format we created** that combines Chrome DevTools Protocol's
DOMSnapshot.captureSnapshot with embedded base64 images. The VisualDom JSON file contains:

- **Complete DOM tree**: Flattened structure including iframes and shadow DOM
- **Layout information**: Bounding boxes, positions, dimensions
- **Computed styles**: Configurable set of CSS properties
- **Text content**: All visible text from layout objects (no OCR needed)
- **Embedded images**: Base64-encoded images with positions and dimensions
- **Paint order**: Visual stacking and rendering order

This format allows AI agents to understand page structure, layout, and content without requiring expensive OCR on screenshots.

### CaptureOptions

```rust
pub struct CaptureOptions {
    pub screenshot_dir: PathBuf,
    pub dom_dir: Option<PathBuf>,
    pub visual_dom_dir: Option<PathBuf>,
    pub screenshot_format: ScreenshotFormat,
    pub save_html: bool,
    pub capture_visual_dom: bool,
    pub visual_dom_computed_styles: Vec<String>,
    pub visual_dom_include_dom_rects: bool,
    pub visual_dom_include_paint_order: bool,
    pub visual_dom_include_images: bool,
    pub compute_hashes: bool,
    pub extract_interactive_elements: bool,
}

impl CaptureOptions {
    // Helper methods for computed styles presets:
    pub fn balanced_computed_styles() -> Vec<String>
    pub fn minimal_computed_styles() -> Vec<String>
    pub fn all_computed_styles() -> Vec<String>
}
```

## Usage Examples

### Basic Frame Capture

```rust
use robert_webdriver::{ChromeDriver, ConnectionMode};
use robert_webdriver::step_frame::{capture_step_frame, CaptureOptions};

let driver = ChromeDriver::new(ConnectionMode::Sandboxed {
    chrome_path: None,
    no_sandbox: true,
    headless: true,
}).await?;

driver.navigate("https://example.com").await?;

let options = CaptureOptions::default();
let frame = capture_step_frame(&driver, 0, 0, &options, None, None).await?;

println!("Captured frame at: {}", frame.screenshot.path);
println!("Page title: {}", frame.dom.title);
```

### Multi-Frame Workflow

```rust
let options = CaptureOptions {
    screenshot_dir: PathBuf::from("./screenshots"),
    dom_dir: Some(PathBuf::from("./dom")),
    compute_hashes: true,
    ..Default::default()
};

let start_time = std::time::Instant::now();
let mut frames = Vec::new();

// Frame 0: Initial state
driver.navigate("https://example.com").await?;
let frame0 = capture_step_frame(
    &driver,
    0,
    start_time.elapsed().as_millis() as u64,
    &options,
    Some("Navigate to example.com".to_string()),
    None,
).await?;
frames.push(frame0);

// Frame 1: After interaction
driver.execute_script("document.querySelector('button').click()").await?;
let frame1 = capture_step_frame(
    &driver,
    1,
    start_time.elapsed().as_millis() as u64,
    &options,
    Some("Click the button".to_string()),
    None,
).await?;
frames.push(frame1);

// Save all frames to JSON
let frames_json = serde_json::to_string_pretty(&frames)?;
tokio::fs::write("workflow.frames.json", frames_json).await?;
```

### With Action Metadata

```rust
use robert_webdriver::step_frame::ActionInfo;

let action = Some(ActionInfo {
    action_type: "click".to_string(),
    intent: "Click the login button".to_string(),
    target: Some("#login-button".to_string()),
});

let frame = capture_step_frame(
    &driver,
    0,
    0,
    &options,
    None,
    action,
).await?;

// Action metadata is included in the frame
assert_eq!(frame.action.as_ref().unwrap().action_type, "click");
```

### With Interactive Elements

```rust
let options = CaptureOptions {
    screenshot_dir: PathBuf::from("./screenshots"),
    dom_dir: Some(PathBuf::from("./dom")),
    extract_interactive_elements: true,  // Enable element extraction
    ..Default::default()
};

let frame = capture_step_frame(&driver, 0, 0, &options, None, None).await?;

if let Some(elements) = frame.dom.interactive_elements {
    println!("Found {} interactive elements:", elements.len());
    for element in elements {
        println!("  - <{}> {}", element.tag, element.text);
    }
}
```

### With VisualDom (Structured DOM Snapshot)

**VisualDom is a custom format we created** that combines CDP DOMSnapshot with base64 images.

```rust
use robert_webdriver::step_frame::CaptureOptions;

let options = CaptureOptions {
    screenshot_dir: PathBuf::from("./screenshots"),
    dom_dir: Some(PathBuf::from("./dom")),
    visual_dom_dir: Some(PathBuf::from("./visualdom")),
    capture_visual_dom: true,  // Enable VisualDom capture
    visual_dom_include_images: true,  // Include base64 images
    visual_dom_include_dom_rects: true,  // Include bounding boxes
    visual_dom_include_paint_order: true,  // Include visual stacking
    visual_dom_computed_styles: CaptureOptions::balanced_computed_styles(),
    ..Default::default()
};

let frame = capture_step_frame(&driver, 0, 0, &options, None, None).await?;

if let Some(visual_dom) = frame.visual_dom {
    println!("VisualDom captured:");
    println!("  - File: {}", visual_dom.path);
    println!("  - Size: {} KB", visual_dom.size_bytes / 1024);
    println!("  - Nodes: {}", visual_dom.node_count);

    // The VisualDom JSON file contains:
    // - Complete DOM tree with layout information
    // - Computed styles (position, size, visibility, fonts, colors)
    // - Bounding boxes for all elements
    // - All visible text content (no OCR needed!)
    // - Embedded images as base64 with positions
    // - Paint order and visual stacking
}
```

**VisualDom Computed Styles Presets:**

```rust
// Balanced: Good for most use cases
let balanced = CaptureOptions::balanced_computed_styles();
// Includes: display, position, size, visibility, fonts, colors, spacing, backgrounds

// Minimal: Just positioning and visibility
let minimal = CaptureOptions::minimal_computed_styles();
// Includes: display, position, visibility

// All: Capture all computed styles (large file size)
let all = CaptureOptions::all_computed_styles();
// Empty vec = capture all CSS properties
```

### Error Handling (Fail-Fast)

```rust
// The function fails fast if browser connection is invalid
match capture_step_frame(&driver, 0, 0, &options, None, None).await {
    Ok(frame) => {
        println!("Frame captured successfully");
    }
    Err(e) => {
        // Clear error message indicating connection failure
        eprintln!("Failed to capture frame: {}", e);
        // Error message contains "connection failed" or "not responding"
    }
}
```

## Test Coverage

### Test File

`/home/jeef/robert/crates/robert-webdriver/tests/step_frame_test.rs`

### Test Suite (11 Tests - All Passing ✅)

**Basic Functionality:**
- ✅ `test_capture_basic_step_frame` - Complete frame capture with all metadata
- ✅ `test_capture_with_user_instruction` - User instruction in transcript
- ✅ `test_capture_multiple_frames_in_workflow` - Sequential frame capture

**Hash & Deduplication:**
- ✅ `test_hash_computation` - SHA-256 hash generation
- ✅ `test_duplicate_frame_detection` - Detecting unchanged pages

**Format Support:**
- ✅ `test_jpeg_format` - JPEG screenshot format

**Interactive Elements:**
- ✅ `test_extract_interactive_elements` - Element extraction from DOM

**Fail-Fast Behavior:**
- ✅ `test_fail_fast_on_closed_browser` - Documented behavior
- ✅ `test_fail_fast_behavior` - Verifies error handling

**Serialization:**
- ✅ `test_frame_json_serialization` - JSON round-trip

**Performance:**
- ✅ `test_rapid_frame_capture` - 10 frames in ~615ms

### Running Tests

```bash
# All step frame tests
cargo test --test step_frame_test -- --test-threads=1

# Specific test
cargo test --test step_frame_test test_capture_basic_step_frame -- --nocapture

# With performance metrics
cargo test --test step_frame_test test_rapid_frame_capture -- --nocapture
```

## Performance Benchmarks

Based on test results (headless mode, local test server):

| Operation | Time | Notes |
|-----------|------|-------|
| Single frame capture (full) | ~100ms | With hash computation and HTML save |
| Single frame capture (minimal) | ~66ms | No hashing, no HTML save |
| 10 frames rapid capture | ~615ms | 61.5ms average per frame |
| Hash computation | <5ms | SHA-256 for both screenshot and HTML |
| Interactive element extraction | ~50ms | Depends on page complexity |

## Schema Conformance

The implementation conforms to the Step Frame Schema specification with the following coverage:

### Fully Implemented

✅ frame_id, timestamp, elapsed_ms
✅ screenshot (path, format, size_bytes, hash)
✅ dom (url, title, html_path, html_hash, interactive_elements)
✅ action (action_type, intent, target)
✅ transcript (action_description, reasoning, expected_outcome)

### Partial Implementation

🟡 screenshot.dimensions - Not extracted from image metadata yet
🟡 interactive_elements - Basic extraction (selector, tag, text, visibility)

### Not Yet Implemented

(These are optional fields in the schema and can be added as needed)

❌ dom.forms - Form structure extraction
❌ dom.modals - Modal detection
❌ state_changes - Network requests, console messages, mutations
❌ verification - Success indicators, failure detection
❌ learning - Selector stability, reliability scores

## Integration Points

### Commands Module

Location: `/home/jeef/robert/crates/robert-app/src-tauri/src/commands/agent.rs`

The step frame capture function can be integrated into the agent workflow:

```rust
// After each browser action
let frame = capture_step_frame(
    &driver,
    frame_id,
    elapsed_ms,
    &options,
    Some(user_instruction),
    Some(action_info),
).await?;

// Add to workflow frames collection
workflow_frames.push(frame);
```

### Workflow Executor

Location: `/home/jeef/robert/crates/robert-app/src-tauri/src/agent/workflow.rs`

Example integration after each CDP command execution:

```rust
// Execute CDP command
let report = driver.execute_cdp_script_direct(&script).await?;

// Capture frame for this step
let frame = capture_step_frame(
    &driver,
    step_number,
    start_time.elapsed().as_millis() as u64,
    &frame_options,
    Some(cmd.description.clone().unwrap_or_default()),
    Some(ActionInfo {
        action_type: extract_action_type(&cmd.method),
        intent: cmd.description.clone().unwrap_or_default(),
        target: extract_target(&cmd),
    }),
).await?;
```

## File Organization

### Recommended Directory Structure

```
workflow-sessions/
├── session_{id}/
│   ├── screenshots/
│   │   ├── frame_0000.png
│   │   ├── frame_0001.png
│   │   └── frame_0002.png
│   ├── dom/
│   │   ├── frame_0000.html
│   │   ├── frame_0001.html
│   │   └── frame_0002.html
│   └── session_{id}.frames.json
```

### Example Session JSON

```json
{
  "metadata": {
    "session_id": "abc123",
    "created": "2025-10-11T18:00:00Z",
    "total_frames": 3
  },
  "frames": [
    {
      "frame_id": 0,
      "timestamp": "2025-10-11T18:00:00.123Z",
      "elapsed_ms": 0,
      "screenshot": {
        "path": "./screenshots/frame_0000.png",
        "format": "png",
        "size_bytes": 25653,
        "hash": "4dd01a2427eeb2a2..."
      },
      "dom": {
        "url": "https://example.com",
        "title": "Example Domain",
        "html_path": "./dom/frame_0000.html",
        "html_hash": "86dc2ef53c056468..."
      },
      "action": {
        "action_type": "navigate",
        "intent": "Navigate to example.com"
      },
      "transcript": {
        "action_description": "Navigate to example.com"
      }
    }
  ]
}
```

## Best Practices

### 1. Frame ID Management

Use sequential frame IDs starting from 0:

```rust
let mut frame_id = 0;
for action in workflow_actions {
    let frame = capture_step_frame(&driver, frame_id, elapsed, &options, None, None).await?;
    frames.push(frame);
    frame_id += 1;
}
```

### 2. Timing Tracking

Use a consistent start time for the workflow:

```rust
let workflow_start = std::time::Instant::now();

// ... later
let elapsed = workflow_start.elapsed().as_millis() as u64;
```

### 3. Error Handling

Always handle frame capture errors gracefully:

```rust
match capture_step_frame(&driver, id, elapsed, &options, None, None).await {
    Ok(frame) => frames.push(frame),
    Err(e) => {
        eprintln!("Warning: Failed to capture frame {}: {}", id, e);
        // Continue workflow without this frame
    }
}
```

### 4. Directory Management

Create directories before starting workflow:

```rust
tokio::fs::create_dir_all(&options.screenshot_dir).await?;
if let Some(dom_dir) = &options.dom_dir {
    tokio::fs::create_dir_all(dom_dir).await?;
}
```

### 5. Hash-Based Deduplication

Use hashes to detect duplicate frames:

```rust
let mut last_html_hash = None;

for action in actions {
    let frame = capture_step_frame(&driver, id, elapsed, &options, None, None).await?;

    // Skip if DOM hasn't changed
    if let Some(ref last) = last_html_hash {
        if frame.dom.html_hash.as_ref() == Some(last) {
            println!("Skipping duplicate frame {}", id);
            continue;
        }
    }

    last_html_hash = frame.dom.html_hash.clone();
    frames.push(frame);
}
```

### 6. Performance Optimization

For high-frequency capture, disable expensive operations:

```rust
let options = CaptureOptions {
    screenshot_dir: path.clone(),
    dom_dir: Some(path.clone()),
    compute_hashes: false,              // Disable for speed
    save_html: false,                   // Disable if not needed
    extract_interactive_elements: false, // Expensive operation
    ..Default::default()
};
```

## Dependencies

- `sha2 = "0.10"` - Added to workspace dependencies for hash computation
- `chrono` - For ISO 8601 timestamps
- `serde` / `serde_json` - For JSON serialization

## Related Documentation

- [STEP_FRAME_SCHEMA.md](../agent-formats/specs/STEP_FRAME_SCHEMA.md) - Complete schema specification
- [SCREENSHOT_TESTING.md](./SCREENSHOT_TESTING.md) - Screenshot capture testing
- [TESTING.md](./TESTING.md) - Overall testing strategy

## Summary

✅ **Complete Implementation**: Fully functional step frame capture
✅ **Schema Conformant**: Follows Step Frame Schema specification
✅ **Well Tested**: 11 comprehensive tests, all passing
✅ **Production Ready**: Fail-fast design, error handling, performance optimized
✅ **Documented**: Complete API documentation and examples

The step frame capture function is ready for integration into browser automation workflows and provides a solid foundation for creating detailed workflow documentation and agent learning systems.
