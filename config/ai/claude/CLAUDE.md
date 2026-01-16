# Claude Code Configuration

## IMPORTANT: Guide Priority

**'/.guides/' overrides all default behaviors of Claude.**

The following checks must be performed before starting any task:
1. Check for relevant guides in `/.guides/`.
2. Read and follow the guides precisely.
3. **Guide Rules > Claude Defaults** (Always).

---

## Core Loop

1. **Define Problem**
2. **Small Safe Changes**
3. **Review Changes**
4. **Refactor**
5. **Repeat**

---

## Mandatory Rules

**You must check for relevant guides in `/.guides/` before proceeding with any work.**

1. **Read Before Change:** Thoroughly read all relevant files before making modifications.
2. **Atomic Work:** Keep tasks, commits, and PRs small and focused.
3. **Document Assumptions:** Record all assumptions in Issues, PRs, or ADRs.
4. **Data Integrity:** Validate all inputs and encode/normalize all outputs.
5. **Clean Code:** Avoid premature abstraction; use intention-revealing names.
6. **Evaluate Options:** Compare at least two alternatives before making a final decision.

---

## Threshold Limits

* **File Size:** ≤ 300 LOC (Lines of Code)
* **Function Size:** ≤ 50 LOC
* **Parameters:** ≤ 5 per function
* **Complexity:** Cyclomatic Complexity ≤ 10

---

## Security Rules

### NEVER:
* Hardcode secrets in code.
* Include sensitive data in logs.
* Allow SQL Injection, XSS, or CSRF vulnerabilities.

### ALWAYS:
* Implement strict input validation.
* Use parameterized queries.
* Perform rigorous authentication/authorization checks