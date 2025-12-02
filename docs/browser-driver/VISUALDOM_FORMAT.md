# VisualDom Format Specification

## Overview

**VisualDom is a custom format we created** to give AI agents a precise understanding of webpage structure, layout, and content without expensive OCR. It combines:

1. **Chrome DevTools Protocol's DOMSnapshot.captureSnapshot** - Provides complete DOM tree with layout data
2. **JavaScript-based image extraction** - Converts all images to base64 data URIs
3. **Custom JSON packaging** - Combines everything into a single analyzable file

## Why We Created VisualDom

Traditional approaches for AI agents to understand webpages have limitations:

- **Screenshots alone**: Require expensive OCR, lose structure, imprecise positioning
- **HTML DOM alone**: No layout information, no visual positioning, no images
- **Accessibility tree**: Limited information, missing visual hierarchy

**VisualDom solves these problems** by providing:
- All visible text without OCR
- Precise bounding boxes for every element
- Computed styles for understanding design
- Embedded images that Claude can see
- Visual hierarchy (paint order, stacking)

## File Format

### File Naming Convention
```
frame_0000.visualdom.json
frame_0001.visualdom.json
frame_0002.visualdom.json
```

### JSON Structure

```json
{
  "documents": [
    {
      "documentURL": 0,
      "title": 1,
      "baseURL": 2,
      "nodes": { /* DOM tree */ },
      "layout": { /* Layout tree */ },
      "textBoxes": { /* Text positioning */ },
      "scrollOffsetX": 0,
      "scrollOffsetY": 0,
      "contentWidth": 1920,
      "contentHeight": 3000
    }
  ],
  "strings": [
    "https://example.com",
    "Example Page Title",
    "https://example.com",
    "display",
    "block",
    "...all other strings..."
  ],
  "images": [
    {
      "src": "https://example.com/logo.png",
      "data": "data:image/png;base64,iVBORw0KG...",
      "width": 100,
      "height": 50,
      "x": 20,
      "y": 10,
      "displayWidth": 100,
      "displayHeight": 50,
      "alt": "Company Logo"
    }
  ]
}
```

## Key Components

### 1. String Table (`strings`)

All strings in the document are stored once in this array and referenced by index. This dramatically reduces file size for repeated strings (like "div", "span", CSS properties, etc.).

**Example:**
```json
"strings": ["div", "span", "display", "block", "inline"]
```

Then `nodeName: 0` means the node name is "div".

### 2. DOM Node Tree (`nodes`)

Parallel arrays describing the DOM structure:

```json
"nodes": {
  "parentIndex": [null, 0, 0, 1, 1, 2],
  "nodeType": [9, 1, 1, 1, 3, 1],
  "nodeName": [0, 1, 2, 3, 4, 5],
  "nodeValue": [-1, -1, -1, -1, 6, -1],
  "backendNodeId": [1, 2, 3, 4, 5, 6],
  "attributes": [[], [], [7, 8], [], [], []],
  "textValue": {
    "index": [4],
    "value": [6]
  },
  "currentSourceURL": {
    "index": [5],
    "value": [9]
  },
  "isClickable": {
    "index": [3, 5]
  }
}
```

**Field Descriptions:**
- `parentIndex`: Index of parent node (null for root)
- `nodeType`: DOM node type (1=Element, 3=Text, 9=Document)
- `nodeName`: String table index for node name
- `nodeValue`: String table index for node value
- `attributes`: Array of string table indexes (pairs of name, value)
- `textValue`: Sparse array for text nodes
- `currentSourceURL`: Sparse array for image/media sources
- `isClickable`: Sparse array of clickable element indexes

### 3. Layout Tree (`layout`)

Visual layout information for rendered elements:

```json
"layout": {
  "nodeIndex": [0, 1, 2, 3],
  "bounds": [
    [0, 0, 1920, 1080],
    [0, 0, 1920, 100],
    [20, 20, 200, 60],
    [240, 20, 800, 600]
  ],
  "text": [-1, -1, 10, -1],
  "styles": [
    [3, 4, 11, 12],
    [3, 5, 13, 14],
    [3, 4],
    []
  ],
  "paintOrders": [0, 1, 2, 3],
  "offsetRects": [...],
  "scrollRects": [...],
  "clientRects": [...]
}
```

**Field Descriptions:**
- `nodeIndex`: Index into nodes array
- `bounds`: `[x, y, width, height]` bounding box
- `text`: String table index for text content
- `styles`: Array of string table indexes for computed styles (pairs of property, value)
- `paintOrders`: Rendering order (higher = on top)
- `offsetRects/scrollRects/clientRects`: Additional box model data

### 4. Text Boxes (`textBoxes`)

Precise text positioning:

```json
"textBoxes": {
  "layoutIndex": [2, 2, 3],
  "start": [0, 11, 0],
  "length": [10, 5, 20],
  "bounds": [
    [20, 20, 100, 20],
    [120, 20, 50, 20],
    [240, 20, 200, 20]
  ]
}
```

Maps text ranges to their visual positions.

### 5. Images (`images`)

**This is our custom addition**, not part of CDP's native response:

