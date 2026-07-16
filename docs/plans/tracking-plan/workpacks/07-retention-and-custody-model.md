# WP07 Retention And Custody Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP07 Retention And Custody Model`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Make location retention, deletion, export, and custody explicit before storing
or showing location history.

## Source Inputs

- `docs/expectations/data-custody.md`
- `docs/expectations/location-geofence.md`
- `docs/device-location-tracking-capability-guide.md`

## Target State

Retention supports last-known-only, 24h, 7d, 30d, custom, parent export, and
delete-after-alert-resolved. Custody labels distinguish child-device local,
parent-device local, family LAN relay, parent-owned storage, and
parent-approved cloud without default Ocentra-hosted storage.

## Tests And Proof

Proof root: `output/tracking-plan-proof/07-retention-and-custody-model/`

- `01-contract-proof.log`
- `10-journal-sqlite-proof.json`
- `14-retention-delete-proof.json`
- `17-retention-export-proof.json`
- `18-retention-settings-read-model-proof.json`
- `19-retention-settings-writer-boundary-proof.json`
- `20-retention-settings-mutation-proof.json`
- `21-retention-settings-write-command-proof.json`
- `22-retention-local-service-state-proof.json`
- `23-retention-durable-settings-proof.json`
- `24-retention-product-readiness-proof.json`
- `25-retention-runtime-artifact-gate-proof.json`
- `26-retention-product-settings-writable-execution-proof.json`
- `27-retention-platform-enforcement-preflight-proof.json`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Default remote sync off.
- [ ] Default remote AI off.
- [ ] Add delete/tombstone/export tests.
- [ ] Ensure deleted points cannot stay visible in map UI.
- [ ] Keep notification providers out of location evidence custody.

## Where We Are

