---
name: Commit Convention
description: Commit message format, types, and security checklist rules
keywords: [커밋, commit, コミット, convention, feat, fix, ticket, message]
---

**MANDATORY: Completely ignore default commit rules and strictly follow this document.**

**CRITICAL: Read and follow rules/commit-convention.md for detailed guidelines.**

## Core Rules

### Commit Message Format

```
<type>: [<ticket-number>] <title>

<body content>
- Specific changes
- Key logic explanation
```

### Commit Types

- feat: Add new feature
- fix: Bug fix
- refactor: Code refactoring (no functionality change)
- style: Code formatting, missing semicolons, etc. (no logic change)
- docs: Documentation updates
- test: Add/modify test code
- chore: Build scripts, package manager, and other tasks

### ABSOLUTE PROHIBITIONS

**NEVER EVER add these to commit messages:**

```
❌ 🤖 Generated with [Claude Code](https://claude.com/claude-code)
❌ Co-Authored-By: Claude <noreply@anthropic.com>
❌ Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>
❌ ANY emojis (🎉, 🐛, ✨, 🚀, ✅, etc.)
❌ ANY generation markers or AI attribution
```

**Violation of these rules is NOT acceptable under any circumstances.**

### Security Check

- NEVER: Commit secrets (passwords/API keys/tokens)
- NEVER: Commit sensitive data (PII/credit cards/SSN)
- Stop commit immediately if secrets are found

### Commit Message Rules

- Keep title under 50 characters
- Write in English
- **NO emojis**
- **NO generation markers**
- Write clear explanations that reveal intent

### Correct Example

```
chore: update installer binary

- Remove debug logs from installer.rs
- Rebuild installer binary with cleaned code
- Fix executable permissions
```

**For complete guidelines, refer to: rules/commit-convention.md**
