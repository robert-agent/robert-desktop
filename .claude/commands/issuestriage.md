---
description: Triage untriaged GitHub issues and propose implementation plans
---

# Issue Triage Command

Automatically triage GitHub issues that haven't been reviewed yet, analyze their content, devise implementation plans, and post the plans as comments.

## Instructions

You are helping the user triage GitHub issues. Follow these steps carefully:

### 1. Check GitHub Authentication

First, verify GitHub CLI is authenticated:
```bash
gh auth status 2>&1
```

If not authenticated, inform the user to run `gh auth login` and stop.

### 2. Fetch All Open Issues

Retrieve all open issues from the repository:
```bash
gh issue list --state open --json number,title,labels,body,url,comments --limit 100
```

### 3. Filter Untriaged Issues

From the fetched issues, identify those that:
- Do NOT have a label named `triaged`
- Are still open

Store the list of untriaged issue numbers for processing.

### 4. Process Each Untriaged Issue

For each untriaged issue, do the following:

#### 4.1. Read Issue Content
- Extract the issue title, body/description, and all comments
- If there are comments, read through all of them to understand the full context
- Look for any discussions, clarifications, or additional requirements

#### 4.2. Analyze and Devise a Plan

Using the issue content, create a comprehensive implementation plan that includes:

**Plan Structure:**
```markdown
## [Proposed Plan]

### Summary
[Brief 1-2 sentence overview of what needs to be done]

### Implementation Steps
1. [Specific step with file references like `src/file.rs:123`]
2. [Another step with technical details]
3. [Testing approach]
4. [Documentation updates if needed]

### Code Changes Required
- **File**: `path/to/file.rs`
  - [What needs to change and why]
- **File**: `path/to/another.rs`
  - [What needs to change and why]

### Technical Approach
[Explain the technical approach, patterns, or architecture to use]

### Acceptance Criteria
- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Criterion 3

### Estimated Complexity
[Low/Medium/High] - [Brief justification]

### Dependencies
[List any dependencies on other issues, PRs, or external factors]
```

**Guidelines for Creating Plans:**
- Be specific with file paths and line numbers when possible
- Reference existing code patterns in the repository
- Consider edge cases and error handling
- Think about testing requirements
- Consider backward compatibility
- Note any breaking changes
- Identify dependencies or prerequisites

#### 4.3. Search Codebase for Context

Before finalizing the plan, use the Explore agent to understand the relevant codebase:
- Search for related files, functions, or patterns
- Understand existing architecture that the fix should align with
- Identify similar implementations that can be referenced
- Check for existing tests that need updates

Use the Task tool with `subagent_type="Explore"` and set thoroughness to "medium":
```
Task(
  subagent_type="Explore",
  description="Search codebase for [relevant context]",
  prompt="Find all files related to [feature/bug area]. Understand the current implementation approach and architecture patterns used."
)
```

#### 4.4. Post Plan as Comment

After devising the plan, post it as a comment on the issue using:
```bash
gh issue comment <issue-number> --body "$(cat <<'EOF'
[Your plan content here]
EOF
)"
```

**Important:**
- Always prefix the comment with `## [Proposed Plan]` heading
- Use proper markdown formatting
- Include code blocks with syntax highlighting where appropriate
- Make sure the heredoc syntax is correct with proper EOF markers

### 5. Handle Errors Gracefully

If you encounter issues:
- **No untriaged issues found**: Report "All issues have been triaged ✅"
- **Cannot create plan**: Explain why (e.g., "Issue lacks sufficient detail to create a plan")
- **API failures**: Show error and suggest user checks authentication or network
- **Cannot find relevant code**: Note in the plan that exploration is needed

### 6. Provide Summary Report

After processing all untriaged issues, provide a summary:

```
## Issue Triage Summary

**Total untriaged issues processed**: X

### Successfully Triaged:
- Issue #123: [title] - Plan posted ✅
- Issue #124: [title] - Plan posted ✅

### Could Not Triage:
- Issue #125: [title] - [Reason why plan couldn't be created]

### Next Steps:
- Review proposed plans and adjust as needed
- Add `triaged` label to reviewed issues
- Assign issues to team members
```

## Guidelines

### What Makes a Good Triage Plan

**Comprehensive:**
- Covers all aspects mentioned in the issue
- Addresses edge cases
- Includes testing strategy
- Considers documentation needs

**Specific:**
- Uses exact file paths and line numbers when possible
- References specific functions, types, or modules
- Provides code examples or pseudocode
- Mentions specific error messages or logs to check

**Actionable:**
- Clear step-by-step approach
- Realistic acceptance criteria
- Identifies dependencies upfront
- Notes any blockers or unknowns

**Contextual:**
- Aligns with existing codebase patterns
- References similar implementations
- Considers project architecture
- Respects coding standards

### When to Skip Creating a Plan

Skip plan creation if:
- Issue is too vague or lacks critical information
- Issue is a duplicate of another issue
- Issue requires user clarification before proceeding
- Issue is actually a question or discussion, not actionable

In these cases, add a comment asking for clarification instead of a plan.

## Examples

### Example 1: Bug Fix Plan

**Issue**: "[bug] login fails when password contains special characters"

**Plan Posted**:
```markdown
## [Proposed Plan]

### Summary
Fix password handling to properly encode special characters during authentication.

### Implementation Steps
1. Update password encoding in `src/auth/login.rs:45-67`
2. Add URL encoding for special characters before transmission
3. Update password validation regex in `src/auth/validator.rs:23`
4. Add unit tests for special character scenarios
5. Update integration tests in `tests/auth_test.rs`

### Code Changes Required
- **File**: `src/auth/login.rs`
  - Wrap password in `urlencoding::encode()` before API call
  - Add error handling for encoding failures
- **File**: `src/auth/validator.rs`
  - Update PASSWORD_REGEX to allow all special chars: `!@#$%^&*()`
  - Add validation tests

