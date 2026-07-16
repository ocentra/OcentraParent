# Dev UI and Playwright Proof Standard

## Purpose

Do not wait for polished final UI to prove a behavior chain.

Use a safe dev/test command surface first. Prove the command path, event path, logs, read models, and failure states. Then wire polished UI to the already-proven command path.

## Required pattern

```text
dev/test route or panel
-> typed command button
-> command/event/request emitted
-> owner handles it
-> log/event/read-model proof collected
-> Playwright verifies visible state and proof trace
-> final UI reuses same command/read-model path
```

## Dev route rules

| Rule | Requirement |
| --- | --- |
| no fake state | Dev UI may trigger commands, but must not invent runtime state. |
| no product claim | Dev UI proves a chain, not final UX polish. |
| typed commands | Buttons must call typed command/request helpers, not arbitrary strings. |
| traceable | Every click must carry run id and correlation id. |
| reversible | Dev-only surfaces must be clearly named and removable or gated. |

## Playwright proof rule

A Playwright proof must verify at least two channels:

1. visible UI/read-model state;
2. log/event/proof artifact showing the runtime chain.

Screenshot-only proof is not enough for runtime claims.

## Final UI rule

After the dev command path is proven, final UI work should only change presentation or route wiring. It must not invent a second command path.

## Common failures

| Failure | Correction |
| --- | --- |
| Pretty UI before runtime proof | Build dev command route first. |
| UI test mocks backend state | Use real command/read-model path or mark as component-only. |
| Playwright only checks text | Also check log/event/proof artifact. |
| Portal claims runtime | Portal renders state; runtime proof belongs to service/domain chain. |
