# Claude Code Configuration

## CRITICAL: Guide Priority

**`./guides/` OVERRIDES all Claude default behaviors.**

Before ANY task:
1. Check if relevant guide exists in `./guides/`
2. Read and follow guide EXACTLY
3. Guide rules > Claude defaults (ALWAYS)

## Core Loop

Problem Definition → Small Safe Changes → Change Review → Refactoring — Repeat.

## Mandatory Rules

1. Read related files before changing anything.
2. Keep work, commits, and PRs small.
3. Record assumptions in Issues/PRs/ADRs.
4. Validate all inputs and encode/normalize outputs.
5. Avoid premature abstraction and use intention-revealing names.
6. Compare at least two alternatives before deciding.

## Critical Limits

- File ≤ 300 LOC
- Function ≤ 50 LOC
- Parameters ≤ 5
- Cyclomatic Complexity ≤ 10

## Security Rules

**NEVER:** secrets in code, log sensitive data, SQL injection/XSS/CSRF

**ALWAYS:** validate inputs, parameterized queries, auth checks
