---
description: Automatically implement fixes for triaged GitHub issues
---

# Issue Fix Command

Automatically implements fixes for triaged GitHub issues that have proposed plans but no associated pull requests.

## Instructions

You are helping the user automatically implement fixes for triaged issues. Follow these steps carefully:

### 1. Check Prerequisites

First, verify the environment is ready:

```bash
# Check GitHub CLI authentication
gh auth status 2>&1

# Check we're on a clean working tree
git status

# Check current branch
git branch --show-current
```

**Important**: If not authenticated or working tree is dirty, stop and inform the user.

### 2. Fetch All Triaged Issues

Retrieve all open issues that have been triaged:

```bash
gh issue list --state open --json number,title,labels,body,url,comments,createdAt --limit 100
```

### 3. Filter Eligible Issues

From the fetched issues, identify those that:
- ✅ Have a `triaged` label
- ✅ Have a comment containing `[Proposed Plan]` or `## [Proposed Plan]`
- ❌ Do NOT have a `wip` label (work in progress)
- ❌ Do NOT have an associated pull request

**To check for associated PRs**, search for mentions of the issue number:
```bash
gh pr list --state all --search "fixes #<issue-number>" --json number,title,state --limit 10
```

If any PR mentions the issue (open, closed, or merged), skip this issue.

### 4. Select Issue to Fix

From the eligible issues:
- Sort by creation date (oldest first)
- Select the **oldest** eligible issue
- If no eligible issues found, report: "No eligible issues found to fix ✅"

### 5. Checkout Main and Create New Branch

Before creating the branch, ensure we're starting from a clean state:

```bash
# Get the base branch name (usually 'main' or 'master')
BASE_BRANCH=$(git remote show origin | grep "HEAD branch" | cut -d' ' -f5)

# Checkout base branch and pull latest
git checkout $BASE_BRANCH
git pull origin $BASE_BRANCH

# Create new branch with issue number and title
# Format: issue-<number>-<slugified-title>
# Example: issue-123-fix-authentication-bug
git checkout -b issue-<number>-<slugified-title>
```

**Branch naming rules:**
- Prefix with `issue-<number>-`
- Convert title to lowercase
- Replace spaces with hyphens
- Remove special characters (keep only a-z, 0-9, hyphens)
- Remove type prefix like `[bug]` or `[feat]` from title
- Limit to 50 characters total

Examples:
- Issue #123 "[bug] login fails with special chars" → `issue-123-login-fails-with-special-chars`
- Issue #45 "[feat] add dark mode support" → `issue-45-add-dark-mode-support`

### 6. Extract and Document the Plan

Extract the proposed plan from the issue comments:

1. Find the comment containing `[Proposed Plan]` or `## [Proposed Plan]`
2. Extract the full plan content
3. Create documentation file: `./docs/issue_<number>_fix.md`

**Documentation Format:**
```markdown
# Fix Plan for Issue #<number>

**Issue Title**: <title>
**Issue URL**: <url>
**Created**: <date>
**Branch**: issue-<number>-<slugified-title>

---

## Original Issue Description

<paste the original issue body here>

---

## Proposed Implementation Plan

<paste the full [Proposed Plan] content here>

---

## Implementation Notes

This document tracks the implementation of the fix for issue #<number>.

### Progress Checklist
- [ ] Branch created
- [ ] Plan documented
- [ ] Implementation started
- [ ] Tests passing locally
- [ ] Pull request created
- [ ] Issue tagged as WIP

### Implementation Log

**Date**: <current-date>
**Status**: Implementation in progress

<This section will be updated by the engineer agent as work progresses>
```

**Important**: Create the `./docs/` directory if it doesn't exist:
```bash
mkdir -p ./docs
```

### 7. Implement the Fix with Engineer Agent

Now use the Task tool to invoke the engineer agent to implement the fix:

```
Task(
  subagent_type="engineer",
  description="Implement fix for issue #<number>",
  prompt="You are implementing a fix for GitHub issue #<number>: <title>

ISSUE DETAILS:
<paste full issue description>

PROPOSED IMPLEMENTATION PLAN:
<paste the full [Proposed Plan] content>

YOUR TASK:
1. Read and understand the proposed plan thoroughly
2. Explore the codebase to locate the relevant files and understand the current implementation
3. Implement the fix following the proposed plan
4. Write or update tests to cover the changes
5. Run all relevant tests locally to ensure they pass:
   - Rust: cargo test, cargo clippy, cargo fmt --check
   - Bun: bun lint, bun check (if applicable)
6. Update the implementation log in ./docs/issue_<number>_fix.md with your progress and any notable decisions
7. Commit your changes with a descriptive message following the format:
   [<type>] <short description>

   <detailed description>

   Fixes #<issue-number>

CRITICAL REQUIREMENTS:
- Follow existing code patterns and architecture
- Ensure all tests pass before completing
- Document any deviations from the proposed plan
- Add appropriate error handling
- Update relevant documentation if needed
- DO NOT push to remote - I will handle that after review

When you're done, report:
- What was implemented
- Test results (all passing or any failures)
- Any challenges or deviations from the plan
- Whether the fix is ready for PR or needs more work"
)
```

