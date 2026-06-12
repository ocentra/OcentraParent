<!-- agent-capsule -->

> Agent Capsule
> Doc: Ocentra Parent Product Constitution
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Ocentra Parent Product Constitution

This document is the product-level constitution for Ocentra Parent. It is
written for anyone deciding what the product is, why it exists, and what must be
true before a feature can be called complete.

The implementation roadmap and expectation files are subordinate to this
constitution. If a roadmap item or checkpoint conflicts with this document,
update the roadmap or checkpoint instead of weakening the product contract.

## Product Promise

Ocentra Parent gives parents local-first, evidence-backed control over a child's
digital life without turning Ocentra into the family's data warehouse.

The product is a real child-device system, not a dashboard-only service:

- The child device runs a local Rust agent.
- Evidence is captured, journaled, queried, classified, and acted on locally by
  default.
- Parent rules, schedules, approvals, and overrides are typed and auditable.
- Local AI helps parents understand activity and set rules, but it does not get
  hidden authority to enforce policy.
- The parent portal is a control and visibility surface. It is not the source of
  truth for child-device capture, timers, policy, AI safety decisions, or
  enforcement.

## Why Ocentra

Parents should choose Ocentra when they want more than simple screen-time
settings or cloud-controlled category blocks.

Ocentra's intended difference:

- Local-first child safety: child activity evidence, AI decisions, and policy
  enforcement stay on the child device by default.
- Parent-owned data: reports and exports go to the parent device or
  parent-selected storage unless the parent explicitly enables a remote path.
- AI that works like an operator: a parent can ask the assistant to create a
  schedule, explain a decision, draft a rule, preview an action, or tune a
  policy without learning every setting first.
- Context-aware controls: rules can use browser URL evidence, app/game sessions,
  network summaries, screen-analysis summaries, schedules, time budgets, and
  parent approvals instead of only a global app block.
- Social and video intent: Ocentra should reason over the actual available
  evidence for a URL, video, visible page, social app, or interaction context
  instead of trusting a single platform rating when richer local evidence exists.
- Honest capability status: unsupported, unavailable, degraded, stale,
  manual-required, and scaffold-only states are visible instead of hidden.

## Product Truths

- Ocentra is not a cloud surveillance product.
- Ocentra-hosted services do not store raw child activity evidence by default.
- Parent rules are household decisions, not hidden Ocentra moral judgments.
- Platform support is only claimed when the platform path is proved.
- Web and portal surfaces author intents and show state; they do not run
  child-device enforcement.
- AI output is evidence for policy. It is not direct authority.
- Every sensitive capability needs a visible setting, reason, audit record, and
  degraded-state story.
- Every feature needs a status: `done`, `in progress`, `planned`,
  `manual-required`, `scaffold-only`, `degraded`, `blocked`, or `not started`.

## Status Vocabulary

Use these terms consistently in README files, roadmap entries, expectation docs,
worker reports, and PR bodies.

| Status            | Meaning                                                                                                                            |
| ----------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `done`            | Runtime behavior exists, uses real contracts/state, is validated, and the product claim is true for the named platform/scope.      |
| `in progress`     | A real slice exists, but some required runtime, UI, platform, or validation proof remains.                                         |
| `planned`         | The product requirement is accepted, but implementation has not started or is only represented by roadmap text.                    |
| `manual-required` | CI or automated tests cannot prove the OS/device behavior; a named manual proof artifact is required before the claim is complete. |
| `scaffold-only`   | Package, app shell, contract, or CI mechanics exist, but the user-visible product behavior is not implemented.                     |
| `degraded`        | The feature has a defined fallback or unavailable state that is surfaced to the parent.                                            |
| `blocked`         | Work cannot proceed until a named dependency, entitlement, credential, permission, device, or design decision exists.              |
| `not started`     | No meaningful product work exists beyond idea tracking.                                                                            |

## Required Product Surfaces

The product cannot be considered coherent until these surfaces are represented
in the roadmap, expectation docs, feature checklist, and README links.

