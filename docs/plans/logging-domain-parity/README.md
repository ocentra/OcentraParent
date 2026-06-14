<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Plan`
> Kind: short plan entry point.
> Read when: starting or resuming logging-domain parity work.
> Stop rule: Do not continue into broad repo docs unless this file routes you there.
> Proves: only the local logging-domain parity scope stated by assigned workpack/proof rows.
> Does not prove: full repo logging completion, production telemetry readiness, product runtime readiness, or PR readiness unless routed proof says so.
> Proof rule: If status or claims change, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

# Logging Domain Parity Plan

This is the short, token-efficient entry point for `logging-domain-parity`.

The goal is to bring OcentraParent logging to parity with the mature games logging-domain pattern without turning the work into another proof-only/read-model-only pass.

## Reference and target

Reference implementation:

```text
ocentra-games/packages/logging-domain
```

Target implementation:

```text
OcentraParent/packages/logging-domain
OcentraParent/crates/logging-core
OcentraParent/scripts/dev/*
```

## Key local docs

Read in this order:

1. [AGENTS.md](AGENTS.md)
2. [PLAN_STATE.md](PLAN_STATE.md)
3. [NEXT_ACTIONS.md](NEXT_ACTIONS.md)
4. [WORKPACK_INDEX.md](WORKPACK_INDEX.md)
5. Open only the assigned workpack under [workpacks/](workpacks/)
6. Use [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md) for exact checklist rows
7. Use [PROOF_INDEX.md](PROOF_INDEX.md) when proof artifacts are needed

Reference source docs already written:

- [00 Current State and Reference Audit](00-current-state-and-reference-audit.md)
- [01 Parent Logging Architecture](01-parent-logging-architecture.md)
- [02 Rust Logging Core Crate](02-rust-logging-core-crate.md)
- [03 Local Validation Evidence](03-local-validation-evidence.md)
- [04 Validation and Enforcement](04-validation-and-enforcement.md)

## Current scope

This plan owns local developer/agent observability parity for OcentraParent:

```text
TypeScript logging-domain parity
bridge / NDJSON / DuckDB / query scripts
Rust logging-core crate
agent-service dev logging migration
portal dev-log routing fix
local validation evidence wrappers
validation/enforcement scripts
```

## Non-negotiable interpretation

Do not collapse these two concerns:

```text
product/runtime safe logging
local developer/agent observability
```

The existing parent logging-domain proof/read-model contracts remain. This plan adds the missing local observability pipeline.

## Implementation-first rule

Use this order:

```text
PLAN -> CODE -> TEST -> RUN/FIX -> PROOF -> DOC
```

Do not add another metadata-only contract package.
Do not write more proof-only docs before source behavior exists.
Do not use an LLM to summarize raw logs as the logging architecture.

## Handoff report format

Every completion report must include:

```text
Assigned workpack:
Real source behavior added:
Files changed:
Tests added/changed:
Focused commands run:
Proof artifacts:
Checklist/docs updated:
No-claim boundaries preserved:
Remaining gaps:
```

If `Real source behavior added` is empty, call the work proof-routing-only and justify why.
