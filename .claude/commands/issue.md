# Create GitHub Issue

Create a GitHub issue for the current problem or feature request being discussed.

## Instructions

You are helping the user document and track work by creating GitHub issues. Follow these steps:

### 1. Understand the Context
- Review the recent conversation history to understand what problem or feature is being discussed
- Identify the core issue, bug, feature request, or improvement
- Note any relevant details: file paths, error messages, proposed solutions, related PRs/issues

### 2. Determine Issue Type
Based on the context, classify the issue:
- `[bug]` - Something is broken or not working as expected
- `[feat]` - New feature request or enhancement
- `[chore]` - Maintenance, refactoring, or infrastructure work
- `[docs]` - Documentation improvements
- `[question]` - Discussion or clarification needed

### 3. Craft a Clear Title
Format: `[type] short, descriptive title in lowercase`

Examples:
- `[bug] agent configs stored globally instead of per-user`
- `[feat] add keyboard shortcuts for common actions`
- `[chore] update dependencies to latest versions`

### 4. Structure the Issue Body

Use this template structure (adapt as needed):

```markdown
## Problem

[Clear description of the issue, bug, or need for the feature]

**Current Behavior:**
- [What currently happens]
- [Specific problems or limitations]

**Expected Behavior:**
- [What should happen instead]

## Proposed Solution

[Detailed description of how to fix/implement this]

**Implementation Steps:**
1. [Step 1]
2. [Step 2]
3. [Step 3]

**Code Changes Required:**
- File: `path/to/file.rs` - [what needs to change]
- File: `path/to/other.rs` - [what needs to change]

## Context

[Additional background information]

**Related:**
- Issue #X
- PR #Y
- Documentation: `docs/FILE.md`

**Impact:**
- [Who is affected]
- [Why this matters]
- [Urgency/priority]

## Acceptance Criteria

- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Criterion 3
```

### 5. Create the Issue

Use the `gh issue create` command:

```bash
gh issue create \
  --title "[type] descriptive title" \
  --body "$(cat <<'EOF'
[Issue body content here]
EOF
)"
```

### 6. Report Back

After creating the issue, report to the user:
- Issue URL
- Issue number
- Brief summary of what was documented

## Guidelines

### What Makes a Good Issue

**Clear Title:**
- ✅ `[bug] login fails when password contains special characters`
- ❌ `Fix login` or `Password bug`

**Detailed Description:**
- Include specific error messages, file paths, line numbers
- Describe what was tried and what didn't work
- Reference related code or documentation
- Provide context about why this matters

**Actionable:**
- Clear steps to reproduce (for bugs)
- Specific acceptance criteria
- Proposed implementation approach
- Files that need modification

**Well-Structured:**
- Use headings for organization
- Use code blocks for code/commands/errors
- Use bullet points for lists
- Use checkboxes for action items

### When to Ask for Clarification

If the conversation is unclear about:
- The exact problem or feature request
- Priority or urgency
- Implementation approach
- Acceptance criteria

Ask the user questions using the AskUserQuestion tool before creating the issue.

### Issue Labels

If the repository supports labels, suggest appropriate ones:
- `bug`, `enhancement`, `documentation`
- `good first issue`, `help wanted`
- `priority: high`, `priority: medium`, `priority: low`
- Component labels: `frontend`, `backend`, `testing`, etc.

Note: The `gh issue create` command supports `--label` flag:
```bash
gh issue create --title "..." --body "..." --label "bug,priority: high"
```

## Examples

### Example 1: Bug Report

**Conversation Context:**
"The agent config system is saving files to ~/.config/robert/ instead of per-user directories."

**Issue Created:**
```markdown
Title: [bug] agent configs stored globally instead of per-user

Body:
## Problem

The agent configuration system currently stores configs in a global directory (`~/.config/robert/agents/`) instead of per-user directories. This breaks user isolation.

**Current Behavior:**
- All users share the same agent configs at `~/.config/robert/agents/`
- Changes by one user affect all others
- No encryption or isolation for agent customizations

**Expected Behavior:**
- Agent configs should be stored at `~/.robert/users/{username}/agents/`
- Each user has isolated agent configurations
- Aligns with profiles directory structure

[... rest of issue body ...]
```

### Example 2: Feature Request

**Conversation Context:**
"We should add keyboard shortcuts like Cmd+R to run scripts."

**Issue Created:**
```markdown
Title: [feat] add keyboard shortcuts for common actions

Body:
## Problem

Users currently must use mouse clicks for all actions. Keyboard shortcuts would improve efficiency for power users.

**Proposed Shortcuts:**
- `Cmd+R` - Run current script
- `Cmd+.` - Stop execution
- `Cmd+K` - Clear output
- `Cmd+,` - Open settings

[... rest of issue body ...]
```

## Error Handling

If issue creation fails:
1. Check GitHub CLI authentication: `gh auth status`
2. Verify you're in a git repository
3. Check network connectivity
4. Show the error to the user and suggest troubleshooting steps

## Output Format

After successful creation, respond with:

```
✅ Issue created: <URL>

**Summary:**
- Issue #<number>: [type] title
- [Brief description of what was documented]
- [Any next steps or related tasks]
```