| Surface            | Product requirement                                                                                                                         |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------- |
| Family setup       | Parent account, household, child profile, device role, co-parent/observer state, recovery, and migration.                                   |
| Child device agent | Local service, device identity, capture, journal, query store, local AI/policy, timers, enforcement, audit, and capability status.          |
| Parent portal      | Clear setup, device health, activity, policy, AI, reports, alerts, approvals, source/custody labels, and degraded states.                   |
| Evidence           | Browser, app/game, network, screen, social/video/location where applicable, with source ids, custody labels, and confidence.                |
| Policy             | Rules, schedules, time budgets, categories, permissions, ask-parent, overrides, conflict resolution, dry-run preview, and audit.            |
| Enforcement        | Real platform adapters, capability status, rollback, timer recovery, child-facing explanation, and parent-visible result.                   |
| AI                 | Child-safety classifier/evaluator, parent assistant, provider scheduler, local runtime status, degraded behavior, and action previews.      |
| Remote access      | LAN first, optional relay, parent-owned storage, stale/offline states, revocation, route status, and no default Ocentra child-data custody. |
| Notifications      | Minimal-detail alerts, authenticated drill-in, delivery status, quiet hours, escalation, and privacy-preserving payloads.                   |
| Reports            | Parent-owned report generation, evidence references, trend summaries, action history, and assistant Q&A over cited evidence.                |
| Mobile             | Separate parent-mobile and child-agent claims for Android and iOS, each backed by platform-specific proof.                                  |
| Subscription       | Billing, entitlement, device limits, grace states, and safety behavior during billing failure.                                              |
| Production         | Signed installers, store paths, update channels, support docs, privacy/legal docs, dependency/security gates, and release evidence.         |

## Feature Documentation Contract

Every feature needs a feature row in
[product-capability-checklist.md](product-capability-checklist.md) and a
feature document linked from [feature-list.md](feature-list.md). If the feature
is large or product-critical, it also needs its own expectation file under
`docs/expectations/`.

Every feature row must name:

- parent-facing outcome;
- current status;
- owning expectation docs;
- current proof or checkpoint;
- missing proof or next slice;
- platform scope;
- data custody boundary;
- whether AI, enforcement, remote access, or mobile claims are involved.

## Module Documentation Contract

Every app, package, crate, and platform area needs a README that explains:

- what the module owns;
- what must not go there;
- how it connects to neighboring modules;
- relevant expectation docs;
- current implementation status;
- known gaps and next proof.

Source files remain the truth for implementation. Module READMEs are the map.

## Product Claim Gate

A product claim is not allowed in user-facing docs unless the repo can point to:

1. a roadmap milestone;
2. relevant expectation docs;
3. domain/protocol contracts where data crosses module boundaries;
4. runtime implementation or explicit scaffold-only state;
5. validation command or manual proof artifact;
6. module README ownership where the behavior lives;
7. current status in the product capability checklist.

## Competitor Parity Rule

If Google Family Link, Apple Screen Time, Microsoft Family Safety, Bark,
Qustodio, Norton Family, Net Nanny, Canopy, Kidslox, FamilyTime, FamiSafe, or a
similar serious competitor exposes a parent-facing capability, Ocentra must do
one of three things:

- implement it;
- deliberately reject it with a product/privacy/platform rationale;
- track it as a gap with owner, expectation docs, and roadmap placement.

See [competitor-capability-map.md](competitor-capability-map.md).

## Non-Negotiable Boundaries

- Do not claim app blocking until the platform adapter actually blocks or
  terminates the named app/process under a typed policy decision.
- Do not claim exact browser URL knowledge from process/window or network
  metadata alone.
- Do not claim social/message monitoring until the source, scope, platform
  permission, privacy boundary, and parent-visible alert contract are explicit.
- Do not claim video analysis as product-complete until the input source,
  local/runtime model path, confidence handling, policy action, and audit
  evidence are proved.
- Do not claim Android child-agent parity from APK scaffolding.
- Do not claim iOS child-agent parity without Family Controls, DeviceActivity,
  Network Extension or equivalent approved API proof where those capabilities
  are required.
- Do not claim remote access unless route status, auth, revocation, stale state,
  custody, and relay behavior are explicit.
- Do not claim production readiness without installer/store/signing/update,
  support, privacy/legal, and validation evidence.

## Required Reading Order

For product planning:

1. This constitution.
2. [README.md](../README.md) for the product-facing summary.
3. [product-roadmap.md](product-roadmap.md) for milestone order.
4. [product-capability-checklist.md](product-capability-checklist.md) for
   feature status.
5. [feature-list.md](feature-list.md) for per-feature docs.
6. [feature-expectations.md](feature-expectations.md) for expectation docs.
7. Relevant module README files under `apps/`, `packages/`, `crates/`, and
   `platforms/`.
