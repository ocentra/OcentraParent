<!-- agent-capsule -->

> Agent Capsule
> Doc: LAN Pairing Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# LAN Pairing Expectations

LAN features expose the child-device agent beyond loopback and must be treated as trust-boundary work.

## Parent Outcome

A parent can use a trusted device on the same local network to find, pair with, select, query, and configure a child-device agent without needing cloud availability. The parent should understand which child device is selected, whether the link is local/LAN, and whether a command was accepted, rejected, or waiting for pairing.

## Child-Device Outcome

The child-device agent stays the execution authority. It accepts only schema-valid rule, query, and approval intents from a paired parent device, records pairing and control events for audit, and rejects anonymous or incorrectly routed LAN requests.

For household AI provider mesh work, the child-device agent also stays AI work
authority and policy authority. Trusted household providers may claim bounded AI
work only after the child agent grants a lease, and they return worker results
only. They cannot publish policy decisions, enforcement commands, or child
configuration updates.

## Platform Scope

- Windows is the first required implementation target because the first local product is Windows-first.
- Other desktop or mobile platforms may reuse the pairing contracts only after their local agent transport, device identity, and package permissions are proven.
- Browser-only web surfaces are parent portals. They do not become child-device agents because a browser can reach a LAN URL.

## Data Scope

LAN pairing may exchange device identity, pairing proof, parent-device identity, selected child-device id, route id, intent id, command status, and minimal health/query payloads. It must not expose raw journals, SQLite files, local filesystem paths, decrypted evidence blobs, or unrelated device telemetry through pairing endpoints.

Household AI provider messages may exchange provider advertisement, heartbeat,
capability, AI work claim, lease, bounded payload transfer, and result-return
messages. They must not expose a shared LAN-wide event bus, allow direct remote
publish into another runtime's local event bus, or transfer raw screenshots by
default.

## Trust Boundary

Loopback remains the default. LAN mode is explicit and must require both network exposure enablement and a pairing proof. Pairing proof material must be scoped to a device relationship, not a broad LAN admin credential. Origin checks, route checks, and intent validation stay active after pairing.

## Contract Boundary

Pairing contracts belong in shared domain packages before runtime code consumes them. Expected contract families include device discovery, explicit production discovery state, pairing challenge/proof, trusted-device registry entry, selected route recovery state, route target, parent intent envelope, child-agent response, rejection reason, LAN AI provider routing state, custody label, and audit event. Rust protocol shapes must mirror the TypeScript contracts before the Rust service accepts or emits the payloads.

Household AI provider mesh contracts must add provider advertisement/heartbeat,
provider capability snapshot, AI work item, claim decision, lease, result,
result validation, dead-letter, and mesh transport envelope shapes before the
Rust service accepts or emits them.

## Failure Behavior

- Unpaired LAN callers receive an explicit rejection reason and no control surface.
- Expired, replayed, malformed, wrong-device, or wrong-origin pairing proofs are rejected and audited.
- If LAN discovery fails, direct local address entry may be allowed only through the same pairing and origin checks.
- If a paired device is unavailable, the portal shows offline or stale status instead of silently falling back to another child device.
- Pairing revocation takes effect before any new rule, query, or approval intent is accepted.
- If proof is local-only, the evidence must label real household two-device LAN
  discovery as `manual-required` instead of treating local sibling services as
  physical-device proof.

## Expected Deliverables

- Explicit LAN enablement.
- Origin allowlist.
- Pairing proof contract.
- Trusted device registry.
- Device identity display.
- Parent Devices route renders service-backed local-agent identity, host CPU/GPU
  and memory details, and observed LAN neighbor IP/MAC/interface data without
  inventing hardware details for devices that have not reported through an
  agent.
- Parent Devices route renders a service-backed scan summary that counts
  child-agent, passive LAN, and infrastructure/router evidence separately.
- If the local child agent also appears in passive LAN evidence, it is merged
  into one device row with agent and LAN badges rather than duplicated as an
  IP-only neighbor.
- Multi-device rule/query/approval routing.
- Pairing revocation path.
- Pairing audit events in the local evidence pipeline.
- Parent-visible selected-device state.

## Acceptance

- Anonymous LAN control is rejected.
- Paired portals can send only typed rule, query, and approval intents; execution remains agent-side.
- Loopback remains the default.
- Pairing state is auditable.
- Portal can distinguish devices.
- Tests cover rejected and accepted routes.
- Parent-visible UI makes the selected device clear.
- Parent-visible UI distinguishes the local Rust agent from LAN neighbors and
  shows `Not reported` for CPU/GPU/memory fields that are not agent-backed.
- Parent-visible UI keeps passive LAN neighbors and routers out of controllable
  Policy/Activity/Parent Portal target lists until an agent-backed pairing path
  exists.
- The same command sent to the wrong paired device is rejected rather than applied to the currently selected device by accident.
- Pairing state survives service restart through an explicit local registry or produces a safe unpaired state.
- Sensitive child activity details are not included in discovery beacons or pairing challenge previews.

## Validation Gates

- TypeScript contract tests for valid and invalid pairing payloads.
- Rust protocol parity tests for pairing, route, rejection, and audit shapes.
- Local service integration tests for anonymous rejection, successful pairing, wrong-origin rejection, wrong-device rejection, revocation, and service restart behavior.
- Portal Playwright coverage for discovery or direct-address entry, selected-device display, accepted route, rejected route, and offline/stale state.
- Security/static-analysis gates because LAN exposure and device identity are security-sensitive.
- Local multi-service proof through
  `node scripts/test/v0-9-production-lan-multidevice-hardening.mjs`.
- Manual two-device proof artifacts for router/firewall reachability, distinct
  parent/child hosts, service logs, and generated proof JSON before production
  household LAN readiness is claimed.

## Non-Goals

- Do not treat LAN as production auth.
- Do not expose broad unauthenticated control APIs.
- Do not skip origin checks because a workflow is dev-only.

## Done Signal

A parent portal can discover or pair with a local agent only through explicit trusted flow, and the agent rejects anonymous or incorrectly routed LAN rule, query, and approval intents.
