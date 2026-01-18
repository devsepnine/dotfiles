## Core Loop

**Define Problem** → **Small Safe Change** → **Review Change** → **Refactor** — Repeat.

## Essential Rules

1. **Read related files** before making any changes.
2. Keep tasks, commits, and PRs **small**.
3. Record **assumptions** in Issues, PRs, or ADRs.
4. **Validate all inputs** and encode/normalize all outputs.
5. Avoid **premature abstraction**; use names that reveal intent.
6. Compare at least **two alternatives** before making a decision.

## Development Workflow

Before you start coding, if the problem is complex or unclear, draft a **Problem 1-Pager** including the following items. If any items are ambiguous, request an interview to clarify.

* **Background:** Context and motivation for the change.
* **Problem:** What specific issue are we trying to solve?
* **Goal:** What is the definition of success (the "success state")?
* **Non-goals:** What is explicitly out of scope?
* **Constraints:** Mandatory technical or business constraints.

## Threshold Limits

* **File Length:** ≤ 300 LOC
* **Function Length:** ≤ 50 LOC
* **Parameters:** ≤ 5
* **Cyclomatic Complexity:** ≤ 10

## Security Rules

* **NEVER:** Hardcode secrets in code/logs, leak sensitive data, or allow SQL Injection/XSS/CSRF vulnerabilities.
* **ALWAYS:** Use input validation, parameterized queries, and authentication checks.