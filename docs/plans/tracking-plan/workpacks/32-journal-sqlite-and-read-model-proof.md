# WP32 Journal SQLite And Read-Model Proof

## Purpose

Store, replay, query, delete, and cite tracking evidence through the shared
journal/SQLite/read-model path before portal, policy, AI, or reports consume it.

## Source Inputs

- `docs/features/evidence-store-query.md`
- `docs/expectations/evidence-storage.md`
- `docs/expectations/data-custody.md`
- `docs/plans/tracking-plan/v0-5-location-test-blueprint.md`

## Target State

Location, status, geofence, check-in, acknowledgement, alert, AI, and retention
events are journaled, replayable, queryable, deletable, and cited by read
models.

## Tests And Proof

Proof root: `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/`

- `03-runtime-location-evidence.json`
- `04-device-status-proof.json`
- `05-geofence-transition-proof.json`
- `10-journal-sqlite-proof.json`
- `14-retention-delete-proof.json`
- `18-service-read-model-proof.json`
- `19-wsl-local-replay-proof.json`
- `16-validation-commands.log`
- Pre-device gate:
  `output/tracking-plan-proof/pre-device-gap-closure/proof-summary.json`

## AI Worker Checklist

- [ ] Journal evidence before portal/policy/AI use.
- [ ] Add replay/query/delete tests.
- [ ] Add tombstone proof.
- [ ] Ensure AI/report/policy cite stored refs.
- [ ] Keep Ocentra-hosted storage off by default.
- [x] Include the P2 service read-model proof in the pre-device gate.
- [x] Record WSL/local replay proof state for the read-model replay path.

## Where We Are

This workpack has P0 contract proof, P1 Rust ActivityStore SQLite ingest proof,
and P2 service-command proof for tracking event kinds from
`codex/tracking-plan-full-scope` under the proof root below. The service proof
adds a narrow `agent.activity.tracking.read-model.get` command that returns
SQLite tracking rows and citation IDs through `trackingReadModel`; the parent
portal now consumes that event as a narrow live summary on the `policy-tracking`
route. Full UI, platform replay, deletion/tombstone replay, export, and
physical-device product claims are not claimed beyond the proof state recorded
in `proof-summary.json`, `10-journal-sqlite-proof.json`,
`18-service-read-model-proof.json`, and the implementation checklist.
The pre-device proof gate now reruns this service proof and records the
remaining deletion/tombstone replay, richer read-model, full UI, hosted
accessibility, and platform replay gaps before device work starts.
WSL/local replay now has evidence through
`npm run test:tracking-plan-wsl-local-proof`. That harness proves WSL2/Ubuntu
availability, maps this Windows linked worktree Git dir for WSL, runs
`npm run build:contracts`, and passes
`cargo test -p ocentra-parent-agent-core tracking_read_model`. The WSL JS
service read-model proof is still blocked by the missing Linux Rolldown native
optional dependency in this Windows-installed `node_modules`, so the WSL
artifact is partial and does not upgrade product readiness.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/32-journal-sqlite-and-read-model-proof.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Hosted portal screenshot/accessibility proof, WSL JS service replay,
  deletion/tombstone replay, richer product read models, full UI, platform
  replay, export, provider, and physical-device claims remain manual-required
  until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [x] Touched files: Rust ActivityStore/protocol files, tracking
      contract/runtime files, proof scripts, tracking plan docs, checklist, and
      this workpack doc.
- [x] Validation commands and results:
      `node scripts/test/tracking-plan-runtime-proof.mjs` passed; focused
      service proof command
      `node scripts/test/tracking-plan-service-read-model-proof.mjs` passed.
- [x] Proof artifacts under
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/`,
      including `10-journal-sqlite-proof.json` and
      `18-service-read-model-proof.json`.
- [x] Product doc/checklist updates: owning feature doc, feature list, product
      capability checklist, implementation checklist, and this workpack doc
      updated for the P2 service read-model proof.
- [x] Known gaps/manual-required states: hosted portal screenshot/accessibility
      proof, deletion/tombstone replay, richer read models, full UI, platform
      replay, export, Android/iOS physical proof, provider delivery, and
      notifications remain proof-gated as applicable.
- [x] Workpack id and branch: `codex/tracking-wsl-local-proof`.
- [x] Touched files: WSL local proof script, root test script wiring, tracking
      feature doc, tracking README, implementation checklist, this workpack
      doc, and generated WSL proof artifacts.
- [x] Validation commands and results:
      `npm run test:tracking-plan-wsl-local-proof` passed as an honest partial proof.
      WSL2/Ubuntu, linked-worktree Git mapping, `npm run build:contracts`, and
      `cargo test -p ocentra-parent-agent-core tracking_read_model` passed.
      WSL JS service proof remains blocked by the missing Linux Rolldown optional
      dependency.
- [x] Proof artifacts under
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/`,
      including `19-wsl-local-replay-proof.json`.
- [x] Product doc/checklist updates: owning feature doc, tracking README,
      implementation checklist, and this workpack doc updated. Central product
      capability checklist update is queued through DOC_DELTA instead of
      editing `docs/product-capability-checklist.md`.
- [x] Known gaps/manual-required states: WSL JS service replay dependency,
      hosted portal screenshot/accessibility proof, deletion/tombstone replay,
      richer read models, full UI, platform replay, export, Android/iOS
      physical proof, provider delivery, and notifications remain proof-gated.