**Wait for the engineer agent to complete before proceeding.**

### 8. Verify Tests Pass Locally

After the engineer agent completes, verify all tests pass:

```bash
# Run Rust tests
cargo test --workspace

# Run Rust linting
cargo clippy --workspace -- -D warnings

# Run Rust formatting check
cargo fmt --check

# Run Bun checks if applicable
bun lint 2>/dev/null || echo "No bun lint configured"
bun check 2>/dev/null || echo "No bun check configured"
```

**Decision Point:**
- ✅ **All tests passing**: Proceed to step 9 (create PR)
- ❌ **Tests failing**: Stop here, report failures, and DO NOT create PR

If tests are failing:
```
❌ Tests failed. Cannot create pull request.

**Failed Tests:**
<list failing tests>

**Action Required:**
- Review test failures
- Fix the issues manually or ask engineer agent to fix
- Run tests again
- Re-run /issuesfix when ready
```

### 9. Create Pull Request (Only if Tests Pass)

If all tests pass, create a pull request:

```bash
# Push branch to remote
git push -u origin HEAD

# Create PR with comprehensive description
gh pr create --title "<issue-title>" --body "$(cat <<'EOF'
## Summary
Fixes #<issue-number>

<Brief summary of what was fixed>

## Changes Made
<List of key changes from the implementation>

## Testing
- [x] All unit tests pass
- [x] All integration tests pass
- [x] Linting passes
- [x] Formatting check passes

## Implementation Details
See full implementation plan in `./docs/issue_<number>_fix.md`

## Related
- Issue: #<issue-number>
- Implementation plan: [./docs/issue_<number>_fix.md](./docs/issue_<number>_fix.md)

---
🤖 Automated fix generated by `/issuesfix` command
EOF
)"
```

**PR Title Format:**
- Use the same format as the issue title (e.g., `[bug] fix login with special chars`)
- Keep type prefix (`[bug]`, `[feat]`, etc.)
- Should match or be very similar to the issue title

### 10. Add WIP Label to Issue

Mark the issue as work-in-progress:

```bash
gh issue edit <issue-number> --add-label "wip"
```

Also add a comment linking to the PR:

```bash
gh issue comment <issue-number> --body "$(cat <<'EOF'
## 🚧 Implementation In Progress

A pull request has been created to implement this fix.

**PR**: #<pr-number>
**Branch**: issue-<number>-<slugified-title>
**Implementation Plan**: See `./docs/issue_<number>_fix.md`

The PR will be reviewed and merged once approved.

---
🤖 Automated by `/issuesfix` command
EOF
)"
```

### 11. Provide Summary Report

After completing the process, provide a comprehensive report:

```
# Issue Fix Report

## ✅ Successfully Implemented Fix

**Issue**: #<issue-number> - <title>
**Issue URL**: <url>

### What Was Done

1. ✅ Created branch: `issue-<number>-<slugified-title>`
2. ✅ Documented plan: `./docs/issue_<number>_fix.md`
3. ✅ Implemented fix using engineer agent
4. ✅ All tests passing locally
5. ✅ Pull request created: #<pr-number>
6. ✅ Issue tagged as `wip`

### Changes Summary

<Brief summary of what the engineer agent implemented>

### Test Results

**Rust Tests**: ✅ Passing
**Cargo Clippy**: ✅ Passing
**Cargo Fmt**: ✅ Passing
**Bun Checks**: ✅ Passing (if applicable)

### Next Steps

1. Review the pull request: <pr-url>
2. Review implementation details: `./docs/issue_<number>_fix.md`
3. Approve and merge when ready
4. Issue will be automatically closed when PR merges

### Engineer Agent Report

<Include the engineer agent's final report about what was implemented>
```

## Error Handling

### No Eligible Issues

```
✅ No eligible issues found to fix.

**Status Check:**
- Total open issues: X
- Triaged issues: Y
- Issues with proposed plans: Z
- Issues without WIP or PR: 0

All triaged issues are either already being worked on or have been completed!
```

### Tests Failing After Implementation

