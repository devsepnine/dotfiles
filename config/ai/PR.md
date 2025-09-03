## Pull Request Guide

### PR Title Format

```
[TICKET-ID] <One-line Summary>
```

Examples:
- `[PP-XXXX] Add user authentication system`
- `[PP-XXXX] Fix payment module bug`

### PR Description Template

```markdown
#### Issue Type
- [ ] Feature addition (feat)
- [ ] Feature removal (feat)
- [ ] Bug fix (fix)
- [ ] Refactoring (refactor)
- [ ] Performance improvement (perf)
- [ ] Dependencies, environment variables, config file updates (chore)
- [ ] Styling (style)
- [ ] Documentation (docs)
- [ ] Test code (test)

#### Priority
> Mark issue priority based on current JIRA `Priority` standards
- [ ] Blocker
- [ ] Urgent
- [ ] Critical
- [ ] Major
- [ ] Trivial

#### Background
> Summarize what work is included in this PR and why this work was done

#### Changes
> List major changes. Add additional comments for parts that might be difficult for reviewers to understand

**API Changes:**
- [ ] No Breaking Changes
- [ ] Breaking Changes (affects backward compatibility)

**Database Changes:**
- [ ] No schema changes
- [ ] Schema changes (migration required)
- [ ] Data migration required

**Major Changed Files:**
- `path/filename.ext` - Summary of changes

#### Testing
> Testing content before PR request. List test cases briefly

**Automated Tests:**
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] E2E tests pass
- [ ] New tests added (for new code)
- [ ] Regression tests added (for bug fixes)

**Manual Testing:**
- [ ] Normal operation confirmed in local environment
- [ ] Normal operation confirmed in dev environment
- [ ] Browser compatibility confirmed (if applicable)
- [ ] Mobile responsive confirmed (if applicable)

**Performance Testing:**
- [ ] No performance impact
- [ ] Performance improvement confirmed
- [ ] Performance degradation exists (reason: )

#### Screenshots
> Attach Before/After screenshots if there are UI changes

**Before:**
<!-- Screenshot before changes -->

**After:**
<!-- Screenshot after changes -->

#### Links
> Add links to related documents, work tickets, design guide documents
- [ ] JIRA Ticket: [PP-XXXX](https://ggnetwork.atlassian.net/browse/PP-XXXX)
- [ ] Related Documentation: [Document Name](link)
- [ ] Design Guide: [Figma](link)
- [ ] Related PR: #number

#### Checklist
> Essential checklist before creating PR
- [ ] Self-review completed
- [ ] Commit messages follow convention
- [ ] Code follows conventions
- [ ] Unnecessary console logs/comments removed
- [ ] No secrets or sensitive information included
- [ ] Documentation updated (if necessary)
- [ ] package-lock.json included when dependencies updated
- [ ] CHANGELOG updated for Breaking Changes
```

### PR Creation Commands

**Using GitHub CLI (Recommended):**
```bash
# If upstream exists: Create PR with upstream/develop as target
gh pr create --base upstream/develop --title "[PP-XXXX] Work description" --body-file .github/PULL_REQUEST_TEMPLATE.md

# If no upstream: Create PR with origin/develop as target
gh pr create --base origin/develop --title "[PP-XXXX] Work description" --body-file .github/PULL_REQUEST_TEMPLATE.md

# Auto-select appropriate target (simple version) - defaults to develop branch
gh pr create --base develop

# Create PR with user-specified branch (e.g., hotfix, release branches)
gh pr create --base feature/hotfix-branch --title "[PP-XXXX] Work description"
```

**Branch Strategy:**
```bash
# If upstream exists
git fetch upstream
git checkout develop
git merge upstream/develop
git checkout -b feature/PP-XXXX-description
# After work completion
gh pr create --base upstream/develop

# If no upstream (origin only)
git fetch origin
git checkout develop
git merge origin/develop
git checkout -b feature/PP-XXXX-description
# After work completion
gh pr create --base origin/develop
```

### Default Settings

**Target Branch Settings:**
- **Default Targets**: `upstream/develop` or `origin/develop`
- **Priority**: `upstream/develop` > `origin/develop`
- If upstream is configured: Use upstream/develop as target
- If no upstream: Use origin/develop as target
- **User-specified branches**: User-specified branches are allowed (e.g., `--base feature/hotfix`)

**Required Checks:**

**Code Quality:**
- File size limit: ≤ 300 LOC
- Function size limit: ≤ 50 LOC  
- Parameter limit: ≤ 5
- Cyclomatic complexity: ≤ 10
- Split/refactor if limits exceeded

