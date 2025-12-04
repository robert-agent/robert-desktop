# AI Agent Workflow Formats - Summary

## What We Created

A complete standard format system for AI agents to learn and improve website navigation workflows.

## Files Created

### 📚 Documentation

1. **[README.md](./README.md)**
   - Main introduction and overview
   - Quick start guide for agents
   - Benefits, features, and FAQ
   - Best starting point for understanding the system

2. **[workflow_graph_schema.md](./workflow_graph_schema.md)**
   - Complete specification of the workflow graph format (`.workflow.md`)
   - Nodes, edges, metadata, and visualization
   - Usage guidelines for agents
   - Detailed examples with GitHub workflow

3. **[step_frame_schema.md](./step_frame_schema.md)**
   - Complete specification of the step frame format (`.frames.json`)
   - Screenshot, DOM, action, and transcript structure
   - Privacy, security, and storage guidelines
   - Learning and analysis queries

4. **[agent_workflow_standards.md](./agent_workflow_standards.md)**
   - Best practices and integration guide
   - Agent responsibilities (recording, learning, updating)
   - Workflow lifecycle (discovery → maintenance)
   - Merging knowledge from multiple agents
   - Metrics and monitoring

### 📂 Examples

5. **[examples/workflows/github.com/create_repository/github.com_create_repository_v1.workflow.md](./examples/workflows/github.com/create_repository/github.com_create_repository_v1.workflow.md)**
   - Real workflow graph example
   - Shows GitHub repository creation workflow
   - Demonstrates nodes, edges, error recovery

6. **[examples/workflows/github.com/create_repository/session_demo/minimal_example.frames.json](./examples/workflows/github.com/create_repository/session_demo/minimal_example.frames.json)**
   - Real step frame example
   - 4-frame session showing repository creation
   - Demonstrates screenshot, DOM, actions, transcripts

## The Two Formats

### Format 1: Workflow Graph (`.workflow.md`)

**Purpose**: High-level navigation roadmap

```markdown
---
domain: github.com
workflow_name: create_new_repository
version: 1.0.0
success_rate: 0.98
tested_sessions: 45
---

## Nodes
- github_home (page)
- new_repo_button (action)
- create_repo_form (page)

## Edges
github_home → new_repo_button
  Action: click
  Selector: [data-test-selector="global-create-menu-button"]
  Confidence: 0.98
  Success indicators: Dropdown menu visible
```

**Use Case**: Agent planning and execution

### Format 2: Step Frame (`.frames.json`)

**Purpose**: Detailed execution evidence

```json
{
  "metadata": {
    "domain": "github.com",
    "workflow_name": "create_repository",
    "success": true,
    "total_duration_ms": 8450
  },
  "frames": [
    {
      "frame_id": 0,
      "screenshot": {"path": "./screenshots/frame_0000.png"},
      "dom": {"url": "https://github.com", "html_path": "./dom/frame_0000.html"},
      "action": {
        "type": "click",
        "target": {"selector": "[data-test-selector='global-create-menu-button']"},
        "intent": "Open the create menu"
      },
      "transcript": {
        "action_description": "Clicking the '+' button to open create menu",
        "reasoning": "Standard entry point for creating items",
        "expected_outcome": "Dropdown menu should appear"
      },
      "learning": {
        "selector_stability": 0.98,
        "action_reliability": 0.96
      }
    }
  ]
}
```

**Use Case**: Agent learning and improvement

## How It Works

### 1. Recording Phase
```
Agent executes workflow
  ↓
Captures frames (screenshot + DOM + action)
  ↓
Saves session as .frames.json
```

### 2. Learning Phase
```
Agent analyzes multiple sessions
  ↓
Calculates selector stability & confidence
  ↓
Extracts nodes, edges, alternative paths
  ↓
Creates/updates .workflow.md
```

### 3. Execution Phase
```
Agent loads .workflow.md
  ↓
Follows highest-confidence path
  ↓
Records new session
  ↓
Updates confidence scores
  ↓
Cycle repeats (continuous improvement)
```

## Key Innovations

### 🎯 Confidence-Based Navigation
Every selector has empirical success rate:
- `0.95+`: Highly reliable, use as primary
- `0.85-0.94`: Good fallback
- `0.70-0.84`: Use with caution
- `<0.70`: Consider alternatives

### 🔄 Self-Improving System
```
Session 1:   60% success → discovering
Session 10:  85% success → learning
Session 50:  94% success → optimized
Session 100: 97% success → mature
```

### 🛡️ Error Recovery Built-In
```markdown
### Error: rate_limit_exceeded
**Detection**: 429 status
**Recovery**:
1. Wait 60 seconds
2. Retry
**Fallback**: Notify user
```

### 🤝 Multi-Agent Knowledge Sharing
Agents merge their knowledge:
- Combine alternative selectors
- Average confidence scores (weighted)
- Share error recovery strategies
- Increment version appropriately

## Directory Structure

```
workflows/
├── {domain}/
│   ├── {workflow_name}/
│   │   ├── {domain}_{workflow}_v{version}.workflow.md    # Graph
│   │   ├── session_{id}/
│   │   │   ├── {domain}_{workflow}_session_{id}.frames.json  # Frames
│   │   │   ├── screenshots/
│   │   │   │   └── frame_*.png
│   │   │   └── dom/
│   │   │       └── frame_*.html
│   │   └── session_{id2}/
│   │       └── ...
│   └── {workflow_name2}/
│       └── ...
└── {domain2}/
    └── ...
```

## Usage Examples

### For Agents: Execute Workflow

