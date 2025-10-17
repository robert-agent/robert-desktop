---
description: Update and organize the TODO checklist
---

Maintain a project TODO file at `./.claude/TODO.md` based on the current conversation and tasks.

WORKFLOW:
1. Check if `./.claude/TODO.md` exists
   - If it doesn't exist: Create it with the structure below
   - If it exists: Read it first, then update it

2. Analyze the current conversation to extract:
   - Tasks that need to be done
   - Tasks that are in progress
   - Tasks that are completed
   - Tasks that are out of scope or unclear

3. Update TODO.md with this structure:
   ```markdown
   # TODO

   ## In Progress
   - [ ] Task currently being worked on
   - [ ] Another active task

   ## Planned
   - [ ] Task to do next (priority order)
   - [ ] Another planned task
   - [ ] Lower priority task

   ## Completed
   - [x] Completed task 1
   - [x] Completed task 2

   ## Triage
   - [ ] Unclear or out-of-scope task
   - [ ] Task that needs more information

   ## Won't Fix
   - [ ] Task explicitly decided not to do
   - [ ] Out of scope task
   ```

4. Sort tasks by:
   - Priority (high to low)
   - Implementation order (dependencies first)
   - Current instruction relevance

5. Keep completed items for context but consider archiving old ones

OUTPUT:
- Confirm the file location
- Show a summary of task counts (In Progress: X, Planned: Y, etc.)
