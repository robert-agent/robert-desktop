# Workflow Graph Format

## Overview
A standardized markdown format for representing common user workflows on websites. This format allows AI agents to understand, learn, and improve navigation paths without broad exploration.

## File Naming Convention
```
{domain}_{workflow_name}_v{version}.workflow.md
```

Examples:
- `github.com_create_repository_v1.workflow.md`
- `gmail.com_compose_send_email_v2.workflow.md`
- `amazon.com_purchase_product_v1.workflow.md`

## Schema

### 1. Metadata Section
```markdown
---
domain: example.com
workflow_name: workflow_description
version: 1.0.0
created: 2025-10-09
updated: 2025-10-09
success_rate: 0.95
avg_duration_seconds: 45
tested_sessions: 150
prerequisites:
  - User must be logged in
  - User must have verified email
tags:
  - authentication
  - core_workflow
---
```

### 2. Workflow Description
```markdown
## Workflow: [Human-readable name]

**Goal**: [Clear statement of what this workflow accomplishes]

**Starting Point**: [URL or state where workflow begins]

**End State**: [Expected outcome or final URL]

**Common Variations**:
- [List alternate paths or conditions]
```

### 3. Graph Representation

#### Node Format
```markdown
### Node: [node_id]

**Type**: [page | modal | dropdown | action | decision]
**URL Pattern**: [regex or exact URL, if applicable]
**Selector**: [CSS selector for the element, if applicable]
**State Conditions**: [Any required state, e.g., "cart must have items"]

**Description**: [What this node represents]
```

#### Edge Format
```markdown
### Edge: [source_node_id] → [target_node_id]

**Action**: [click | type | select | hover | scroll | navigate | wait]
**Trigger Element**:
  - Selector: `[CSS selector]`
  - Text: "[visible text, if applicable]"
  - Attributes: `[key attributes for identification]`

**Input Data**: [For type/select actions, describe expected input]
  - Field: `[field name]`
  - Type: [text | email | number | date]
  - Validation: [any validation rules]
  - Example: "[example value]"

**Wait Conditions**: [What to wait for after action]
  - Type: [networkidle | selector | timeout]
  - Value: `[selector or duration]`

**Success Indicators**: [How to verify the action succeeded]

**Failure Modes**: [Common failure scenarios and recovery]
```

### 4. Visual Graph (Mermaid)
```markdown
## Workflow Graph

\`\`\`mermaid
graph TD
    START[Start: Homepage] --> A{Logged in?}
    A -->|No| B[Login Page]
    A -->|Yes| C[Dashboard]
    B --> D[Enter Credentials]
    D --> E[Click Login]
    E --> C
    C --> F[Click New Item]
    F --> G[Fill Form]
    G --> H[Submit]
    H --> END[Success: Item Created]
\`\`\`
```

### 5. Alternative Paths
```markdown
## Alternative Paths

### Path: [path_name]
**Condition**: [When this path is taken]
**Nodes**: [List of node IDs in sequence]
**Probability**: [0.0-1.0, based on observed usage]
```

### 6. Error Recovery
```markdown
## Error Recovery

### Error: [error_type]
**Detection**: [How to detect this error]
**Recovery Steps**:
1. [Step-by-step recovery actions]
2. [...]

**Fallback**: [Ultimate fallback if recovery fails]
```

### 7. Optimization Notes
```markdown
## Optimization Notes

**Bottlenecks**:
- [Node/edge that commonly causes delays]

**Improvements**:
- [Suggested optimizations based on agent learning]

**Confidence Scores**:
- Node [node_id]: 0.95 (based on 150 successful traversals)
- Edge [edge_id]: 0.87 (12% failure rate, usually due to timeout)
```

---

## Complete Example

```markdown
---
domain: github.com
workflow_name: create_new_repository
version: 2.1.0
created: 2025-09-15
updated: 2025-10-09
success_rate: 0.96
avg_duration_seconds: 38
tested_sessions: 342
prerequisites:
  - User must be logged in
  - User must have verified email
tags:
  - repository_management
  - core_workflow
  - create_action
---

## Workflow: Create a New GitHub Repository

**Goal**: Create a new public or private repository on GitHub

**Starting Point**: https://github.com (user logged in)

**End State**: New repository created at https://github.com/{username}/{repo-name}

**Common Variations**:
- Creating with/without README
- Public vs Private repository
- Adding .gitignore and license

## Nodes

### Node: github_home
**Type**: page
**URL Pattern**: `^https://github\.com/?$`
**Description**: GitHub homepage when logged in

### Node: new_repo_button
**Type**: action
**Selector**: `[data-test-selector="global-create-menu-button"]`
**Description**: The "+" dropdown button in top navigation

### Node: new_repo_menu_item
**Type**: action
**Selector**: `[data-test-selector="global-create-menu"] a[href="/new"]`
**Description**: "New repository" option in dropdown menu

### Node: create_repo_form
**Type**: page
**URL Pattern**: `^https://github\.com/new$`
**Description**: Repository creation form page

### Node: repo_name_input
**Type**: action
**Selector**: `input[data-testid="repository-name-input"]`
**Description**: Repository name input field

### Node: visibility_public
**Type**: action
**Selector**: `input[type="radio"][value="public"]`
**Description**: Public repository radio button

### Node: visibility_private
**Type**: action
**Selector**: `input[type="radio"][value="private"]`
**Description**: Private repository radio button

### Node: create_repo_submit
**Type**: action
**Selector**: `button[data-test-selector="create-repository-button"]`
**Description**: Final "Create repository" button