```python
# 1. Load workflow graph
workflow = load_workflow("github.com_create_repository_v1.workflow.md")

# 2. Check prerequisites
if not meets_prerequisites(workflow.prerequisites):
    return Error("Prerequisites not met")

# 3. Execute highest-confidence path
for edge in workflow.optimal_path:
    selector = edge.best_selector  # Highest confidence
    execute_action(edge.action, selector)
    wait_for(edge.wait_conditions)
    verify(edge.success_indicators)
```

### For Agents: Learn New Workflow

```python
# 1. Record while executing
session = Session("github.com", "create_repository")

for action in execute_workflow():
    session.capture_frame_before()
    result = execute(action)
    session.capture_frame_after()

session.save("session_abc123.frames.json")

# 2. Create workflow graph from session
workflow = WorkflowGraph.from_session(session)
workflow.save("github.com_create_repository_v1.workflow.md")
```

### For Agents: Improve Workflow

```python
# 1. Load existing workflow
workflow = load_workflow("github.com_create_repository_v1.workflow.md")

# 2. Execute and record
session = execute_with_recording(workflow)

# 3. Update confidence scores
for frame in session.frames:
    selector = frame.action.target.selector
    success = frame.verification.action_succeeded
    workflow.update_confidence(selector, success)

# 4. Save new version
workflow.increment_version()  # v1.0.0 → v1.0.1
workflow.tested_sessions += 1
workflow.save()
```

## Benefits

### For AI Agents
✅ No need to explore websites from scratch
✅ Follow proven paths with empirical success rates
✅ Automatic error recovery using documented strategies
✅ Learn from every execution
✅ Share knowledge with other agents

### For Developers
✅ Transparent agent behavior
✅ Debuggable with session frames
✅ Version-controlled workflow evolution
✅ Multi-agent collaboration
✅ Detect when websites change

## Comparison to Existing Approaches

| Approach | Exploration | Learning | Error Recovery | Knowledge Sharing |
|----------|-------------|----------|----------------|-------------------|
| **Traditional automation** (Selenium) | ❌ Manual | ❌ No | ❌ Manual | ❌ No |
| **Research datasets** (Mind2Web) | ✅ Yes | ⚠️ Limited | ❌ No | ✅ Yes |
| **LLM-based agents** (Current) | ⚠️ Each time | ❌ No | ⚠️ Ad-hoc | ❌ No |
| **This system** | ✅ Once | ✅ Continuous | ✅ Documented | ✅ Multi-agent |

## Integration Examples

### With Mind2Web Dataset
```python
# Convert Mind2Web to our format
for task in mind2web.tasks:
    workflow = WorkflowGraph(
        domain=task.domain,
        workflow_name=task.intent,
        nodes=extract_nodes(task.actions),
        edges=extract_edges(task.actions)
    )
    workflow.save()
```

### With LangChain
```python
from langchain.tools import WorkflowTool

tool = WorkflowTool.from_workflow_graph(
    "github.com_create_repository_v1.workflow.md"
)
agent.add_tool(tool)
```

### With Custom Agent
```python
class MyAgent:
    def learn_workflow(self, workflow_path):
        self.workflow = load_workflow(workflow_path)

    def execute_task(self, task):
        for edge in self.workflow.optimal_path:
            self.execute_action(edge)
            if failed:
                self.recover_from_error(edge.error_recovery)
```

## Metrics to Track

```yaml
# Workflow health
workflows/github.com/create_repository:
  success_rate: 0.96
  avg_duration_seconds: 12.3
  p95_duration_seconds: 18.7
  total_sessions: 342
  failed_sessions: 14
  common_failures:
    - rate_limit: 8 occurrences
    - selector_not_found: 6 occurrences

# Agent learning progress
agents/agent-001:
  workflows_learned: 23
  total_sessions: 1547
  success_rate: 0.94
  contributions:
    workflow_updates: 45
    new_paths_discovered: 12
    error_recoveries_added: 8
```

## Next Steps

### For Implementation

1. **Start small**: Pick one workflow to learn
2. **Record sessions**: Execute 10-20 times while recording frames
3. **Analyze**: Extract workflow graph from sessions
4. **Iterate**: Let the system improve over time
5. **Share**: Multiple agents contribute to same workflows

### For Research

1. **Visual element detection**: ML-based element recognition when selectors fail
2. **Cross-workflow transfer**: Learn patterns applicable across domains
3. **Auto-discovery**: Agents autonomously find common workflows
4. **Real-time collaboration**: Multiple agents update workflows concurrently
5. **Continuous validation**: Automated testing to detect site changes

## Files to Read

**Start here:**
1. [README.md](./README.md) - Overview and introduction
2. [examples/workflows/github.com/create_repository/github.com_create_repository_v1.workflow.md](./examples/workflows/github.com/create_repository/github.com_create_repository_v1.workflow.md) - Example workflow graph
3. [examples/workflows/github.com/create_repository/session_demo/minimal_example.frames.json](./examples/workflows/github.com/create_repository/session_demo/minimal_example.frames.json) - Example session frames

**Deep dive:**
4. [specs/workflow_graph_schema.md](./specs/workflow_graph_schema.md) - Complete graph format spec
5. [specs/step_frame_schema.md](./specs/step_frame_schema.md) - Complete frame format spec
6. [specs/agent_workflow_standards.md](./specs/agent_workflow_standards.md) - Best practices and integration

## Key Takeaways

1. **Two complementary formats**: Graph for navigation, frames for learning
2. **Empirical confidence scores**: Based on actual execution data
3. **Self-improving system**: Gets better with each execution
4. **Multi-agent knowledge**: Agents build on each other's work
5. **Error recovery built-in**: Documented strategies for common failures
6. **No human intervention needed**: Fully autonomous learning loop

---

**The future of AI web navigation: Learn once, execute reliably, improve continuously.** 🚀
