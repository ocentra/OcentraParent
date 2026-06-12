# Household AI Provider Mesh Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `Household AI Provider Mesh Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Goal

Define the event-driven household AI provider mesh before runtime code expands.
The mesh lets any trusted capable household device execute bounded AI work while
the evidence-owning child agent keeps evidence truth, AI work authority, result
validation, policy evaluation, enforcement handoff, audit, and read models.

Short rule:

```text
Every trusted device may help analyze; only the evidence-owning child agent may
decide and act.
```

## Existing Foundation

The repo already has these pieces:

- `crates/ocentra-eventing` as reusable local Rust eventing infrastructure with
  typed envelopes, custody, priority, aggregate keys, idempotency keys,
  request/response, bounded queue behavior, dead letter, topology proof,
  journal/replay, and testkit coverage.
- Local AI provider scheduler proof for one runtime access lane per physical
  device, child-safety priority, local queued/running/unavailable/degraded
  states, and duplicate same-device model-load blocking.
- LAN pairing and LAN AI job protocol pieces in `crates/agent-protocol` and
  `crates/agent-service/src/lan_pairing/lan_ai_job.rs`.
- Legacy screen family-hub route/runtime-discovery proof rows that preserve
  child-local first attempt, redacted crop route, no raw screenshot transfer, no
  retention, and no remote/API child-safety fallback.

Those foundations do not yet prove a decentralized household work mesh. The
missing pieces are provider gossip/heartbeat, distributed work offer, claim and
lease, duplicate prevention, result validation, retry/dead-letter/failover,
child-agent authority proof, mesh bridge event mapping, and topology proof.

## Core Authority Rule

AI safety authority is local to the evidence-owning child agent.

AI execution may run on:

- the same child device;
- a trusted paired household desktop or laptop AI provider;
- a limited or dormant mobile provider only as a controlled fallback;
- never a remote/API provider for normal child safety.

AI providers are workers only. They execute bounded jobs and return
schema-valid results. Providers cannot decide policy, apply configuration,
issue enforcement, mutate child runtime state, or publish policy/enforcement
events.

The evidence-owning child agent owns:

- evidence truth;
- encrypted journal and read models;
- AI work ledger;
- result validation;
- deterministic parent policy evaluation;
- enforcement handoff;
- audit;
- child-device configuration application.

## Three-Plane Architecture

### Local Runtime Plane

Each Rust runtime owns its own local `ocentra-eventing` bus instance:

```text
Parent desktop Rust runtime
  -> local event bus

Child PC Rust runtime
  -> local event bus

Child mobile Rust runtime
  -> local event bus

Other household desktop/laptop runtime
  -> local event bus
```

Local eventing is in-process decoupling only. It is not one shared LAN-wide
event bus.

Required rule for every related plan:

```text
ocentra-eventing is local runtime infrastructure only. Cross-device
coordination is handled by a Household Mesh Bridge that converts selected local
events into typed authenticated LAN messages and republishes validated incoming
messages into the receiving runtime's local bus.
```

### LAN Mesh Coordination Plane

The missing system is the Household Mesh Bridge. It is a consumer of the local
event bus, not the event bus itself.

Responsibilities:

```text
subscribe to selected local events
  -> validate whether they may leave the runtime
  -> convert to typed LAN protocol messages
  -> send to trusted peers

receive typed LAN protocol messages
  -> authenticate, authorize, and validate
  -> map to local event contracts
  -> publish into the receiving runtime's local bus
```

Important non-goals:

- Do not broadcast all local events over LAN.
- Do not let remote peers publish directly into another runtime's bus.
- Do not make `ocentra-eventing` a cross-device broker.
- Do not let LAN messages bypass local validation.
- Do not let parent UI publish business events directly.

Only selected categories may cross LAN:

- device discovery;
- provider advertisement;
- provider heartbeat;
- provider capability update;
- provider stale/offline/revoked state;
- AI work offer or availability query;
- AI work claim request;
- AI work claim grant/reject;
- AI work lease renew/expire;
- AI job payload transfer;
- AI result return;
- config command from parent/controller to child;
- approval or override command;
- read-model/query request.

Local-only events such as raw capture internals, handler state, adapter
internals, and private queue mechanics must not be broadcast.

### Parent Assistant And Control Plane

The parent portal and assistant are not the household AI provider mesh.

The portal can show activity, AI status, provider status, job status,
explanations, approval prompts, rule/config controls, and assistant questions.
It must not own capture, AI safety decisions, policy authority, enforcement, or
child runtime state mutation.

The assistant may help the parent ask questions or initiate approved
configuration flows through parent Rust APIs. The child runtime still validates
pairing, authority, schema, policy, and command applicability before applying
configuration locally.