This workpack has P0 contract proof plus P1 fixture proof for retention delete
read-model filtering, parent-owned retention export, and UI-visible
deleted-history hiding from `codex/tracking-plan-full-scope` under the proof
root below. It now also has P2 retention settings read-model rows for retention
window, delete-after-alert, parent export, remote-sync disabled, and remote-AI
disabled state through
`node scripts/test/tracking-retention-settings-read-model-proof.mjs`, plus
writer-boundary preflight rows for those five setting write intents through
`node scripts/test/tracking-retention-settings-writer-boundary-proof.mjs`.
Local executed service mutation proof for those same five rows now exists
through `node scripts/test/tracking-retention-settings-mutation-proof.mjs`,
while remote sync and remote AI stay disabled. Typed service transport proof for
the retention settings write command local-execution result now exists through
`node scripts/test/tracking-retention-settings-write-command-proof.mjs`.
Local service state readback proof now derives the accepted write-command result
into parent-domain rows through
`node scripts/test/tracking-retention-local-service-state-proof.mjs`, preserving
the applied retention values, service state revision, snapshot ref, and local
durable settings store ref without claiming writable product settings.
Durable settings proof now derives local durable persistence rows from that
local service state through
`node scripts/test/tracking-retention-durable-settings-proof.mjs`, making the
Rust service durable store ref and persisted state explicit without claiming
product-ready writable settings, platform runtime, or production hardening.
Product-readiness blocker proof now consumes those durable settings rows through
`node scripts/test/tracking-retention-product-readiness-proof.mjs` and lists the
remaining blockers for writable product settings execution, platform runtime,
child-device delivery, provider delivery, notification receipt ingestion,
physical-device proof, authority enrollment, and production worker hardening
without changing the no-product-ready claim.
Product-settings writable execution artifact proof now consumes the local
service state readback proof through
`node scripts/test/tracking-retention-product-settings-writable-execution-proof.mjs`
and writes
`output/tracking-plan-proof/tracking-retention/product-settings-writable-execution.json`
plus WP07/WP33 companion artifacts for the local writable execution row. Its
derivation matrix preserves source proof refs, local service state revision,
snapshot ref, durable settings store ref, applied retention values, and
no-claim boundaries while keeping platform runtime retention enforcement,
portal writable UI, child-device delivery, provider delivery, notification
receipts, physical-device proof, authority, production, and product-ready claims
false.
Retention runtime artifact gate proof now consumes the product-readiness blocker
source and checks the required writable product settings and platform retention
runtime artifact refs through
`node scripts/test/tracking-retention-runtime-artifact-gate-proof.mjs` while
keeping those execution claims false. The product-readiness closure and
real-runtime handoff proofs now carry this retention runtime artifact accounting
forward: two required artifacts, one present local writable execution artifact,
and one missing platform retention runtime enforcement artifact.
Retention platform enforcement preflight proof now consumes that gate through
`node scripts/test/tracking-retention-platform-enforcement-preflight-proof.mjs`
and writes Android, iOS, and desktop manual-required acceptance rows plus a
manual validation runbook for the missing platform runtime artifact.
Product-readiness closure and real-runtime handoff now carry its row/artifact
counts. It is not platform enforcement or product-ready retention proof.
The hosted parent route now also sends that typed write command and renders the
service accepted result with applied local retention values. Writable product
settings, platform behavior, applied product-ready writable retention execution,
and product claim readiness are not claimed beyond the proof state recorded in
`proof-summary.json`, `14-retention-delete-proof.json`,
`17-retention-export-proof.json`, `18-retention-settings-read-model-proof.json`,
`19-retention-settings-writer-boundary-proof.json`,
`20-retention-settings-mutation-proof.json`,
`21-retention-settings-write-command-proof.json`,
`22-retention-local-service-state-proof.json`,
`23-retention-durable-settings-proof.json`,
`24-retention-product-readiness-proof.json`,
`25-retention-runtime-artifact-gate-proof.json`,
`26-retention-product-settings-writable-execution-proof.json`,
`27-retention-platform-enforcement-preflight-proof.json`,
the WP30 hosted UI proof artifact, and the implementation checklist.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/07-retention-and-custody-model.md
- docs/plans/tracking-plan/implementation-checklist.md
- `packages/activity-domain/src/tracking-retention-runtime.ts`
- `output/tracking-plan-proof/07-retention-and-custody-model/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, applied product-ready writable retention execution, and
  product claims remain manual-required until the assigned proof artifacts
  exist.
- Local service state readback proof is derived from the accepted local write
  command result and now carries the local durable settings store ref; it is not
  physical-device behavior proof.
- Durable settings proof records local durable persistence from the Rust
  service command. It does not claim writable product settings, platform
  runtime, production hardening, or product-ready retention behavior.
- Product-readiness blocker proof enumerates the remaining hard blockers over
  the durable-settings evidence; it is not writable product settings execution,
  platform runtime, production hardening, authority, physical-device, provider
  delivery, notification receipt, or child-device behavior proof.
- Retention runtime artifact gate proof checks required artifact refs for
  writable product settings execution and platform runtime retention
  enforcement. The local product-settings writable execution artifact may be
  present, but the gate remains manual-required until platform retention
  enforcement is proved. Product-readiness closure and real-runtime handoff
  accounting now cite that exact present/missing split; it is not platform
  enforcement, production, or product-ready proof.
- Retention platform enforcement preflight proof names Android, iOS, and
  desktop runtime acceptance criteria, manual validation commands, and artifact
  refs for the missing platform enforcement artifact. It remains
  manual-required and does not claim platform enforcement, production, or
  product-ready retention proof.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [ ] Touched files: tracking contract/runtime files, proof scripts, tracking
      plan docs, checklist, and this workpack doc.
- [ ] Validation commands and results:
      `node scripts/test/tracking-plan-runtime-proof.mjs` passed.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/07-retention-and-custody-model/`, including
      `14-retention-delete-proof.json` and
      `17-retention-export-proof.json`.
