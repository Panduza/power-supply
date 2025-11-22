# Tasks: TUI

**Purpose**: Break the `TUI` feature into small, focused tasks suitable for individual PRs.

1. Draft `1-reqs.md` using `0-stories.md` as the source of truth. (This file)
2. Review and convert each story into one or more FRs with explicit acceptance criteria.
3. Identify non-functional requirements and list testable outcomes.
4. Add unit tests for parsing/command handling (if applicable).
5. Create a minimal TUI prototype that renders the primary screen(s) in a terminal.
6. Add integration tests or manual test instructions for common flows.
7. Document usage and developer setup in `docs/` or `specs/05-tui/README.md` if needed.
8. Create a small PR per task, include links to `specs/05-tui/1-reqs.md` and `0-stories.md` in the PR title.

Notes:

- Keep tasks small: each should map to a single coherent change and be independently reviewable.
- If additional clarification is required for any task, add a `NEED CLARIFICATION` section to the related spec file and wait for answers before proceeding.