## Runtime Roles

Every installed Rust runtime may expose one or more roles:

- `child-agent`;
- `parent-controller`;
- `parent-observer`;
- `ai-provider`.

Examples:

- Child desktop app: `child-agent`, optional `ai-provider`.
- Parent desktop app: `parent-controller`, `parent-observer`, preferred
  `ai-provider`.
- Other household desktop/laptop: optional `ai-provider` and
  `parent-observer`.
- Parent mobile app: `parent-controller`, `parent-observer`, dormant/limited
  `ai-provider`.
- Child mobile app: `child-agent`, dormant/limited/fallback `ai-provider`.

## Provider Priority Model

Preferred AI providers:

1. Parent desktop/laptop AI provider.
2. Other trusted household desktop/laptop AI provider.
3. Child desktop/laptop same-device AI provider.
4. Parent mobile dormant provider only as fallback.
5. Child mobile minimal local AI only when away from LAN or when no better
   provider exists.

Mobile provider default:

```text
providerClass = mobile-dormant
heavyJobs = false
defaultClaimBehavior = do-not-claim
eligibleOnlyWhen:
  - no desktop/laptop provider is available;
  - battery state is acceptable;
  - thermal state is acceptable;
  - parent policy allows fallback;
  - job kind is permitted for mobile;
  - child is out of LAN or no better local provider exists.
```

## Main Event-Driven Flows

### Parent Changes Child Configuration

```text
Parent UI / Assistant
  -> parent Rust command
  -> parent_controller.parent_action.received
  -> parent_controller.command.validated
  -> parent_controller.child_config_update.forward_requested
  -> Household Mesh Bridge sends typed LAN config command
  -> Child Mesh Bridge receives command
  -> child_agent.command.received
  -> child_agent.command.accepted or child_agent.command.rejected
  -> child_agent.config_update.requested
  -> child_agent.config_updated
  -> screen_capture.schedule_changed
  -> audit.entry.committed
  -> portal.read_model.updated
```

Parent UI is input only. Parent Rust validates parent authority. Child Rust
validates command, pairing, route, authority, schema, and policy before local
application.

### Child Captures Screen And Needs AI

```text
screen.capture.completed
  -> screen.evidence.created
  -> evidence.bundle.created
  -> ai.work.queued
  -> ai.work.route_selection.requested
  -> ai.work.route_selected
```

If the local device is enough:

```text
ai.work.claim.granted(local)
  -> ai.work.started
  -> local OCR/VLM/text/deterministic worker
  -> ai.work.completed
  -> ai.result.validation.requested
  -> ai.result.accepted
  -> policy.evaluation.requested
```

If a trusted LAN provider is better:

```text
ai.work.queued
  -> Household Mesh Bridge sends AiWorkOffer/AiWorkAvailabilityRequest
  -> trusted provider responds or claims
  -> ai.work.claim.requested
  -> child agent validates claim
  -> ai.work.claim.granted
  -> LAN payload sent to provider
  -> provider local bus: ai.remote_work.received
  -> provider local bus: ai.work.started
  -> provider local bus: ai.work.completed
  -> provider Mesh Bridge returns AiWorkResult
  -> child Mesh Bridge receives result
  -> child local bus: ai.result.validation.requested
  -> child local bus: ai.result.accepted or ai.result.rejected
  -> policy.evaluation.requested only after accepted result
```

### Provider Advertisement And Heartbeat

Each AI-capable runtime publishes local capability/heartbeat state. The bridge
exports a bounded advertisement carrying:

- provider peer id;
- physical device id;
- runtime role;
- provider class;
- route id;
- trust state;
- supported job kinds and modalities;
- resource class;
- queue depth and max concurrency;
- current load;
- battery and thermal state;
- custody modes;
- `lastSeenAt`;
- `expiresAt`.

Remote child runtimes project validated provider state into local
`ai.provider.*` read models.

### Claim, Lease, And Duplicate Prevention

The child agent owns the work ledger. A provider does not own the queue.

The child grants a claim only when:

- the job exists, is queued, and has not expired;
- the provider is paired and trusted;
- the route is allowed;
- the provider supports the required capability and modality;
- the custody mode is allowed;
- no active lease exists;
- the `dedupeKey` is not already complete;
- the provider is not stale, offline, or revoked;
- battery, thermal, and resource rules allow the provider.

Grant creates:

```text
ai.work.claim.granted
ai.work.lease.created
```

Reject reasons include:

- `already-claimed`;
- `duplicate-complete`;
- `stale-provider`;
- `revoked-provider`;
- `unsupported-capability`;
- `custody-mismatch`;
- `mobile-dormant`;
- `resource-degraded`;
- `expired-job`;
- `wrong-child-device`;
- `wrong-route`.