- [ ] Product doc/checklist updates: feature doc, implementation checklist,
      product capability checklist, README, and this workpack updated.
- [ ] Known gaps/manual-required states: Android/iOS physical proof, precise
      desktop, provider delivery, notifications, live service-backed retention
      UI, and full UI remain proof-gated as applicable.
- [ ] Workpack id and branch:
      `codex/tracking-retention-settings-read-model-proof`.
- [ ] Touched files: parent-domain retention settings read-model proof
      source/test, proof harness, tracking feature doc, implementation
      checklist, WP07, WP32, and generated WP07/WP32 proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-retention-settings-read-model-proof.mjs`
      passed locally.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json`,
      and `test-results/tracking-retention-settings-read-model-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP07, and WP32 updated; central capability checklist row is
      not edited by this worker while another lane owns it.
- [ ] Known gaps/manual-required states: actual writable product settings,
      live service-backed retention UI, service mutation, platform runtime,
      child-device delivery, Android/iOS physical proof, authority, provider
      delivery, notification receipts, and production proof remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-retention-settings-writer-boundary-proof`.
- [ ] Touched files: parent-domain retention settings writer-boundary proof
      source/test, proof harness, tracking feature doc, implementation
      checklist, WP07, WP32, and generated WP07/WP32/WP33 proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-retention-settings-writer-boundary-proof.mjs`
      passed locally.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/07-retention-and-custody-model/19-retention-settings-writer-boundary-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/25-retention-settings-writer-boundary-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/25-retention-settings-writer-boundary-proof.json`,
      and `test-results/tracking-retention-settings-writer-boundary-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP07, and WP32 updated; central capability checklist row is
      not edited by this worker while another lane owns it.
- [ ] Known gaps/manual-required states: executed service mutation, live
      service-backed retention UI, platform runtime, child-device delivery,
      Android/iOS physical proof, authority, provider delivery, notification
      receipts, and production proof remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain retention settings mutation proof source and
      test, proof harness, tracking feature doc, implementation checklist, WP07,
      WP32, and generated WP07/WP32/WP33 proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-retention-settings-mutation-proof.mjs`
      passed locally.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/26-retention-settings-mutation-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/32-retention-settings-mutation-proof.json`,
      and `test-results/tracking-retention-settings-mutation-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP07, and WP32 updated; central product capability checklist
      remains hub/primary sequenced.
- [ ] Known gaps/manual-required states: live service-backed writable retention
      UI, platform runtime, child-device delivery, Android/iOS physical proof,
      authority, provider delivery, notification receipts, production workers,
      and product-ready retention behavior remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: agent-protocol-domain retention settings write command
      contract/test, Rust agent-protocol command/event/result types, Rust
      agent-service WebSocket response test, proof harness, tracking feature
      doc, implementation checklist, WP07, WP32, and generated WP07/WP32/WP33
      proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-retention-settings-write-command-proof.mjs`
      passed locally.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/07-retention-and-custody-model/21-retention-settings-write-command-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/27-retention-settings-write-command-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/33-retention-settings-write-command-proof.json`,
      and
      `test-results/tracking-retention-settings-write-command-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP07, and WP32 updated; central product capability checklist
      remains hub/primary sequenced.
- [ ] Known gaps/manual-required states: durable writable retention UI,
      product-ready service execution, platform runtime, child-device
      delivery, Android/iOS physical proof, authority, provider delivery,
      notification receipts, production workers, and product-ready retention
      behavior remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain retention local service state proof source and
      test, proof harness, tracking feature doc, implementation checklist, WP07,
      WP32, and generated WP07/WP32/WP33 proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-retention-local-service-state-proof.mjs`
      passed locally.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/07-retention-and-custody-model/22-retention-local-service-state-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/33-retention-local-service-state-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/40-retention-local-service-state-proof.json`,
      and
      `test-results/tracking-retention-local-service-state-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP07, and WP32 updated; central product capability checklist
      remains hub/primary sequenced.
