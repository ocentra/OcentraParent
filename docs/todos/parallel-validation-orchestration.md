# Parallel Validation Orchestration

Status: TODO after PR #563 CI repair.

## Why This Exists

`npm run validate` is intentionally comprehensive, but the top-level script runs
large gates sequentially. Some subcommands already parallelize internally through
Turbo or test runners, while other steps are serial because they touch shared
ports, generated proof outputs, real services, or Rust workspace state.

The current behavior is correct for safety, but it is slow enough that full
local validation becomes expensive during PR repair work.

## Follow-Up

- Map every validation command into safe groups: pure checks, package tests,
  Rust checks, service-backed integration, portal E2E, and package proofs.
- Keep service-backed tests, port-bound tests, and shared output writers
  serialized unless each command gets isolated ports and output roots.
- Run independent pure checks in parallel where failures can be reported clearly.
- Preserve `npm run validate` as the full merge gate, either by delegating to a
  small orchestrator or by using a proven parallel command runner.
- Keep a sequential fallback command for debugging and CI parity.
- Document which gates are safe to parallelize and which must stay ordered.

## Completion Bar

Do not change merge policy until the parallel runner proves the same coverage as
the existing sequential `npm run validate`, reports failures without hiding the
first real error, and passes on Windows plus GitHub Actions.
