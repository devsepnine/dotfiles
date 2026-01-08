---
keywords: [테스트, test, testing]
---

# Testing Rules

## Core Principles

- Add new tests for new code
- Bug fixes must include regression tests (write to fail first)
- Tests should be deterministic and independent
- Replace external systems with fakes/contract tests

## Test Coverage Requirements

### Unit Tests
- Cover all public functions
- Test edge cases and boundary conditions
- Test error handling paths

### Integration Tests
- Test component interactions
- Test database operations
- Test external service integrations (with mocks)

### E2E Tests
- Must include ≥1 success path
- Must include ≥1 failure path
- Cover critical user journeys

## Test Quality Checklist

- [ ] Tests are deterministic (same result every run)
- [ ] Tests are independent (no shared state)
- [ ] Tests serve as documentation (clear intent)
- [ ] Tests include boundary/failure cases
- [ ] External systems replaced with fakes/mocks

## Concurrency Testing

Proactively assess risks from:
- Concurrent access
- Lock contention
- Retry logic (duplicates, deadlocks)
- Race conditions

## Test Commands

```bash
# Run all tests
npm test
# or
yarn test

# Run specific test file
npm test -- path/to/test.spec.ts

# Run with coverage
npm run test:coverage
```