Expired leases publish `ai.work.lease.expired` then requeue or dead-letter
according to attempt policy.

### Result Validation And Policy Authority

A provider returns only a result. It cannot publish:

- `policy.decision.completed`;
- `enforcement.command.issued`;
- `child_agent.config_updated`.

The child validates:

- job id exists;
- claim id matches the active lease;
- provider peer id matches the lease owner;
- result is within deadline or explicitly stale;
- schema validates;
- evidence refs match allowed evidence refs;
- custody policy was respected;
- raw screenshot was not retained or transferred unless explicitly allowed;
- prompt/template version matches;
- model/runtime refs are present;
- confidence and degraded state are valid.

Only then can the child publish `ai.result.accepted` and request policy
evaluation. Rejected results requeue or dead-letter.

## Required Contracts

### Provider Contracts

- `AiProviderAdvertisement`;
- `AiProviderHeartbeat`;
- `AiProviderCapabilitySnapshot`;
- `AiProviderReachabilityState`;
- `AiProviderResourceState`;
- `AiProviderClass`;
- `AiProviderEligibility`;
- `AiProviderSelectionDecision`.

Provider classes:

- `desktop-preferred`;
- `laptop-preferred`;
- `child-desktop-local`;
- `mobile-dormant`;
- `mobile-fallback`;
- `mobile-unavailable`;
- `remote-assistant-only`.

### Work Contracts

- `AiWorkItem`;
- `AiWorkState`;
- `AiWorkKind`;
- `AiWorkPriority`;
- `AiWorkPayloadMode`;
- `AiWorkCustodyPolicy`;
- `AiWorkDedupeKey`;
- `AiWorkLease`;
- `AiWorkClaimRequest`;
- `AiWorkClaimDecision`;
- `AiWorkResult`;
- `AiWorkResultValidation`;
- `AiWorkDeadLetter`.

Required `AiWorkItem` fields:

- schema version;
- job id;
- dedupe key;
- aggregate key;
- child device id;
- owning agent peer id;
- child profile id;
- requested-by event id;
- correlation id;
- causation id;
- source evidence refs;
- parent rule refs;
- job kind;
- required capability;
- required modality;
- priority;
- not-before time;
- deadline;
- TTL;
- max attempts;
- attempt number;
- payload mode;
- custody policy;
- result schema ref;
- prompt template version;
- `policyAuthority = child-agent-only`;
- created-at time.

Required `AiWorkLease` fields:

- schema version;
- job id;
- claim id;
- lease owner provider peer id;
- lease owner physical device id;
- lease granted by child agent id;
- lease granted at;
- lease expires at;
- attempt number;
- accepted payload mode;
- provider capability snapshot ref.

Required `AiWorkResult` fields:

- schema version;
- job id;
- claim id;
- result id;
- provider peer id;
- provider runtime ref;
- model id;
- model reference;
- prompt template version;
- output kind;
- structured result;
- confidence;
- unknown state;
- degraded state;
- evidence refs used;
- raw input retained;
- raw output retained;
- completed at;
- duration.

Required `AiWorkResultValidation` fields:

- schema version;
- job id;
- claim id;
- result id;
- validation state;
- validated by child agent id;
- validated at;
- accepted local AI result id;
- rejection reason;
- custody state;
- evidence ref state;
- policy authority state.

## Event Taxonomy

Provider events:

```text
ai.provider.advertised
ai.provider.heartbeat.observed
ai.provider.capability.updated
ai.provider.selected
ai.provider.rejected
ai.provider.stale
ai.provider.unavailable
```

Work lifecycle events:

```text
ai.work.queued
ai.work.route_selection.requested
ai.work.route_selected
ai.work.claim.requested
ai.work.claim.granted
ai.work.claim.rejected
ai.work.lease.created
ai.work.lease.renewed
ai.work.lease.expired
ai.work.started
ai.work.progress.reported
ai.work.completed
ai.work.failed
ai.work.timed_out
ai.work.canceled
ai.work.requeued
ai.work.dead_lettered
```

Result events:

```text
ai.result.received
ai.result.validation.requested
ai.result.accepted
ai.result.rejected
ai.result.journaled
ai.result.projected
```

## Screen Payload Custody

Screen-derived AI mesh jobs must default to `raw-image-forbidden`.

Allowed payload modes:

- `metadata-only`;
- `screen-summary-only`;
- `ocr-text-only`;
- `redacted-crop`;
- `encrypted-local-artifact-ref`;
- `raw-image-forbidden`;
- `raw-image-explicit-opt-in-only`.

Required validation:

