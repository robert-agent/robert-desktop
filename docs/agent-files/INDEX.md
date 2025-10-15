# Agent Workflow Formats - Index

Standardized formats for AI agents to learn and improve website navigation workflows.

## 📚 Documentation Structure

```
agent-formats/
├── README.md                    # Start here - Overview and introduction
├── QUICK_REFERENCE.md           # Quick reference summary
├── INDEX.md                     # This file - Navigation guide
├── specs/                       # Complete specifications
│   ├── WORKFLOW_GRAPH_SCHEMA.md
│   ├── STEP_FRAME_SCHEMA.md
│   └── AGENT_WORKFLOW_STANDARDS.md
└── examples/workflows/          # Real-world examples
    └── github.com/create_repository/
        ├── *.workflow.md
        └── session_demo/*.frames.json
```

## 🚀 Quick Navigation

### For First-Time Users

1. **[README.md](./README.md)** - Start here for overview
2. **[QUICK_REFERENCE.md](./QUICK_REFERENCE.md)** - Quick summary of both formats
3. **[examples/workflows/](./examples/workflows/)** - See real examples

### For Implementation

4. **[specs/WORKFLOW_GRAPH_SCHEMA.md](./specs/WORKFLOW_GRAPH_SCHEMA.md)** - Complete graph format specification
5. **[specs/STEP_FRAME_SCHEMA.md](./specs/STEP_FRAME_SCHEMA.md)** - Complete frame format specification
6. **[specs/AGENT_WORKFLOW_STANDARDS.md](./specs/AGENT_WORKFLOW_STANDARDS.md)** - Integration guide and best practices

## 📋 The Two Formats

### 1. Workflow Graph (`.workflow.md`)
**Purpose**: Navigation roadmap with proven paths

**Contains**:
- Nodes (pages, buttons, forms)
- Edges (actions with selectors)
- Confidence scores (empirical)
- Error recovery strategies
- Alternative paths

**Example**: [examples/workflows/github.com/create_repository/github.com_create_repository_v1.workflow.md](./examples/workflows/github.com/create_repository/github.com_create_repository_v1.workflow.md)

### 2. Step Frame (`.frames.json`)
**Purpose**: Detailed execution evidence

**Contains**:
- Screenshots at each step
- DOM snapshots
- Action metadata
- Natural language transcripts
- Verification results
- Learning data

**Example**: [examples/workflows/github.com/create_repository/session_demo/minimal_example.frames.json](./examples/workflows/github.com/create_repository/session_demo/minimal_example.frames.json)

## 🔄 How It Works

```
1. Recording     → Agent captures frames during execution
2. Learning      → Analyze sessions to build workflow graphs
3. Execution     → Follow highest-confidence paths
4. Improvement   → Update confidence scores after each run
```

## ✨ Key Features

- 🎯 **Confidence-based navigation** - Empirical success rates (0.0-1.0)
- 🔄 **Self-improving** - Success rate increases over time
- 🛡️ **Error recovery** - Documented strategies for failures
- 🤝 **Multi-agent sharing** - Agents build on each other's work
- 📊 **Transparent** - View frames and graphs to debug

## 📖 Documentation by Use Case

### I want to...

**Understand the system**
→ [README.md](./README.md)

**Get a quick overview**
→ [QUICK_REFERENCE.md](./QUICK_REFERENCE.md)

**See an example**
→ [examples/workflows/github.com/create_repository/](./examples/workflows/github.com/create_repository/)

**Implement workflow graphs**
→ [specs/WORKFLOW_GRAPH_SCHEMA.md](./specs/WORKFLOW_GRAPH_SCHEMA.md)

**Implement session recording**
→ [specs/STEP_FRAME_SCHEMA.md](./specs/STEP_FRAME_SCHEMA.md)

**Integrate into my agent**
→ [specs/AGENT_WORKFLOW_STANDARDS.md](./specs/AGENT_WORKFLOW_STANDARDS.md)

**Learn best practices**
→ [specs/AGENT_WORKFLOW_STANDARDS.md](./specs/AGENT_WORKFLOW_STANDARDS.md)

## 🔗 Related Documentation

- **[../docs/PRD.md](../docs/PRD.md)** - Product requirements (see "Workflow Learning System" section)
- **[../docs/IMPLEMENTATION_PLAN.md](../docs/IMPLEMENTATION_PLAN.md)** - Implementation plan (see "Phase 9")
- **[../WORKFLOW_FORMATS_ADDED.md](../WORKFLOW_FORMATS_ADDED.md)** - Summary of what was added

## 📦 File Format Summary

| Format | Extension | Purpose | Size | Human-Readable |
|--------|-----------|---------|------|----------------|
| Workflow Graph | `.workflow.md` | Navigation roadmap | ~10-50KB | ✅ Yes (Markdown) |
| Step Frame | `.frames.json` | Execution evidence | ~100KB-1MB+ | ⚠️ Partial (JSON) |
| Screenshots | `.png` | Visual state | ~200KB each | ✅ Yes (Image) |
| DOM Snapshots | `.html` | Page structure | ~50-500KB | ✅ Yes (HTML) |

## 🎯 Benefits

**For AI Agents:**
- No exploration needed
- Higher reliability
- Automatic error recovery
- Learn from experience
- Share knowledge

**For Users:**
- Faster setup
- More reliable workflows
- Transparent behavior
- Self-improving over time
- Community knowledge sharing

**For Robert:**
- Competitive differentiation
- Network effects
- Improved reliability
- Reduced costs
- Better user trust

---

**Start exploring: [README.md](./README.md)**
