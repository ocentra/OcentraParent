<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-plan`
> Doc: `AGENTS.md`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX or FEATURE_ROUTE_INDEX selects the plan.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Route changes require PLAN_STATE.md, ROUTE_INDEX.md, WORKPACK_INDEX.md, PLAN_INDEX.md, and FEATURE_ROUTE_INDEX.md to stay aligned.

<!-- /agent-capsule -->

# Native Apps Plan Agent Route

Use this plan for native app identity, installed inventory, process/runtime, foreground app evidence, app-only policy targets, app catalog/settings, and legacy app-plan reconciliation.

## High-Density Execution Contract

Task: work only the assigned slice for this plan.
Context: [PLAN_STATE.md](PLAN_STATE.md) is current state; [WORKPACK_INDEX.md](WORKPACK_INDEX.md) chooses one workpack; [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) defines required local proof.
Scope rule: one plan, one workpack, exact checklist/proof rows. Adjacent plans are closed until the selected workpack names a handoff.
Implementation rule: docs define expected outcome, boundary, shape, validation, and proof. They do not prescribe implementation code.
Proof rule: proof must include command/log evidence, negative cases, artifact paths, updated rows, and skipped-risk notes when applicable.
Failure condition: no DONE/PR_READY when expected proof is missing, only happy-path evidence exists, or this plan is used to claim adjacent implementation completion.

## Ownership, Import, And Boundary Contract

Native Apps is the app-only narrowing and reconciliation plan. It does not own the full shared native app/game evidence spine; that remains in `app-game-plan` unless the selected workpack explicitly narrows an app-only slice here.

Module roles:

```text
crates/schema: canonical shared native-app/app-game schema, brands, parser, source-readiness, policy target, platform proof, and handoff shapes when those shapes cross package, crate, app, or plan boundaries.
app-core: child-local Rust native-app observation, evidence event, AI-request event, and policy-request event boundary. It should use event/protocol handoffs rather than importing sibling runtime crates.
app-plan: app-only route, reconciliation, app-specific source-readiness, and proof expectation owner.
app-game-plan: shared native app/game evidence spine, combined runtime/read-model proof, native game slices, and most generated handoff chains.
agent-protocol and agent-service: wire/service/read-model boundaries only when the selected workpack names protocol, service handler, or read API proof.
portal-domain and apps/portal: parent-visible projection only; they do not observe OS state, classify apps, run timers, or enforce.
policy, enforcement, notification, child-runtime, setup, payment, data-custody, LAN, and remote plans: sibling owners or handoff consumers. They must not re-own native-app source truth.
```

Direct imports are allowed only for neutral/shared infrastructure or explicit public helper surfaces:

```text
canonical crates/schema app/app-game/evidence/policy-reference/protocol/capability/logging shapes
neutral event/evidence/logging/protocol primitives
approved public helper exports named by the selected workpack
app-core when the selected workpack names Rust app observation/event proof
pure common helpers that do not own feature behavior or side effects
```

Forbidden direct imports:

```text
sibling feature owner runtime behavior from app-game, AI, policy, enforcement, notification, portal, child-runtime, setup, payment, data-custody, LAN, or remote plans
private source files from another plan's owning package/crate
peer feature contracts when the shared shape should live in schema-domain or another neutral boundary
portal, policy, AI, enforcement, or notification code that scans native-app source state instead of consuming app evidence/read models
policy or enforcement internals that execute app actions without typed native-app source readiness, authority, and adapter-readiness proof
```

If native-app work needs app-game, AI, policy, enforcement, notification, portal, child-runtime, LAN, or remote behavior, it must use typed evidence, commands, events, requests, read models, and proof handoffs. If a shape is used by multiple feature owners, place or consume it through `crates/schema` or another neutral shared boundary. Do not solve cross-plan behavior by importing another feature's runtime internals.

## Local Decision Tree

- If the assignment names a workpack, open only that workpack.
- If the assignment names a checklist row but no workpack, use [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md), then choose one workpack from [WORKPACK_INDEX.md](WORKPACK_INDEX.md).
- If the assignment changes product status, read [DOC_INDEX.md](DOC_INDEX.md), [PLAN_STATE.md](PLAN_STATE.md), and [PROOF_INDEX.md](PROOF_INDEX.md) only for named rows.
- If the assignment touches adjacent implementation ownership, open only the adjacent plan named by the selected workpack.
- If the assignment is DONE/PR_READY, read [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md), [PROOF_INDEX.md](PROOF_INDEX.md), [PLAN_HEALTH.md](PLAN_HEALTH.md), then [../../agent/PR_DONE_FLOW.md](../../agent/PR_DONE_FLOW.md).

## Required Read Order

1. [PLAN_STATE.md](PLAN_STATE.md)
2. [NEXT_ACTIONS.md](NEXT_ACTIONS.md)
3. [WORKPACK_INDEX.md](WORKPACK_INDEX.md)
4. [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when the selected workpack owner path is unclear
5. One assigned workpack under workpacks/
6. [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md)
7. [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md) and [PROOF_INDEX.md](PROOF_INDEX.md) only for named rows/artifacts

## Product Sources

- Feature docs: child-agent-local-service.md, policy-schedules-approvals.md, app-game-control.md
- Expectation docs: platforms.md, policy.md
- Adjacent plans: app-game-plan, v0-8-enforcement-control-plan, portal-ux-household-surfaces-plan