- raw screenshots are not sent to LAN providers by default;
- redacted crop payloads are bounded and evidence-cited;
- results cite image digest and source evidence refs;
- child agent validates deletion/custody before policy;
- provider cannot retain raw payload unless explicit opt-in contract exists;
- remote/API AI cannot receive screenshots by default.

## UI/UX

Household AI provider surfaces must show:

- discovered AI-capable devices;
- device roles;
- provider class;
- trust/pairing state;
- capability flags;
- supported job kinds;
- queue depth;
- current job state;
- last heartbeat/reachability;
- battery/thermal/resource degraded state;
- eligibility for child-safety work;
- worker-only/non-authority status.

AI job surfaces must show:

- claim owner;
- claim id;
- lease expiry;
- claim rejection reason;
- requeue/dead-letter state;
- result validation state;
- child-agent authority state;
- payload/custody mode.

Required snapshots include no-provider, desktop-provider, mobile-dormant,
claimed job, competing claim rejected, lease expired/requeued, result accepted,
result rejected, child-agent authority, and raw screenshot transfer disabled.

## Proof Packs

Mesh proof packs must include:

- event chain manifest path;
- mesh bridge proof path;
- provider discovery/capability proof path;
- AI work lifecycle proof path;
- claim/lease/idempotency proof path;
- result validation proof path;
- policy authority proof path;
- raw-payload custody proof path;
- mobile dormant/fallback proof path;
- topology/orphan-event proof path.

Expected proof scripts:

- `scripts/test/household-ai-provider-mesh-contract-proof.mjs`;
- `scripts/test/household-mesh-event-bridge-proof.mjs`;
- `scripts/test/household-ai-provider-claim-lease-proof.mjs`;
- `scripts/test/household-ai-provider-result-validation-proof.mjs`;
- `scripts/test/child-agent-ai-policy-authority-proof.mjs`;
- `scripts/test/mobile-dormant-ai-provider-proof.mjs`;
- `scripts/test/no-raw-screen-transfer-mesh-proof.mjs`;
- `scripts/test/ai-mesh-event-topology-proof.mjs`.

Expected outputs:

- `output/ai-plan-proof/household-ai-provider-mesh-contract/proof-summary.json`;
- `output/ai-plan-proof/household-mesh-event-bridge/proof-summary.json`;
- `output/ai-plan-proof/household-ai-provider-claim-lease/proof-summary.json`;
- `output/ai-plan-proof/household-ai-provider-result-validation/proof-summary.json`;
- `output/ai-plan-proof/child-agent-ai-policy-authority/proof-summary.json`;
- `output/ai-plan-proof/mobile-dormant-ai-provider/proof-summary.json`;
- `output/ai-plan-proof/no-raw-screen-transfer-mesh/proof-summary.json`;
- `output/ai-plan-proof/ai-mesh-event-topology/proof-summary.json`.

## Test Targets

Unit tests must cover AI work item, claim, lease, result, result validation,
provider advertisement, provider capability, mobile dormant policy, and mesh
transport message contracts.

Rust/service tests must cover AI work event chains, claim/lease/idempotency,
dead-letter behavior, provider selection, result validation authority, mesh
bridge validation, and topology manifests.

Integration tests must cover screen-summary work queuing, trusted desktop
single claim, competing claim rejection, lease expiry requeue, result accepted
before policy, invalid result rejection, mobile dormant while desktop is
available, and no raw screenshot transfer by default.

Security tests must prove providers cannot publish policy or enforcement,
remote peers cannot publish directly into another runtime's bus, wrong-provider
and expired-lease results are rejected, and raw screen payloads are rejected by
default.

## Non-Claims

- No claim that `ocentra-eventing` is a cross-device broker.
- No claim that there is one shared LAN-wide event bus.
- No claim that all local events are broadcast to peers.
- No claim that physical household LAN works until two-device proof exists.
- No claim that mobile AI provider parity exists.
- No claim that mobile processes heavy AI jobs by default.
- No claim that remote/API AI participates in normal child safety.
- No claim that an AI provider has policy authority.
- No claim that an AI provider can enforce.
- No claim that parent portal UI owns business events.
- No claim that raw screenshots transfer to LAN providers by default.
- No claim that provider results are usable before child-agent validation.
- No claim that model quality is product-grade without quality proof.

## Rollout Gates

The mesh cannot be marked product-ready until proof exists for:

- household mesh contracts;
- mesh bridge selected-event export/import;
- provider advertisement and heartbeat;
- claim, lease, duplicate prevention, and dead-letter;
- result validation;
- child-agent-only policy authority;
- no raw screenshot transfer by default;
- mobile dormant/fallback behavior;
- topology/orphan-event safety;
- screen-derived pipeline path through mesh when screen AI uses a household
  provider.
