---
description: Create or update a pull request with automated checks and fixes
---

Create or update a pull request from the current working branch, including automated CI/CD failure resolution.

🚨 CRITICAL CONSTRAINTS - THESE MUST BE FOLLOWED:
- NEVER force push to main/master branches
- Maximum 200 words for PR description in bullet points
- Only resolve CI/CD failures that are code-related (not infrastructure/credentials)
- Ask for user confirmation before pushing fixes for CI failures

WORKFLOW:

## 1. Push local commits and create/update PR
- Run `git status` to check current branch and uncommitted changes
- If there are uncommitted changes, stop and ask user if they want to commit first
- Run `git log origin/$(git branch --show-current)..HEAD` to see unpushed commits
- Push to origin: `git push -u origin HEAD`
- Check if PR exists: `gh pr view --json number,url` (if error, PR doesn't exist)
- If PR exists:
  - Get PR number from the output
  - Update PR description with `gh pr edit <number> --body <description>`
- If PR doesn't exist:
  - Create PR: `gh pr create --title "<title>" --body "<description>"`

## 2. Generate PR description (max 200 words, bullet points)
- Analyze commit history: `git log origin/main..HEAD --oneline` (or appropriate base branch)
- Review commit messages for context
- Check Claude session logs if available at `.claude/session.log` or similar
- Structure the description as:
  ```
  ## What was expected
  - [Bullet points of intended changes]

  ## What was achieved
  - [Bullet points of completed work with file references]

  ## Out of scope / Not yet done
  - [Bullet points of items not completed or intentionally excluded]
  ```

## 3. Check GitHub Actions status
- Wait 10 seconds for workflows to start
- Check PR checks: `gh pr checks`
- If all passing, report success and STOP
- If failures exist, continue to step 4

## 4. Resolve CI/CD failures
For each failing check:
- Get workflow run details: `gh run view <run-id> --log-failed`
- Analyze the error logs to identify:
  - Linting errors (use existing /lint command)
  - Test failures (examine test output)
  - Build errors (analyze compilation errors)
  - Type errors (check TypeScript/Rust type issues)
- If errors are code-related:
  - Present findings to user with specific file/line references
  - Ask: "I found these issues in the CI logs. Should I attempt to fix them?"
  - If approved, fix the issues
  - Run local validation (lint/test/build as appropriate)
  - Commit fixes: `git add -u && git commit -m "fix: resolve CI failures"`
  - Push: `git push`
  - Wait 10 seconds and re-check: `gh pr checks`
- If errors are infrastructure-related (credentials, permissions, timeouts):
  - Report to user and explain manual intervention is needed

## 5. Final report
Provide summary:
- PR URL
- Current check status
- Any unresolved issues requiring manual intervention
