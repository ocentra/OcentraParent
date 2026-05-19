---
name: ocentra-parent-test-audit
description: Audit or write Ocentra Parent tests against the no test doubles and real-boundary proof standard.
---

# Ocentra Parent Test Audit

Use this skill when writing, reviewing, or fixing tests.

## Required Reading

- `.ocentra-ai/rules/ocentra-parent-test-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-validation.mdc`
- Any domain rule for the code under test

## Checklist

- The test exercises a real parser, contract, service, WebSocket loop, or UI path.
- No mock, fake, stub, spy, or replacement behavior is introduced.
- Assertions are exact and falsifiable.
- Invalid input or failure behavior is covered.
- The test name explains the behavior.
- The test belongs to one layer: unit, contract, integration, or e2e.

## Required Commands

Run the focused test first, then at least:

```powershell
npm run test:tooling
npm run lint:schema-boundaries
```

Before handoff after test changes, run:

```powershell
npm run validate
```