**Security Checks:**
- NEVER: Include secrets (passwords/API keys/tokens)
- NEVER: Include sensitive data (PII/credit card/SSN)
- NEVER: Create SQL injection, XSS, CSRF vulnerabilities
- ALWAYS: Validate, normalize, encode all inputs
- ALWAYS: Use parameterized queries
- ALWAYS: Apply authentication/authorization checks

**Testing Requirements:**
- New code → New tests required
- Bug fixes → Regression tests required
- Write tests to fail first, then fix
- E2E tests: ≥1 success and ≥1 failure path each

**PR Size Principles:**
- Keep work, commits, and PRs small
- Split into logical units
- Must be independently buildable/testable

### Pre-PR Creation Checklist

**1. Code Quality Check**
```bash
# Lint check
npm run lint
# or
yarn lint

# Type check
npm run type-check
# or
yarn type-check

# Run tests
npm test
# or
yarn test
```

**2. Essential Checks**
- [ ] **Branch Check**: Confirm current branch is a feature branch
- [ ] **Update**: Reflect latest changes from target branch (upstream/develop or origin/develop)
- [ ] **Commit Cleanup**: Squash unnecessary commits
- [ ] **Conflict Resolution**: Ensure no merge conflicts
- [ ] **Test Execution**: Confirm all tests pass
- [ ] **Documentation Update**: Update related documentation if needed
- [ ] **Build Check**: Confirm build succeeds

**3. Security Check**
- [ ] No secrets, API keys, tokens, or sensitive information included
- [ ] Development debug code removed
- [ ] Console logs cleaned up

### Review Guidelines

**Reviewer Perspective:**
- [ ] **Functionality**: Is the requirement correctly implemented?
- [ ] **Code Quality**: Is readability and maintainability good?
- [ ] **Design**: Are appropriate architecture and patterns used?
- [ ] **Security**: Are there no security vulnerabilities?
- [ ] **Performance**: Is there no negative performance impact?
- [ ] **Testing**: Does it have appropriate test coverage?
- [ ] **Documentation**: Is necessary documentation provided?

**Author Perspective:**
- [ ] **Self Review**: Complete self-review before creating PR
- [ ] **Provide Context**: Clearly explain reasons and intentions for changes
- [ ] **Review Response**: Respond to feedback within 24 hours
- [ ] **Apply Changes**: Quickly reflect requested modifications
- [ ] **CI/CD**: Pass all automated checks

### Frequently Used Commands

**PR Status Check:**
```bash
# View PR list
gh pr list

# View specific PR details
gh pr view <PR-number>

# Checkout PR (for review)
gh pr checkout <PR-number>
```

**PR Updates:**
```bash
# Edit PR title/description
gh pr edit <PR-number> --title "New Title" --body "New Description"

# Change Draft PR to Ready
gh pr ready <PR-number>

# Merge PR
gh pr merge <PR-number> --squash
```

### PR Creation and Review Best Practices

**Before Creating PR:**
1. **Self-Review**: Review your own changes thoroughly
2. **Test Locally**: Run all tests and verify functionality
3. **Check Dependencies**: Ensure all dependencies are properly updated
4. **Documentation**: Update relevant documentation
5. **Clean History**: Squash commits into logical units

**During Review Process:**
1. **Respond Promptly**: Address feedback within 24-48 hours
2. **Ask Questions**: Clarify unclear feedback
3. **Test Changes**: Verify requested changes work as expected
4. **Update Tests**: Add/modify tests for changed functionality
5. **Keep PR Updated**: Rebase with target branch if needed

**After Approval:**
1. **Final Check**: Ensure CI/CD passes
2. **Merge Strategy**: Use squash merge for clean history
3. **Clean Up**: Delete feature branch after merge
4. **Notify Team**: Inform relevant team members if needed

### Common PR Patterns

**Feature PR:**
- Includes new functionality
- Has comprehensive tests
- Updates documentation
- Follows feature flag patterns if applicable

**Bug Fix PR:**
- Includes regression test
- Explains root cause
- Has minimal scope
- References original issue

**Refactoring PR:**
- No functional changes
- Improves code structure
- Maintains test coverage
- Has clear refactoring goals

### Troubleshooting

**Common Issues:**
- **Merge Conflicts**: Rebase with target branch
- **Failed Tests**: Fix failing tests before requesting review
- **Large PR**: Split into smaller, logical PRs
- **Missing Context**: Add more detailed description

**Emergency Hotfix Process:**
1. Create hotfix branch from main/production
2. Make minimal necessary changes
3. Fast-track review process
4. Deploy and monitor closely
5. Follow up with proper fix in develop branch

### PR Language Guidelines

**Write in English with proper technical terminology**
- Descriptions, summaries: English writing
- Technical terms: Use standard English technical vocabulary
- Example: "Add caching logic to improve API response time"
- Maintain consistency with established technical documentation standards