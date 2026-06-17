# ai-plan Event Architecture Instruction

## Owns

- AI contracts;
- context builders;
- provider scheduling;
- local AI runtime boundary;
- parent assistant contracts;
- AI result journals and read models.

## Must not own

- tracking/network/screen/browser/app-game evidence ownership;
- policy or enforcement action authority;
- parent-domain AI wrapper ownership.

## Required chain

```text
evidence owner publishes typed evidence
-> orchestrator creates AI request
-> AI owner routes and records result
-> policy/action owner consumes typed result
-> proof checks log, journal, and read model
```

## Logging/proof

Log context source, route decision, provider decision, degraded/refused state, result journal write, and no-direct-action boundary.

## Tests

AI-domain owns AI tests. Cross-domain evidence-to-AI-to-action proof belongs in app/service/proof runner, not inside the evidence owner.

## First architecture slice

Run AI ownership and architecture cleanup: remove parent-domain AI wrappers, clean source-layout pollution if present, then rebase tests into real categories before local AI core proof.