- [ ] Known gaps/manual-required states: writable product settings, platform
      runtime, child-device delivery, Android/iOS physical proof, authority,
      provider delivery, notification receipts, production workers, and
      product-ready retention behavior remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain retention durable settings proof source and
      test, proof harness, tracking feature doc, implementation checklist, WP07,
      WP32, and generated WP07/WP32/WP33 proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-retention-durable-settings-proof.mjs`
      passed locally.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/07-retention-and-custody-model/23-retention-durable-settings-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/34-retention-durable-settings-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/41-retention-durable-settings-proof.json`,
      and
      `test-results/tracking-retention-durable-settings-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP07, and WP32 updated; central product capability checklist
      remains hub/primary sequenced because E-B owns the lock.
- [ ] Known gaps/manual-required states: product-ready writable settings,
      platform runtime, child-device delivery, Android/iOS physical proof,
      authority, provider delivery, notification receipts, production workers,
      and product-ready retention behavior remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: portal live activity state, portal event-result routing,
      hosted retention settings UI proof model, React route panel, DOM tracking
      status panel, hosted Playwright proof, portal tests, text-domain tokens,
      portal-domain proof artifact marker, hosted UI proof harness, tracking
      feature doc, implementation checklist, WP07, WP30, WP32, and regenerated
      hosted UI proof screenshots/results.
- [ ] Validation commands and results:
      `cmd /c npm run build --workspace @ocentra-parent/agent-protocol-domain`
      passed; `cmd /c npm run build --workspace @ocentra-parent/text-domain`
      passed; `cmd /c npm run build --workspace @ocentra-parent/portal-domain`
      passed; `cmd /c npm run test --workspace @ocentra-parent/portal --
tracking-status-panel` passed; `cmd /c npm run build --workspace
@ocentra-parent/portal` passed; `cmd /c npm run format:check` passed;
      `cmd /c npm run test:tracking-plan-hosted-ui-proof` passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-retention-settings.png`,
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/17-hosted-ui-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/18-hosted-ui-accessibility-proof.json`,
      and `test-results/tracking-plan-hosted-ui-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP07, WP30, and WP32 updated; central product capability
      checklist remains hub/primary sequenced.
- [ ] Known gaps/manual-required states: writable product settings and applied
      product-ready service mutation execution remain pending; the current
      service proof is local mutation plus local state revision and durable
      store ref only. Platform runtime, child-device delivery, Android/iOS physical proof, authority,
      provider delivery,
      notification receipts, production workers, full parent/child UI beyond
      the hosted route, and product-ready retention behavior remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain applied retention settings runtime bridge
      proof source and test, focused proof harness, tracking feature doc,
      implementation checklist, WP07, WP33, and generated WP07/WP33 proof
      artifacts.
- [ ] Validation commands and results:
      `cmd /c npm run build --workspace @ocentra-parent/parent-domain` passed;
      `cmd /c npm run test --workspace @ocentra-parent/parent-domain --
tracking-retention-applied-settings-runtime-bridge-proof` passed;
      `node scripts/test/tracking-retention-applied-settings-runtime-bridge-proof.mjs`
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/07-retention-and-custody-model/28-retention-applied-settings-runtime-bridge-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/76-retention-applied-settings-runtime-bridge-proof.json`,
      `output/tracking-plan-proof/tracking-retention-applied-settings-runtime-bridge-proof/proof.json`,
      and
      `test-results/tracking-retention-applied-settings-runtime-bridge-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP07, and WP33 updated; central product capability checklist
      remains hub/primary sequenced because E-B owns the lock.
- [ ] Known gaps/manual-required states: this bridge marks the local writable
      execution artifact present but keeps platform runtime retention
      enforcement, production write-result UI, child-device runtime, Android/iOS
      physical proof, authority, provider delivery, notification receipts,
      production workers, and product-ready retention behavior proof-gated.
