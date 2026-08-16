<!-- agent-capsule -->

> Agent Capsule
> Doc: Local AI Provider Runtime Boundary
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Local AI Provider Runtime Boundary

Status: V0.7 boundary plan. This document defines the next safe local
provider/runtime adapter shape before any model execution is added. It does not
enable local AI execution, remote AI, policy enforcement, or portal-side
evaluation.

## Purpose

Ocentra Parent needs a local model/runtime boundary that can eventually host a
child-device safety model. The boundary must remain explicit before runtime
code exists:

- provider status can be reported while the adapter is unconfigured;
- local-only custody is the only accepted child-device safety mode;
- unavailable and degraded states are first-class status, not failures to hide;
- model execution stays disabled until a reviewed adapter slice adds it behind
  typed contracts;
- remote/API providers stay out of the child-device safety path.

The current runtime status command is therefore a status surface, not a model
runner. It is allowed to report an unavailable local provider so the context
builder, policy preview, portal, and tests can display why local AI is not ready.

## Current Baseline

The repo has these V0.7 pieces in the local provider/runtime boundary track:

- `LocalModelRuntimeStatus` and `LocalProviderCapability` are migration-era TS
  contract names; new product truth belongs in Rust-owned contracts and
  generated DTOs.
- Rust protocol parity for local AI runtime status.
- `agent.local-ai.runtime.status.get` and
  `agent.local-ai.runtime.status.reported` protocol names.
- A Rust service read path that reports an unconfigured provider with
  `loadState = unavailable`, no capability flags, and
  `degradedState = provider-unavailable`.
- A no-execution adapter probe shape that reports provider configuration state,
  probe state, readiness state, and `executionAllowed = false` without loading a
  model.
- Portal policy-preview UI that displays the reported runtime status and keeps
  enforcement disabled.

That baseline is intentionally conservative. It proves visibility without
claiming model cache, local inference, provider selection, or policy influence.

## Boundary Contract

The provider/runtime adapter boundary should be treated as three separate
surfaces.

| Surface             | Owns                                                                                                                           | Must not own                                                              |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------- |
| Runtime status      | Provider id, model id/ref, load state, capability flags, resource class, degraded state, unavailable reason, last checked time | Model output, policy action, evidence selection, enforcement              |
| Provider capability | Local-only privacy mode, supported task categories, resource class, fallback order                                             | Remote/API provider credentials, remote routing, child activity custody   |
| Adapter execution   | Future local model load/generate lifecycle after separate review                                                               | Hidden model calls inside capture, portal, policy, or enforcement modules |

Status and capability are safe to expose before execution exists. Adapter
execution is not safe to add until the following are true:

- the provider is local-only;
- the model artifact/cache boundary is explicit;
- execution has timeout, output schema, and failure contracts;
- output is parsed into typed local AI results;
- tests prove unavailable, degraded, failed load, invalid output, and timeout
  states without using test doubles;
- the deterministic policy evaluator consumes only schema-valid results and
  remains dry-run until enforcement work starts.

## Required Default State

Until a real local adapter lands, every runtime status response must be safe by
default:

| Field               | Required default                                |
| ------------------- | ----------------------------------------------- |
| `loadState`         | `unavailable`                                   |
| `capabilityFlags`   | empty                                           |
| `resourceClass`     | local hardware class only, normally `cpu`       |
| `degradedState`     | `provider-unavailable`                          |
| `unavailableReason` | explicit unconfigured-provider reason           |
| privacy             | local-only when provider capability is reported |
| remote/API provider | not present                                     |
| execution           | not attempted                                   |

The status command must not probe external APIs, fetch model manifests, start a
model process, load a model into memory, or infer provider readiness from files
unless a later adapter slice adds that behavior with tests.

## No-Execution Adapter Probe Slice

The first adapter probe is deliberately narrower than a runtime adapter. It may
report only local configuration and capability state that is already known to
the child-device service. In the default branch state, that means:

- `probeState = probe-unavailable`;
- `configurationState = local-provider-unconfigured`;
- `readinessState = adapter-not-ready`;
- runtime `adapterBoundary = local-adapter-unavailable`;
- probe contract `adapterBoundary = status-only`;
- `executionState = disabled`;
- `providerSource = unavailable`;
- `executionAllowed = false`;
- no capability flags and no model-cache claim.

This probe path is allowed to travel with the existing
`agent.local-ai.runtime.status.reported` event as flattened status fields. It is
not allowed to start a model process, read prompt input, emit model output,
select evidence, call a remote provider, or influence policy decisions. A later
adapter implementation can change the probe from unavailable to ready only when
local configuration, model artifact references, timeout behavior, output
parsing, and degraded states are all contract-backed and tested.

Readiness is a separate guard from the probe result. `adapter-not-ready` must
keep `executionAllowed = false`. `adapter-ready` is valid only when the provider
is configured, the probe is ready, the adapter boundary is a reviewed local
adapter, execution is dry-run-ready, the provider source is not unavailable, and
there is no unavailable reason. Invalid configuration or failed probes must stay
non-executable.

## Future Status Hardening

The current status contract carries these explicit boundary fields:

- `privacyMode`: `local-only` for child-device safety runtime status.
- `adapterBoundary`: `status-only`, `local-adapter-unavailable`, or a future
  reviewed local adapter state.
- `executionState`: `disabled`, `dry-run-ready`, `running`, `failed`, or
  another reviewed local-only lifecycle state.
- `providerSource`: local config, local model cache, OS capability probe, or
  unavailable.
- `readinessState`: not ready, ready, or invalid readiness for the adapter
  probe.

Future code slices should follow the Rust-first order: add fields in
`crates/schema`, `crates/parent-runtime-core`, or the owning Rust runtime crate;
mirror into `crates/agent-protocol` only when transport-specific Rust code needs
that shape; expose generated TypeScript DTOs or edge decoders only when a
consumer reads them; and render by the portal as status. They must not trigger
model execution.

## Prohibited Shortcuts

- Do not add a remote provider as fallback for child-device safety decisions.
- Do not treat billing, account, entitlement, notification, or release metadata
  as model/runtime evidence.
- Do not make the parent portal execute model calls or build prompts.
- Do not hide provider calls inside capture, query-store, policy, or enforcement
  modules.
- Do not let provider availability erase raw evidence, missing-evidence states,
  or parent rule conflicts.
- Do not use model output as policy authority without deterministic rule
  evaluation and evidence references.

## Adapter Slice Entry Criteria

A future adapter implementation may start only after this boundary is explicit
in code and tests:

1. Local provider/runtime status still reports unavailable when unconfigured.
2. Provider capability proves local-only custody and no remote fallback.
3. Model artifact references are opaque and do not expose local filesystem
   secrets to the portal.
4. Execution input is produced by the evidence context builder, not by a
   provider scanning the OS.
5. Output is parsed through the local AI safety result schema before policy use.
6. Invalid output, timeout, unavailable provider, and degraded provider all
   produce explicit degraded or unknown states.
7. Enforcement remains disabled by default.

## Validation Plan

For a docs-only boundary update:

```powershell
cmd /c npm run format:check
git diff --check
cmd /c npm run lanes:guard
cmd /c npm run hub:guard
```

For a future contract/status hardening update, use scoped Rust-first gates for
the touched owner and keep TS package gates only where an edge decoder or
generated consumer remains:

```powershell
cargo test -p <owning-rust-crate> <focused-filter>
cargo test -p ocentra-parent-agent-protocol local_ai_runtime
cargo test -p ocentra-parent-agent-service local_ai_runtime
cmd /c npm --workspace <generated-or-edge-ts-consumer> run test -- <focused-filter>
node scripts/check-source-shape.mjs
git diff --check
cmd /c npm run lanes:guard
cmd /c npm run hub:guard
```

## Done Signal

This boundary is ready for the next reviewed slice when the repo can explain,
through contracts or documentation, that local provider status is visible,
unconfigured is safe, local-only custody is required, remote AI is not part of
child-device safety, and no model execution happens before a dedicated adapter
implementation is reviewed.