```
❌ Implementation completed but tests are failing.

**Issue**: #<number> - <title>
**Branch**: issue-<number>-<slugified-title>

**Failed Tests:**
<list of failures>

**Action Required:**
1. Review the implementation in the current branch
2. Fix the failing tests manually or ask engineer agent for help
3. Run tests again: cargo test && cargo clippy
4. When tests pass, manually create PR with: gh pr create
5. Add WIP label: gh issue edit <number> --add-label "wip"

**Note**: The branch and implementation are ready, but PR was not created due to test failures.
```

### Engineer Agent Could Not Complete

```
❌ Engineer agent was unable to complete the implementation.

**Issue**: #<number> - <title>
**Branch**: issue-<number>-<slugified-title>
**Reason**: <reason from engineer agent>

**Action Required:**
1. Review the engineer agent's report above
2. The branch has been created and plan documented at `./docs/issue_<number>_fix.md`
3. Either:
   - Fix the blockers and re-run /issuesfix
   - Manually implement the fix
   - Update the issue with additional information needed

**Cleanup**: To reset, run: git checkout main && git branch -D issue-<number>-<slugified-title>
```

### Dirty Working Tree

```
❌ Cannot proceed: working tree has uncommitted changes.

Please commit or stash your changes first:
- To commit: git add . && git commit -m "your message"
- To stash: git stash
- To discard: git restore .

Then re-run /issuesfix
```

### Not on Main Branch

```
⚠️  Warning: You are currently on branch '<current-branch>', not the main branch.

The command will checkout main and create a new branch from there.

Continue? [This will happen automatically]
```

## Guidelines

### What Makes a Good Automated Fix

**Complete:**
- All aspects of the proposed plan are implemented
- Tests are written or updated
- Documentation is updated if needed
- No TODOs or placeholders left

**Quality:**
- Follows existing code patterns
- Includes proper error handling
- Has adequate test coverage
- Passes all linting and formatting checks

**Traceable:**
- Clear commit messages referencing the issue
- Implementation log documents decisions
- PR description explains the changes
- Links back to original issue

### When to Skip an Issue

Skip issues if they:
- Already have a `wip` label (someone is working on it)
- Have an associated PR (open, closed, or merged)
- Don't have a `[Proposed Plan]` (not ready for implementation)
- Are too complex for automated implementation (note in report)

### Branch Management

**Important:**
- Always create branch from the latest main/master
- Use consistent naming: `issue-<number>-<slug>`
- Don't reuse branch names
- If branch already exists, append `-v2`, `-v3`, etc.

### Test Requirements

**Must Pass Before PR:**
- All unit tests
- All integration tests
- Cargo clippy with no warnings
- Cargo fmt check
- Any project-specific linters (bun lint, etc.)

**If Any Fail:**
- Do NOT create PR
- Report failures clearly
- Leave implementation in branch for manual fix
- Document what failed and why

## Security Considerations

**Automated Implementation Limits:**
- Do NOT auto-implement security-critical changes without review
- Do NOT auto-merge PRs (always require manual review)
- Do NOT bypass required CI/CD checks
- Do NOT commit sensitive data or credentials

**Safe Operations:**
- Creating branches: ✅ Safe
- Implementing fixes: ✅ Safe (with tests)
- Creating PRs: ✅ Safe (requires review)
- Adding labels: ✅ Safe

**Unsafe Operations (Never Do):**
- Auto-merging PRs: ❌ Unsafe
- Bypassing tests: ❌ Unsafe
- Disabling security checks: ❌ Unsafe
- Committing without verification: ❌ Unsafe

## Examples

### Example 1: Successful Fix Implementation

**Input**: `/issuesfix`

**Process:**
1. Found issue #123: "[bug] login fails with special characters"
2. Created branch: `issue-123-login-fails-with-special-characters`
3. Documented plan: `./docs/issue_123_fix.md`
4. Engineer agent implemented fix in `src/auth/login.rs`
5. All tests passed
6. Created PR #45
7. Added `wip` label to issue #123

**Output:**
```
# Issue Fix Report

## ✅ Successfully Implemented Fix

**Issue**: #123 - [bug] login fails with special characters
**Issue URL**: https://github.com/org/repo/issues/123

### What Was Done

1. ✅ Created branch: `issue-123-login-fails-with-special-characters`
2. ✅ Documented plan: `./docs/issue_123_fix.md`
3. ✅ Implemented fix using engineer agent
4. ✅ All tests passing locally
5. ✅ Pull request created: #45
6. ✅ Issue tagged as `wip`

### Changes Summary

- Updated password encoding in `src/auth/login.rs` to use URL encoding
- Added unit tests for special characters: !@#$%^&*()
- Updated password validator regex to allow all special chars
- All existing tests still passing

### Test Results

**Rust Tests**: ✅ 47 passed
**Cargo Clippy**: ✅ No warnings
**Cargo Fmt**: ✅ Formatted correctly

### Next Steps

1. Review the pull request: https://github.com/org/repo/pull/45
2. Review implementation details: `./docs/issue_123_fix.md`
3. Approve and merge when ready
```