### Technical Approach
Use the `urlencoding` crate (already in dependencies) to encode passwords before transmission. This ensures special characters are properly handled in HTTP requests.

### Acceptance Criteria
- [ ] Passwords with special chars (!@#$%^&*()) authenticate successfully
- [ ] Existing passwords without special chars still work
- [ ] Unit tests cover at least 5 special char combinations
- [ ] Integration tests verify end-to-end authentication

### Estimated Complexity
Low - Straightforward encoding fix, well-understood problem

### Dependencies
None - can be implemented immediately
```

### Example 2: Feature Implementation Plan

**Issue**: "[feat] add user profile management with encrypted storage"

**Plan Posted**:
```markdown
## [Proposed Plan]

### Summary
Implement a user profile system with encrypted local storage for credentials and preferences.

### Implementation Steps
1. Create `UserProfile` struct in new module `src/profiles/mod.rs`
2. Implement encryption layer using `aes-gcm` crate
3. Add profile CRUD operations (create, read, update, delete)
4. Create CLI commands for profile management
5. Update config to support multiple profiles
6. Add profile switching mechanism
7. Write comprehensive tests

### Code Changes Required
- **File**: `src/profiles/mod.rs` (new)
  - Define `UserProfile` struct with fields: username, encrypted_credentials, preferences
  - Implement `ProfileManager` for CRUD operations
- **File**: `src/profiles/encryption.rs` (new)
  - Implement AES-GCM encryption/decryption
  - Key derivation from user password using Argon2
- **File**: `src/config.rs`
  - Add `active_profile` field
  - Add profile directory path resolution
- **File**: `src/cli/mod.rs`
  - Add subcommands: `profile create`, `profile list`, `profile switch`, `profile delete`

### Technical Approach
- Use `aes-gcm` for encryption and `argon2` for key derivation (add to Cargo.toml)
- Store profiles at `~/.robert/profiles/{username}/profile.encrypted`
- Use JSON for serialization (with `serde_json`)
- Implement `zeroize` trait for secure memory cleanup
- Follow existing CLI command patterns in `src/cli/`

### Acceptance Criteria
- [ ] Users can create profiles with username/password
- [ ] Credentials are encrypted at rest using AES-256-GCM
- [ ] Users can switch between profiles
- [ ] Profile data is isolated per user
- [ ] Password verification works on profile load
- [ ] Unit tests achieve >90% coverage
- [ ] Integration tests verify end-to-end encryption

### Estimated Complexity
High - New feature with security requirements, multiple components

### Dependencies
- Need to add crates: `aes-gcm = "0.10"`, `argon2 = "0.5"`, `zeroize = "1.6"`
- Should complete before implementing issue #X (agent configs per user)
```

## Important Notes

### Using the Explore Agent

Always use the Explore agent to gather codebase context before creating plans. This ensures your plans are accurate and align with existing patterns:

```
Use Task tool with:
- subagent_type: "Explore"
- thoroughness: "medium" (or "very thorough" for complex issues)
- prompt: Describe what to search for in the codebase
```

Example:
```
Task(
  subagent_type="Explore",
  description="Search for authentication code",
  prompt="Find all files related to user authentication, password handling, and login flows. Understand the current implementation approach."
)
```

### Security Considerations

When triaging security-related issues:
- Mark them with appropriate severity
- Avoid posting sensitive details publicly
- Recommend private disclosure if needed
- Consider impact and urgency

### Automation Best Practices

- Process issues in batches of 5-10 to avoid rate limits
- Use parallel processing where possible
- Cache codebase exploration results
- Log all actions for audit trail

## Error Handling

Handle these scenarios gracefully:

**Authentication Failure:**
```
❌ GitHub authentication required. Run: gh auth login
```

**No Untriaged Issues:**
```
✅ All issues have been triaged. Great job!
```

**Issue Lacks Detail:**
Post a comment asking for clarification instead:
```markdown
## [Needs More Information]

This issue needs additional details before we can create an implementation plan:

- [ ] Specific steps to reproduce (for bugs)
- [ ] Expected vs actual behavior
- [ ] Environment details (OS, version, etc.)
- [ ] Error messages or logs

Please update the issue with these details so we can propose a solution.
```

**Codebase Search Fails:**
Note in the plan:
```markdown
### Additional Exploration Needed
Could not locate existing implementations. Manual codebase review needed to:
- Identify relevant modules
- Understand current architecture
- Find similar patterns
```

## Output Format

Use this format for the final report:

```
# Issue Triage Report

Triaged X issues from the repository.

## Successfully Processed

### Issue #123: [bug] authentication fails
- **Status**: Plan posted ✅
- **Complexity**: Medium
- **URL**: https://github.com/org/repo/issues/123
- **Key changes**: Auth module refactoring needed

### Issue #124: [feat] add dark mode
- **Status**: Plan posted ✅
- **Complexity**: Low
- **URL**: https://github.com/org/repo/issues/124
- **Key changes**: CSS and theme system updates

## Skipped

### Issue #125: [question] how to configure X
- **Reason**: Discussion item, not actionable
- **Action**: Added clarifying comment

## Summary Statistics
- Total issues reviewed: X
- Plans created: Y
- Clarifications requested: Z
- Average complexity: Medium

## Next Steps
1. Review proposed plans for accuracy
2. Add `triaged` label to reviewed issues
3. Prioritize issues by complexity and impact
4. Assign issues to team members
```