### Node: repo_created
**Type**: page
**URL Pattern**: `^https://github\.com/[^/]+/[^/]+$`
**Description**: Newly created repository page

## Edges

### Edge: github_home → new_repo_button
**Action**: click
**Trigger Element**:
  - Selector: `[data-test-selector="global-create-menu-button"]`
  - Text: "+"
  - Attributes: `aria-label="Create new..."`

**Wait Conditions**:
  - Type: selector
  - Value: `[data-test-selector="global-create-menu"]`

**Success Indicators**: Dropdown menu becomes visible

**Failure Modes**:
  - Button not found → User may not be logged in
  - Menu doesn't open → Try clicking again after 500ms

### Edge: new_repo_button → new_repo_menu_item
**Action**: click
**Trigger Element**:
  - Selector: `[data-test-selector="global-create-menu"] a[href="/new"]`
  - Text: "New repository"

**Wait Conditions**:
  - Type: networkidle
  - Value: 2000ms

**Success Indicators**: URL changes to /new

### Edge: new_repo_menu_item → create_repo_form
**Action**: navigate
**Success Indicators**: Form page loads with repository name input visible

### Edge: create_repo_form → repo_name_input
**Action**: type
**Trigger Element**:
  - Selector: `input[data-testid="repository-name-input"]`

**Input Data**:
  - Field: `repository-name`
  - Type: text
  - Validation: Must be alphanumeric with hyphens/underscores, not start with hyphen
  - Example: "my-awesome-project"

**Success Indicators**:
  - Input value updates
  - Validation message shows green checkmark or availability status

**Failure Modes**:
  - Name already exists → Show error, agent must choose different name
  - Invalid characters → Remove invalid characters and retry

### Edge: repo_name_input → visibility_public
**Action**: click
**Trigger Element**:
  - Selector: `input[type="radio"][value="public"]`

**Success Indicators**: Radio button becomes checked

### Edge: visibility_public → create_repo_submit
**Action**: click
**Trigger Element**:
  - Selector: `button[data-test-selector="create-repository-button"]`
  - Text: "Create repository"

**Wait Conditions**:
  - Type: networkidle
  - Value: 3000ms

**Success Indicators**:
  - URL changes to new repository pattern
  - Repository page displays with quick setup section

**Failure Modes**:
  - Rate limit → Wait and retry after delay
  - Validation error → Check error message and fix input

### Edge: create_repo_submit → repo_created
**Action**: navigate
**Success Indicators**:
  - Repository page loads
  - Repository name appears in breadcrumb
  - Quick setup section is visible

## Workflow Graph

\`\`\`mermaid
graph TD
    START[github_home] --> A[new_repo_button]
    A --> B[new_repo_menu_item]
    B --> C[create_repo_form]
    C --> D[repo_name_input]
    D --> E{Visibility}
    E -->|Public| F[visibility_public]
    E -->|Private| G[visibility_private]
    F --> H[create_repo_submit]
    G --> H
    H --> END[repo_created]

    style START fill:#90EE90
    style END fill:#87CEEB
\`\`\`

## Alternative Paths

### Path: with_readme_initialization
**Condition**: When user wants to initialize with README
**Nodes**: [create_repo_form, repo_name_input, add_readme_checkbox, visibility_public, create_repo_submit, repo_created]
**Probability**: 0.73

### Path: with_gitignore_license
**Condition**: When user wants to add .gitignore and license
**Nodes**: [create_repo_form, repo_name_input, gitignore_dropdown, license_dropdown, visibility_public, create_repo_submit, repo_created]
**Probability**: 0.45

## Error Recovery

### Error: repository_name_taken
**Detection**: Error message appears below name input
**Recovery Steps**:
1. Clear current name input
2. Append timestamp or random suffix to attempted name
3. Retry submission

**Fallback**: Prompt user for alternative name

### Error: rate_limit_exceeded
**Detection**: 429 status or "You have exceeded a secondary rate limit" message
**Recovery Steps**:
1. Wait 60 seconds
2. Retry the create action
3. If fails again, wait 300 seconds

**Fallback**: Notify user to try again later

## Optimization Notes

**Bottlenecks**:
- Edge: create_repo_submit → repo_created (avg 3.2s wait time)

**Improvements**:
- Version 2.1.0: Updated selectors to use data-testid attributes (more stable)
- Version 2.0.0: Added rate limit detection and recovery

**Confidence Scores**:
- Node repo_name_input: 0.98 (selector stable across 342 sessions)
- Edge create_repo_submit → repo_created: 0.94 (occasional timeout issues)
- Node new_repo_button: 0.92 (selector changed once in last 6 months)
```

---

## Usage Guidelines for Agents

### Creating a New Workflow
1. Start with metadata section
2. Define all nodes encountered during workflow
3. Define edges with detailed action information
4. Create mermaid diagram for visualization
5. Document alternative paths observed
6. Add error recovery procedures
7. Set initial confidence scores to 0.5

### Updating an Existing Workflow
1. Increment version number (patch for fixes, minor for new paths, major for breaking changes)
2. Update `updated` timestamp
3. Recalculate `success_rate` and `avg_duration_seconds`
4. Increment `tested_sessions`
5. Update confidence scores based on new data
6. Document what changed in optimization notes

### Merging Workflows
When multiple agents have learned the same workflow:
1. Compare success rates and choose higher performing version as base
2. Merge alternative paths from both versions
3. Combine error recovery strategies
4. Average confidence scores weighted by tested_sessions
5. Increment version to next major version

### Confidence Score Calculation
```
confidence = (successful_traversals) / (total_attempts)
```

For nodes/edges: Track separately and update incrementally.
