# 37 Household Mesh Screen Analysis Queue

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `37 Household Mesh Screen Analysis Queue`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Target State

Heavier analysis moves to a local trusted household mesh provider before any
remote/API path.

## MVP Boundary

This is AI-pass and architecture-alignment work. Capture MVP should expose route
state for household-provider-required cases.

## Current production-code truth (2026-08-24)

The current Rust mesh path is a contract/proof-shaped boundary, not a shipped
household-provider runtime. `ScreenHouseholdMeshInput` is a public,
caller-constructible serde DTO with arbitrary string fields, and
`proof_fixture()` supplies synthetic input. The publisher builds
`ScreenHouseholdMeshSpine::without_owner_handlers()` over an in-memory event
bus and returns in-memory journal/dead-letter state. These paths must not be
treated as provider, claim, lease, result, or policy authority.

The real service path is also unavailable: `agent-service` starts the existing
screen cadence/foreground/event runtime, but does not compose a household-mesh
producer or consumer. Its row-ready path records `RuntimeOwnerUnavailable`
and returns the manual-required error. The current lease store keeps
process-local state and synthesizes claim/lease identifiers from the job ID;
provider role/read-model, heartbeat/capability, and environment availability
are routing hints, not authenticated durable provider authority.

Signed LAN ingress verification and SQLite ingress custody are real prerequisite
boundaries, but no service caller connects them to this screen queue. No source
completion, provider transfer, result receipt, restart/replay path, test, proof,
READY, or DONE claim is made here.

### Indexed owner map and missing composition

| Required capability | Indexed owner workpacks | Current truth |
| --- | --- | --- |
| Authenticated durable household-provider selection | `WP-ai-plan-08-ai-provider-routing-contract`; runtime authority and selected-route/custody composition: `WP-lan-plan-26-signed-child-beacon-ingress-and-household-mesh-authority-handoff` | AI WP08 owns the route contract; LAN WP26 is the blocked runtime authority/route owner. Neither currently supplies a shipped authenticated provider selection for WP37. |
| Durable claim/lease/replay custody | `WP-ai-plan-07-ai-job-queue-contract`; LAN ingress/idempotency custody: `WP-lan-plan-26-signed-child-beacon-ingress-and-household-mesh-authority-handoff`; local republish boundary: `WP-eventing-plan-10-lan-household-mesh-consumer` | These are separate contract, LAN custody, and Eventing-local boundaries. No connected cross-device screen AI claim/lease/replay runtime exists. |
| Provider result receipt and ownership | `WP-ai-plan-07-ai-job-queue-contract`; child-owned result acceptance boundary: `WP-ai-plan-32-family-ai-hub-and-remote-assistant-boundary`; journal/read-model sink: `WP-ai-plan-19-ai-result-journal-sqlite-ingest` | The indexed owners define contracts/boundaries only; no provider-issued receipt, owner acknowledgement, or screen result composition is shipped. |
| Service startup composition | Generic parent integration: `WP-eventing-plan-08-parent-runtime-integration`; local mesh consumer: `WP-eventing-plan-10-lan-household-mesh-consumer` | No indexed owner currently supplies the complete screen-specific `agent-service` startup producer/consumer composition. This is an explicit missing-owner blocker, not a dependency to fabricate. |

WP37 therefore remains **blocked/source-incomplete** until the named owner
boundaries provide real authenticated authority, durable custody, and a
service-owned composition. Public arbitrary inputs, in-memory owner-less buses,
synthesized IDs, heartbeat/environment state, or LAN evidence cannot unlock it.

## Checklist

- [ ] Define trusted household provider availability state.
- [ ] Define local-network custody boundary.
- [ ] Define summary/image transfer rules if any.
- [ ] Prefer redacted/cropped input.
- [ ] Record parent approval requirements.
- [ ] Add fallback to manual-required when no trusted household provider is
      available.

## Proof

- Household mesh provider route contract.
- Custody and no-remote-default proof.

Proof command:

```powershell
node scripts/test/screen-family-ai-hub-routing-proof.mjs
```

Proof artifact:

```text
output/screen-plan-proof/37-family-ai-hub-screen-analysis-queue/proof-summary.json
```

## Non-Claims

- No real LAN household mesh runtime, discovery protocol, or relay is
  implemented by this screen-plan proof.
- No production OCR/VLM model quality is claimed.
- No remote/API child-safety route, policy decision, portal UI, or enforcement
  adapter is claimed.
