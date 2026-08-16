<!-- agent-capsule -->

> Agent Capsule
> Plan: `remote-access-plan`
> Doc: `Remote Access Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Remote Access Plan Test Proof Expectations

## Proof root

```text
output/remote-access-plan-proof/<workpack-file-stem>/
```

## Common commands

Use the subset relevant to the selected workpack:

```bash
npm run build --workspace @ocentra-parent/screen-domain
npm run test --workspace @ocentra-parent/screen-domain
cargo test -p ocentra-parent-agent-protocol remote
cargo test -p ocentra-parent-agent-service remote
npm run test --workspace @ocentra-parent/portal -- remote
npm run lint:architecture -- --files packages/screen-domain packages/agent-protocol-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/remote-access-plan
```

Run through `npm run agent:run --` when collecting proof if the wrapper is available.

## Command ownership notes

- `remote-access-plan` owns remote capability, standing access, pairing/grant, relay semantics, abuse controls, and proof routing.
- `screen-plan` owns capture primitives and protected-surface behavior; screen proof is not remote access proof.
- `lan-plan` owns LAN-only transport and local pairing; LAN proof is not relay-backed remote proof.
- `account-identity-family-plan` and `device-trust-bootstrap-plan` own actor/session/device authority and step-up.
- `data-custody-storage-plan` owns remote artifact/diagnostic retention, export, deletion, and privacy boundaries.
- `portal-ux-household-surfaces-plan` owns rendered remote state only.
- WP03 remote control is deferred; do not run or claim control tests in the current live-view pass unless explicitly assigned.

## Remote E2E meaning

Do not use one proof family to claim the whole remote-access path. For this plan, E2E has separate meanings:

```text
capability/grant E2E: actor/household/device/capability -> scoped grant -> no generic remote flag.
live-view relay E2E: paired grant -> relay session -> live view/degraded/protected-surface state -> no control claim.
pairing/standing-access E2E: initial pairing -> standing access -> visible grant state until revoke/remove-device.
revocation/remove-device E2E: revoke or remove device -> reconnect/stale/cache denied -> audit proof.
relay abuse/security E2E: token/session -> rate limit/backpressure/replay/cross-household denial -> redacted diagnostics.
custody/retention E2E: remote artifact/diagnostic -> retention/export/delete policy -> no raw payload default.
portal disclosure E2E: remote state -> parent/child visible disclosure/degraded/manual-stop state -> no runtime proof claim.
deferred-control no-claim E2E: live-view path -> remote input/control disabled/deferred boundary.
rollout gate E2E: accepted proof roots + carried blockers -> allowed/blocked claims -> no-overclaim boundary.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## Structured harness logging expectations

Product/runtime-safe logging:

```text
redact raw screen frames, input stream payloads, child-private payloads, account/session secrets, relay tokens, diagnostic payload bodies, and support-private notes
log workpack, capability type, actor role, household/device refs, pairing state, grant state, relay state, revocation/removal state, custody state, child disclosure state, abuse state, manual-required note, and no-claim boundary when safe
separate capture source, LAN transport, remote relay, account authority, device trust, custody, portal projection, live view, deferred control, and support/admin states
never treat local capture logs, LAN logs, relay route existence, UI-only logs, or happy-path live-view logs as product readiness without selected proof root and no-claim boundary
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, capability type, actor role, device ref, exit code, result, artifact pointer, diagnostics summary, manual-required note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## Required negative states

```text
expired grant denied
revoked grant denied
removed device denied
wrong household denied
wrong role denied
missing device-trust handoff blocked
transport unavailable visible
manual stop visible
private payload not exposed by default
UI-only proof not product proof
local screen proof not remote access proof
LAN pairing proof not relay-backed remote proof
live-view proof not remote control proof
support/admin hidden access denied
```
