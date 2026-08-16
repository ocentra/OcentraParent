<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `Screen Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Screen Plan Test Proof Expectations

## Proof root

```text
output/screen-plan-proof/<workpack-file-stem>/
```

Historical retained artifacts may use named subdirectories under `output/screen-plan-proof/`; the selected workpack must name the accepted artifact path before any row is checked.

## Common commands

Use the subset relevant to the selected workpack:

```bash
npm run build --workspace @ocentra-parent/screen-domain
npm run test --workspace @ocentra-parent/screen-domain
cargo test -p ocentra-parent-agent-protocol screen
cargo test -p ocentra-parent-agent-service screen
npm run test --workspace @ocentra-parent/portal -- screen
npm run lint:architecture -- --files packages/screen-domain packages/agent-protocol-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/screen-plan
```

Run through `npm run agent:run --` when collecting proof if the wrapper is available.

## Command ownership notes

- `screen-plan` owns local screen capture/evidence/custody/settings/live-view-boundary proof.
- `screen-domain` owns public screen contracts, screen intelligence, disclosure, routing, and handoff guard surfaces.
- `screen-ai-pipeline-plan` owns screen -> AI -> policy/action product-path proof.
- `ai-plan/schema-domain` owns shared AI context/result/provider/degradation behavior and contracts.
- `policy-control-plane-plan` owns policy authority and parent-rule precedence.
- `v0-8-enforcement-control-plan` owns enforcement adapter execution and rollback.
- `data-custody-storage-plan` owns product retention/export/delete/privacy policy.
- `portal-ux-household-surfaces-plan` owns broader portal UX completion.
- `remote-access-plan` owns relay-backed remote live-access/session authority.

## Screen E2E meaning

Do not use one proof family to claim the whole screen path. For this plan, E2E has separate meanings:

```text
contract/status E2E: schema/contract -> malformed payload rejection -> no runtime platform proof.
platform-capture E2E: permission/probe -> selected display/window/app capture -> protected/degraded state -> queue/deletion; no AI or cross-platform claim.
trigger-scope E2E: source trigger/scope -> capture job or structured skip -> custody state; no domain source-truth claim.
queue-scheduler E2E: encrypted temp image queue -> debounce/backpressure -> deletion/expiry/failure states; no AI/policy claim.
analysis-result E2E: screen summary/result schema -> redaction/confidence/evidence refs -> validator rejection; no model-quality claim unless selected.
journal/read-model E2E: screen evidence row -> service/read model -> portal visible state; no raw capture or portal UX completion claim.
deletion/retention E2E: raw image path -> delete success/TTL/delete-failed visibility -> raw retention disabled or opt-in proof.
policy-evidence E2E: validated summary/evidence refs -> policy target/dry-run handoff -> no enforcement claim.
live-view E2E: optional live-view setting/preflight/loopback/relay-cache/worker gate -> no product-ready claim without prompt/physical/privacy proof.
remote-boundary E2E: redacted summary export -> parent approval/audit/custody -> raw screenshot remote upload denied.
rollout E2E: accepted proof roots + open workpack blockers + known gaps -> allowed/blocked claims.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## Structured harness logging expectations

Product/runtime-safe logging:

```text
redact raw images, raw screenshot paths, OCR text unless fixture-scoped, VLM prompt/output unless fixture-scoped, child-private text, account/session secrets, remote/export payloads, and support-private diagnostics
log workpack, platform, capture scope, trigger type, permission state, protected-surface state, queue state, deletion state, retention mode, live-view state, custody state, portal state, proof tier, artifact pointer, blocker, and no-claim boundary when safe
separate capture source, queue/custody, AI analysis, policy handoff, enforcement, portal projection, live-view, remote/export, and legal/privacy states
never treat mock screenshots, fixture-only proof, checked rows, portal screenshots, capture-only proof, or live-view preflight proof as product readiness without selected proof root and no-claim boundary
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, platform, proof tier, exit code, result, artifact pointer, diagnostics summary, blocker note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## Required negative states

```text
permission missing visible
unsupported platform visible
capture disabled visible
protected surface blocked or degraded visible
image deleted/expired state visible
delete-failed state visible where selected
private raw image not exposed by default
raw screenshot remote upload denied by default
mock screenshot not product proof
portal screenshot not runtime proof
capture-only proof not AI/policy/enforcement proof
AI analysis handoff remains separate unless selected workpack owns it
live-view preflight/loopback proof not product live-view readiness
redacted summary export not raw screenshot remote upload
```