```json
"images": [
  {
    "src": "https://example.com/image.jpg",
    "data": "data:image/png;base64,iVBORw0KGgoAAAANS...",
    "width": 400,
    "height": 300,
    "x": 50,
    "y": 100,
    "displayWidth": 400,
    "displayHeight": 300,
    "alt": "Description of image"
  },
  {
    "src": "https://example.com/blocked.jpg",
    "data": null,
    "width": 200,
    "height": 150,
    "x": 500,
    "y": 100,
    "displayWidth": 200,
    "displayHeight": 150,
    "alt": "",
    "error": "CORS or load error"
  }
]
```

**How Images Are Captured:**

1. JavaScript queries all `<img>` elements
2. For each visible image:
   - Create a canvas with image dimensions
   - Draw the image onto the canvas
   - Convert to data URI using `canvas.toDataURL('image/png')`
3. If CORS or other errors occur, record metadata without data
4. Include position (x, y) and dimensions

## Computed Styles

The `styles` array in the layout tree contains computed CSS properties. We offer three presets:

### Balanced (Default)
```
display, position, top, left, right, bottom, width, height, z-index,
visibility, opacity, font-size, font-weight, font-family, color,
padding, margin, background-color, background-image
```

### Minimal
```
display, position, visibility
```

### All
Empty array = capture all computed styles (large file size)

## Usage by AI Agents

### Finding Text
```javascript
// Get all text content
const textContent = visualDom.layout.text
  .map(idx => idx >= 0 ? visualDom.strings[idx] : '')
  .join(' ');
```

### Finding Clickable Elements
```javascript
// Get clickable elements with positions
const clickable = visualDom.nodes.isClickable.index.map(nodeIdx => {
  const layoutIdx = visualDom.layout.nodeIndex.indexOf(nodeIdx);
  const bounds = visualDom.layout.bounds[layoutIdx];
  const text = visualDom.layout.text[layoutIdx];
  return {
    nodeIndex: nodeIdx,
    bounds: bounds,
    text: text >= 0 ? visualDom.strings[text] : '',
  };
});
```

### Finding Images
```javascript
// Get all images with their data
const images = visualDom.images.map(img => ({
  url: img.src,
  base64: img.data,
  position: { x: img.x, y: img.y },
  size: { width: img.width, height: img.height },
  alt: img.alt
}));
```

### Understanding Layout
```javascript
// Find elements by position
function findElementAt(x, y) {
  return visualDom.layout.bounds.findIndex(([bx, by, bw, bh]) =>
    x >= bx && x <= bx + bw && y >= by && y <= by + bh
  );
}
```

## File Size Considerations

Typical VisualDom file sizes:
- **Simple page** (few elements, no images): 10-50 KB
- **Typical page** (moderate complexity, few images): 50-200 KB
- **Complex page** (many elements, several images): 200-500 KB
- **Image-heavy page**: 500 KB - 2 MB

**File size is dominated by:**
1. Base64-encoded images (largest factor)
2. Number of DOM nodes
3. Computed styles (more properties = larger)

**To reduce file size:**
- Use minimal computed styles preset
- Disable images if not needed
- Filter to visible elements only (future enhancement)

## Implementation Details

### CDP Command
```javascript
DOMSnapshot.captureSnapshot({
  computedStyles: ['display', 'position', ...],
  includeDOMRects: true,
  includePaintOrder: true
})
```

### Image Extraction
```javascript
const images = Array.from(document.querySelectorAll('img'));
const results = [];

for (const img of images) {
  const rect = img.getBoundingClientRect();
  if (rect.width === 0 || rect.height === 0) continue;

  const canvas = document.createElement('canvas');
  canvas.width = img.naturalWidth || img.width;
  canvas.height = img.naturalHeight || img.height;

  const ctx = canvas.getContext('2d');
  ctx.drawImage(img, 0, 0);

  const dataUrl = canvas.toDataURL('image/png');
  results.push({
    src: img.src,
    data: dataUrl,
    ...
  });
}
```

## Comparison with Other Formats

| Feature | Screenshot | HTML DOM | Accessibility Tree | VisualDom |
|---------|-----------|----------|-------------------|-----------|
| Visual layout | ✓ | ✗ | ✗ | ✓ |
| Text extraction | OCR required | ✓ | ✓ | ✓ |
| Bounding boxes | ✗ | ✗ | Partial | ✓ |
| Images | ✓ (pixels) | URLs only | ✗ | ✓ (base64) |
| Computed styles | ✗ | ✗ | ✗ | ✓ |
| Paint order | ✗ | ✗ | ✗ | ✓ |
| File size | Large (PNG) | Small | Small | Medium |
| Agent-friendly | ✗ | Partial | ✓ | ✓✓✓ |

## Future Enhancements

Potential improvements to consider:
- **Background images**: Currently only captures `<img>` tags
- **CSS animations/transitions**: Capture animated state
- **Viewport filtering**: Only include visible elements
- **Compression**: Gzip or custom compression
- **Differential updates**: Only changes from previous frame
- **SVG extraction**: Inline SVG as structured data
- **Video frames**: Capture video thumbnails

## License & Attribution

VisualDom format created by the Robert project team.

Based on Chrome DevTools Protocol's DOMSnapshot domain:
https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/
