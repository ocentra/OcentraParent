# Event Taxonomy And Parent Integration

<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Event Taxonomy And Parent Integration`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This document names the Parent event families that should eventually consume the
reusable eventing crate. It is not the implementation source for event constants.
When code starts, Parent event names that cross Rust service or protocol
boundaries must live in `crates/agent-protocol` constants or an explicitly
created protocol/domain crate.

## Reusable Crate Event Types

The generic eventing crate may own only generic internal event types:

```text
eventing.dead_letter.created
eventing.queue.overflowed
eventing.handler.failed
eventing.handler.timed_out
eventing.handler.panicked
eventing.replay.started
eventing.replay.completed
```

These should be constants in the reusable crate. They are not product events.

## Parent Event Namespaces

Parent-specific event namespaces:

```text
system.*
device.*
parent_controller.*
child_agent.*
network.*
browser.*
app_game.*
screen.*
ai.*
policy.*
enforcement.*
audit.*
portal.*
sync.*
lan.*
support.*
```

Every namespace needs a contract owner, schema versioning, roundtrip tests, and
duplicate event-type tests before runtime code publishes it.

`parent_controller.*` and `child_agent.*` are Rust runtime namespaces. They are
not UI namespaces. The Vite/TypeScript portal can send typed parent intents to a
Rust service and render read models, but it cannot own business event chains.

## Event Ordering Matrix

Every Parent event family must define ordering and custody before runtime
publishers land.

| Event Family            | Aggregate Key                           | Causation Required                        | Idempotency Required        | Dispatch Mode                              | Journal Mode                                        | Replay Mode                                        | Allowed Next Event                            |
| ----------------------- | --------------------------------------- | ----------------------------------------- | --------------------------- | ------------------------------------------ | --------------------------------------------------- | -------------------------------------------------- | --------------------------------------------- |
| `network.*`             | device or flow/session key              | yes after raw observation                 | yes for command-like events | ordered by aggregate for state transitions | before or after by contract                         | projection-only by default                         | AI, policy request, audit, portal read model  |
| `ai.*`                  | evidence bundle or analysis request key | yes                                       | yes for model work requests | concurrent unless contract states ordered  | after completion; before for request when auditable | projection-only                                    | policy request, audit, portal read model      |
| `policy.*`              | child/profile/policy chain key          | yes                                       | yes                         | ordered by child/profile/policy key        | before and after decision as contract requires      | projection-only                                    | enforcement command, audit, portal read model |
| `enforcement.command.*` | adapter command key                     | yes; policy decision ref required         | yes                         | ordered by adapter target                  | before dispatch                                     | projection-only unless explicit action replay mode | enforcement result                            |
| `enforcement.result.*`  | adapter command key                     | yes; command ref required                 | yes                         | ordered by adapter target                  | after adapter result                                | projection-only                                    | audit, portal read model                      |
| `audit.*`               | audit chain key                         | yes                                       | yes                         | ordered by audit chain                     | before/after by contract                            | projection-only                                    | portal read model, export/read model          |
| `portal.*`              | read-model or parent-intent key         | parent intent requires service validation | yes for commands            | ordered by entity where stateful           | after read-model update                             | projection-only                                    | policy request or read-model update           |

Workpacks must replace broad rows with exact event-specific rows before
implementation. Enforcement command rows must always prove same-chain policy
decision refs and journal-before-action.

## First Parent Event Chain

The first product chain should be the network-driven safety path, but only after
the reusable bus is implemented and proved.

```text
network.flow.observed
  -> network.domain.observed
  -> network.activity.classified
  -> ai.analysis.requested
  -> ai.analysis.completed
  -> policy.evaluation.requested
  -> policy.decision.completed
  -> enforcement.command.issued
  -> enforcement.result.observed
  -> audit.entry.committed
  -> portal.read_model.updated
