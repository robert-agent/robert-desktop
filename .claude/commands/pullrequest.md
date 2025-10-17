---
description: Create or update a pull request with automated checks and fixes
---

Create or update a pull request from the current working branch, including automated CI/CD failure resolution.

🚨 CRITICAL CONSTRAINTS - THESE MUST BE FOLLOWED:
- NEVER force push to main/master branches
- Maximum 200 words for PR description in bullet points
- Only resolve CI/CD failures that are code-related (not infrastructure/credentials)
- Ask for user confirmation before pushing fixes for CI failures
- **PR title format**: Use `[tag] lowercase description` format
  - Tags in brackets: `[feat]`, `[fix]`, `[chore]`, `[docs]`, `[refactor]`, etc.
  - First word after bracket is lowercase
  - Examples: `[feat] add chat interface`, `[fix] resolve authentication bug`, `[chore] update dependencies`
  - NOT: `feat: Add chat interface` or `[feat] Add chat interface`

WORKFLOW:

## 0. Prerequisites check
- Verify GitHub CLI authentication: `gh auth status 2>&1`
- If not authenticated, inform user to run `gh auth login` and wait for them to authenticate
- Only proceed after authentication is confirmed

## 1. Push local commits and create/update PR
- Run `git status` to check current branch and uncommitted changes
- If there are uncommitted changes, stop and ask user if they want to commit first
- Check for unpushed commits: `git log origin/$(git branch --show-current)..HEAD --oneline 2>/dev/null || echo "No tracking branch yet"`
- If there are unpushed commits OR no tracking branch:
  - Push to origin: `git push -u origin HEAD`
- If no unpushed commits and tracking branch exists:
  - Branch is already pushed, proceed to PR check
- Check if PR exists: `gh pr view --json number,url,title 2>&1`
- If PR exists (JSON output):
  - Parse the JSON to get PR number
  - Update PR with: `gh pr edit <number> --title "<title>" --body "$(cat <<'EOF'\n<description>\nEOF\n)"`
- If PR doesn't exist (error message):
  - Create PR with: `gh pr create --title "<title>" --body "$(cat <<'EOF'\n<description>\nEOF\n)"`
- IMPORTANT: Always use heredoc syntax (shown above) for multiline PR descriptions to preserve formatting

## 2. Generate PR description (max 200 words, bullet points)
- First, detect the base branch (usually `main` or `master`): `git remote show origin | grep "HEAD branch" | cut -d' ' -f5`
- Analyze commit history: `git log <base-branch>..HEAD --oneline`
- For comprehensive understanding, also review:
  - Detailed commit messages: `git log <base-branch>..HEAD --format="%h %s%n%b"`
  - File changes: `git diff <base-branch>...HEAD --stat`
  - Key file diffs for context (if needed)
- Review commit messages for context
- If available, check conversation history or previous analysis in the session
- Structure the description as:
  ```
  ## What was expected
  - [Bullet points of intended changes/goals]

  ## What was achieved
  - [Bullet points of completed work with specific file references like `file.rs:123-145`]
  - [Include key features, bug fixes, refactorings]
  - [Mention new types, functions, or components added]

  ## Out of scope / Not yet done
  - [Bullet points of items not completed or intentionally excluded]
  - [Future work or known limitations]
  ```
- IMPORTANT: The description should accurately reflect what was actually achieved, not just paraphrase commit messages

## 3. Check GitHub Actions status
- First check current status: `gh pr checks`
- Analyze the output:
  - If checks show "pending" or are missing: Wait 10 seconds and check again with `sleep 10 && gh pr checks`
  - If all checks are passing: Report success and STOP
  - If any checks are failing: Continue to step 4
  - If checks haven't started yet: Wait up to 30 seconds total (check every 10s) for workflows to start
- NOTE: For PRs that were previously pushed, checks may already be complete

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
  - Commit fixes: `git add -u && git commit -m "[fix] resolve CI failures"`
  - Push: `git push`
  - Wait 10 seconds and re-check: `gh pr checks`
- If errors are infrastructure-related (credentials, permissions, timeouts):
  - Report to user and explain manual intervention is needed

## 5. Final report
Provide a clear summary with:
- PR URL (from the gh command output)
- PR title
- Overall status (✅ All passing / ⚠️ Some failures / ❌ Failed)
- List of all checks with their status and timing:
  - Format: `- ✅ Check Name (duration)`
  - Use ✅ for pass, ❌ for fail, ⏳ for pending
- If any failures remain:
  - List unresolved issues requiring manual intervention
  - Provide specific guidance on what the user needs to do
- Confirmation that PR is ready for review (if all checks pass)

## Tips for success
- **PR title format**: Always use `[tag] lowercase description` - never capitalize the first word after the bracket
  - Common tags: `[feat]` for features, `[fix]` for bug fixes, `[chore]` for maintenance, `[docs]` for documentation, `[refactor]` for code restructuring
- **Authentication**: Always check `gh auth status` first to avoid mid-workflow failures
- **Heredoc syntax**: Use heredoc for PR descriptions to preserve formatting: `--body "$(cat <<'EOF'\n...\nEOF\n)"`
- **Branch detection**: Auto-detect base branch instead of assuming `main`
- **Already pushed branches**: Handle gracefully when branch is already pushed (check for unpushed commits first)
- **Check timing**: Don't assume checks need 10 seconds - they might already be complete
- **Comprehensive analysis**: Review actual code changes, not just commit messages, for accurate PR descriptions