### Example 2: Tests Failing

**Input**: `/issuesfix`

**Process:**
1. Found issue #124: "[feat] add dark mode"
2. Created branch: `issue-124-add-dark-mode`
3. Documented plan: `./docs/issue_124_fix.md`
4. Engineer agent implemented changes
5. ❌ Tests failed

**Output:**
```
# Issue Fix Report

## ❌ Implementation Completed But Tests Failing

**Issue**: #124 - [feat] add dark mode
**Branch**: issue-124-add-dark-mode

### What Was Done

1. ✅ Created branch: `issue-124-add-dark-mode`
2. ✅ Documented plan: `./docs/issue_124_fix.md`
3. ✅ Implemented changes using engineer agent
4. ❌ Tests failed - PR NOT created

### Test Results

**Rust Tests**: ❌ 2 failed, 45 passed
**Failed Tests:**
- `test_theme_switching` - assertion failed: expected "dark" but got "light"
- `test_css_variables` - CSS variable --bg-color not defined

**Cargo Clippy**: ✅ No warnings
**Cargo Fmt**: ✅ Formatted correctly

### Action Required

1. Review the implementation in branch `issue-124-add-dark-mode`
2. Fix the failing tests:
   - Check theme switching logic in `src/theme.rs`
   - Verify CSS variables are properly initialized
3. Run tests again: `cargo test`
4. When tests pass, manually create PR:
   ```
   gh pr create --title "[feat] add dark mode" --body "Fixes #124"
   ```
5. Add WIP label: `gh issue edit 124 --add-label "wip"`

The implementation is complete but needs test fixes before PR creation.
```

### Example 3: No Eligible Issues

**Input**: `/issuesfix`

**Output:**
```
# Issue Fix Report

## ✅ No Eligible Issues Found

**Status Check:**
- Total open issues: 12
- Triaged issues: 8
- Issues with proposed plans: 8
- Issues without WIP or PR: 0

**Current State:**
All triaged issues are either:
- Already being worked on (have `wip` label)
- Have associated pull requests
- Completed and closed

Great job! All triaged issues are being handled.

**Suggestions:**
- Run `/issuestriage` to triage new issues
- Review existing PRs for issues with `wip` label
- Check closed issues for any that need follow-up
```

## Important Notes

### Engineer Agent Integration

The engineer agent (`.claude/agents/engineer.md`) is responsible for:
- Understanding the codebase architecture
- Implementing the fix according to the plan
- Writing/updating tests
- Running tests locally
- Committing changes with proper messages

**Trust the Engineer Agent:**
- It has specialized skills for implementation
- It follows TDD and best practices
- It will report if it cannot complete the task
- It will document any deviations from the plan

### Documentation Tracking

The `./docs/issue_<number>_fix.md` file serves as:
- Implementation plan reference
- Progress tracker
- Decision log
- Historical record

This helps with:
- Code review context
- Future maintenance
- Understanding why changes were made
- Debugging if issues arise

### PR Review Process

**After PR Creation:**
1. Review the PR description and changes
2. Check the implementation plan document
3. Verify tests are comprehensive
4. Look for any deviations from the plan
5. Approve and merge when satisfied

**The PR is NOT auto-merged** - always requires human review.

### Workflow Integration

This command integrates with `/issuestriage`:
1. `/issuestriage` - Creates plans for untriaged issues
2. `/issuesfix` - Implements the oldest triaged issue with a plan
3. Manual review and merge of PR
4. Issue auto-closes when PR merges

**Recommended Flow:**
1. Run `/issuestriage` weekly to process new issues
2. Run `/issuesfix` to automatically implement fixes
3. Review and merge PRs daily
4. Repeat as needed

## Command Options (Future Enhancement)

While not implemented yet, consider these options for future versions:

```bash
/issuesfix --issue 123          # Fix specific issue number
/issuesfix --label "bug"        # Only fix issues with specific label
/issuesfix --limit 3            # Fix up to 3 issues in one run
/issuesfix --dry-run            # Show what would be done without doing it
/issuesfix --skip-tests         # Skip test verification (not recommended)
```

For now, the command processes one issue at a time to ensure quality.