```

The chain is intentionally explicit. It prevents shortcuts such as:

```text
AI says bad -> block directly
network sees domain -> block directly
portal button -> adapter command directly
```

## Event Contract Fields

Parent event payloads should include only fields owned by the event contract.
The envelope already carries cross-cutting event metadata.

Required envelope-level metadata:

- event id;
- event type;
- schema version;
- occurred at;
- published at;
- correlation id;
- causation id when applicable;
- aggregate key when ordering matters;
- idempotency key when duplicate commands must be rejected;
- source;
- custody;
- priority;
- deadline;
- target handler when applicable.

Required Parent evidence/action refs:

- evidence refs for evidence-derived events;
- policy decision ref before enforcement command events;
- adapter capability proof ref before adapter command events;
- audit ref target for action events;
- rollback, expiry, or unavailable state for enforcement commands;
- uncertainty codes where evidence can overclaim.

## Namespace Sketch

### Parent Controller

```text
parent_controller.parent_action.received
parent_controller.command.validated
parent_controller.command.rejected
parent_controller.child_command.forward_requested
parent_controller.child_command.forwarded
parent_controller.read_model.projected
```

Parent-controller events are Rust-service events. They represent validated
parent intents, controller-side orchestration, and read-model projection. They
do not live in Vite/TypeScript UI code.

### Child Agent

```text
child_agent.command.received
child_agent.command.accepted
child_agent.command.rejected
child_agent.capability_state.updated
child_agent.runtime_health.updated
```

Child-agent events are Rust-service events. They represent child-device runtime
state and validated child-agent command boundaries.

### Network

```text
network.flow.observed
network.domain.observed
network.flow.summary.updated
network.activity.classified
network.bypass.candidate_detected
network.restricted_destination.detected
network.evidence_bundle.created
```

Network events must preserve the network plan boundaries:

- no exact HTTPS URL from network-only evidence;
- no decrypted payload;
- no message, search, or page-content claim;
- evidence grade and uncertainty are required.

### AI

```text
ai.analysis.requested
ai.analysis.completed
ai.audit_report.completed
ai.manual_review.required
ai.provider.unavailable
```

AI events must cite evidence refs and cannot issue enforcement commands.

The household AI provider mesh expands the broad `ai.*` namespace into exact
families before implementation.

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

Required household mesh event-family rows:

| Event Family      | Aggregate Key                          | Causation Required                              | Idempotency Required                         | Dispatch Mode                       | Journal Mode                           | Replay Mode     | Allowed Next Event                                             |
| ----------------- | -------------------------------------- | ----------------------------------------------- | -------------------------------------------- | ----------------------------------- | -------------------------------------- | --------------- | -------------------------------------------------------------- |
| `ai.provider.*`   | provider peer id or physical device id | yes for updates after discovery                 | yes for heartbeat/capability update          | latest-state projection by provider | after validation, selected events only | projection-only | `ai.work.route_selection.requested`, portal, audit             |
| `ai.work.*`       | AI work job id                         | yes from evidence bundle or parent/manual input | yes using dedupe key                         | aggregate-ordered for same job      | before claim and after terminal states | projection-only | `ai.work.claim.*`, `ai.result.*`, audit, portal                |
| `ai.work.claim.*` | AI work job id                         | yes from queued or route-selected work          | yes using job id, provider peer id, claim id | aggregate-ordered                   | before lease grant/reject              | projection-only | `ai.work.lease.*`, `ai.work.started`, audit, portal            |
| `ai.work.lease.*` | AI work job id                         | yes from claim grant or renewal request         | yes using job id, claim id, lease owner      | aggregate-ordered                   | before provider execution or requeue   | projection-only | `ai.work.started`, `ai.work.requeued`, `ai.work.dead_lettered` |
| `ai.result.*`     | AI work job id                         | yes from work completion or LAN result received | yes using job id, claim id, result id        | aggregate-ordered                   | before policy request                  | projection-only | `policy.evaluation.requested`, audit, portal                   |

### Policy

```text
policy.evaluation.requested
policy.decision.completed
policy.manual_review.required
policy.rule_conflict.detected
policy.decision.expired
```

Policy decision events are the first point where an action can be authorized.
They still do not execute platform behavior.

### Enforcement

```text
enforcement.command.issued
enforcement.command.accepted
enforcement.command.rejected
enforcement.command.failed
enforcement.result.observed
enforcement.rollback.requested
enforcement.rollback.completed
```

Enforcement command events require policy decision refs and adapter capability
proof. Adapter result events must include applied, rejected, unavailable,
rollback-needed, rollback-completed, failed, dry-run, or manual-required state.

### Audit

```text
audit.entry.append_requested
audit.entry.committed
audit.entry.failed
audit.replay.requested
audit.replay.completed
```

Audit events should make the chain reviewable without reading runtime logs.

### Portal

```text
portal.parent_action.received
portal.read_model.updated
portal.capability_state.updated
portal.manual_required.visible
```

Portal events represent validated local-service commands or read-model updates.
The UI must not publish adapter commands directly.

## Parent Command Flow

Parent action from portal:

```text
parent clicks allow/block/ask
  -> local API validates request
  -> parent_controller.parent_action.received
  -> policy.evaluation.requested
  -> policy.decision.completed
  -> enforcement.command.issued only if allowed
  -> enforcement.result.observed
  -> audit.entry.committed
  -> portal.read_model.updated
```

The UI step above is input only. The first business event is published by Rust
after local-service validation.

Network-triggered action:

```text
network activity classified
  -> evidence bundle created
  -> AI analysis requested if needed
  -> policy evaluation requested
  -> manual-required, ask, warn, limit, block, or observe decision
  -> adapter command only if proof exists
```

## Aggregate Keys

Use aggregate keys for ordered state transitions:

- child profile;
- device;
- policy decision;
- enforcement command;
- evidence bundle;
- audit chain;
- LAN peer.

Do not parallelize ordered state transitions for speed.

## Idempotency Keys

Use idempotency keys for:

- enforcement commands;
- rollback commands;
- parent approval actions;
- journal append requests where duplicate writes could mislead audit;
- replay projections.

Duplicate commands should return an idempotent report, not execute twice.

## Journal Policy By Event Family

| Event Family               | Journal Mode                                   | Reason                                        |
| -------------------------- | ---------------------------------------------- | --------------------------------------------- |
| Evidence observed          | Before or after dispatch, depending on source. | Preserve observed facts and custody.          |
| AI requested/completed     | Before and after where product uses AI output. | Prove AI input/output references.             |
| Policy requested/completed | Before and after.                              | Prove decision authority.                     |
| Enforcement command        | Before dispatch.                               | Never act before an auditable command exists. |
| Enforcement result         | After adapter result.                          | Prove what happened.                          |
| Audit committed            | After journal append.                          | Prove audit persistence.                      |
| Portal read model          | After projection.                              | UI is a view over service state.              |

## Network Plan Dependency

Network implementation order:

1. Implement and prove reusable `ocentra-eventing`.
2. Add Parent protocol constants and event payload contracts for network, AI,
   policy, enforcement, audit, and portal events.
3. Add network event publishers and subscribers.
4. Add journal/projector proof.
5. Only then wire network cascade and enforcement workpacks.

This prevents network from creating a private bus that cannot be reused by the
rest of Parent or other Ocentra projects.
