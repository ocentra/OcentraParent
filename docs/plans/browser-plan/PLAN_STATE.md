# Browser Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `Browser Plan State`
> Kind: current state and open gaps.
> Read when: Immediately after plan AGENTS.md; use for current state and no-claim boundaries.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This is the default resume/status file; large historical docs are linked, not
embedded.

Audit note (2026-06-16): earlier generated summaries in this file overstated
checked checklist/workpack state. Use the selected workpack,
`implementation-checklist.md`, and the current proof roots as truth.

Code-pass note (2026-08-16): WP05 has a production-code draft on
`codex/browser-code-pass`. It is unvalidated and test/proof/checklist-deferred;
the graph must not treat it as complete.

Code-pass note (2026-08-16): WP17 now has a production-code draft for
action-intent reference ownership and the child-delivery boundary. References
are derived from the action intent; a handoff without trusted parent context
remains unavailable/manual-required and does not mint child acceptance. This
slice is unvalidated and tests/proof/checklist-deferred; WP17 remains open.

Code-pass note (2026-08-16): WP19 now has a production-code draft that keeps
unmanaged fallback state unavailable for managed/unobserved rows and routes
unmanaged block-like actions to `os-block-manual-required`. It does not claim
OS blocking, process termination, relaunch, or exact unmanaged URL control;
tests/proof/checklist remain deferred and WP19 remains open.

Code-pass note (2026-08-16): WP14 now has a production-code draft for the
parent-runtime Browser route. The existing typed agent-service managed-status
and intervention models are serialized through dedicated protocol payload
fields and hydrated into the existing Rust-owned live-activity bridge. The
route predicate keeps browser state off unrelated snapshots. This remains
unvalidated with tests/proof/checklist deferred; it does not claim active-tab
focus, unmanaged exact-URL authority, OS blocking, or action delivery.

Code-pass note (2026-08-16): WP13 now has a production-code draft for the
typed `ActivityBrowserReadModel` parent-bridge seam. The Activity and Browser
routes load the existing agent-service read model and project it through the
existing portal live-activity adapter shape. This remains unvalidated with
tests/proof/checklist deferred; it does not add capture, focus proof,
unmanaged exact-URL authority, intervention delivery, or enforcement.

Code-pass note (2026-08-16): WP18 now has a production-code draft for the
typed `BrowserInventoryReadModel` parent-bridge seam. The Browser route loads
the existing service inventory event and projects its process-only rows into
the existing portal live-activity state. This remains unvalidated with
tests/proof/checklist deferred; it does not add exact URL, active-tab, page
title, OS blocking, process termination, or enforcement authority.

Code-pass note (2026-08-16): WP13 now also has a production-code draft for
the stored `BrowserEvidenceReadModel` parent-bridge seam. The Browser route
loads the existing evidence event and projects its typed rows, active-state,
proof-source, custody, and query-visibility fields into the existing portal
state. This remains unvalidated with tests/proof/checklist deferred and does
not promote target-list evidence into known-active or add enforcement.

Code-pass audit note (2026-08-16): no further legal Browser production slice
was found after WP18 and the WP13 evidence bridge. WP11 has no focus/activation
provider beyond target-list-only evidence; WP21 has only the managed native-host
validator and no extension package or host registration owner; WP20 has
representation only and requires real Windows AppLocker/WDAC authority; WP22
has fixture budget evaluation but no runtime health producer or bridge owner.
The remaining numbered workpack rows are validation, proof, manual-platform, or
documentation gaps unless one of those external authorities is added.

Production reachability audit (2026-08-16):

```text
WP11: crates/agent-service/src/browser_runtime_impl/bridge.rs calls
      crates/agent-core/src/browser_bridge_poll.rs, whose target parser emits
      BrowserActiveTabState::Unknown and BrowserActiveProofSource::TargetListOnly.
      No focus/activation, extension, foreground-correlation, or owned-shell
      provider is present; target-list evidence must remain unknown.
WP21: crates/agent-core/src/browser_bridge_native_host.rs exports only a
      validator whose inbound references are test references. No extension
      package, native-host registration, or runtime IPC owner exists.
WP20: the browser-domain App Control representation is reachable through the
      agent-service enforcement product-control report, but its states are
      static/manual-required specifications. No Windows AppLocker/WDAC runtime
      input or policy owner exists.
WP22: crates/browser-core/src/performance_budget.rs exposes a fixture budget
      matrix and evaluator with no production service-health producer or bridge
      caller. A telemetry bridge would be synthetic without a runtime owner.
```

Browser production implementation is therefore parked, not complete. The exact
remaining production-code blockers are WP11 native focus authority, WP21
extension/native-host packaging and registration, WP20 Windows AppLocker/WDAC
authority, and WP22 runtime performance-health measurement ownership.

Topology warning (2026-08-16): `npm run graph:report -- --json` currently
derives all 30 Browser workpacks as `planned` with empty dependency lists,
despite the source/index notes and existing Rust/service implementation roots.
This audit does not edit `graph.json` or run graph bootstrap. Legacy ownership
references to `packages/activity-domain/src/browser*.ts` remain stale; the
active TypeScript edge ownership is `packages/browser-domain` and the active
runtime ownership is the Rust paths in `source-index.md`.

Reviewed source result (2026-08-19, WP07/WP09): canonical `f80b47c6a` removes
the unreachable service launch state, environment/dev profile authority,
placeholder bridge polling, and dead Browser-to-Screen handoff. The production
websocket path now returns explicit managed-browser manual-required/unavailable
status; it cannot claim a retained launch, connected bridge, trusted target, or
Screen delivery. Core launch/capture authority remains private and bounded,
but no service owner mounts it. This is honest source consolidation, not
delivered WP07/WP09 behavior, validation, or PR readiness.

The next coherent production packet remains owner-gated:

1. Introduce a private owner-issued start/stop boundary that retains
   `BrowserManagedLaunch` in service state without env/dev/caller authority.
2. Revalidate PID, executable, exact profile argument, and bridge ownership
   before and after `/json/version` and `/json/list` I/O; teardown must wait for
   and confirm process exit rather than reporting stopped optimistically.
3. Keep target-list activity `Unknown`; mint same-launch target authority only
   from the retained owner state.
4. Add a typed Screen-owned handoff only after Screen accepts that boundary.
   Browser must not import or simulate Screen runtime authority.

The later test-source wave must first repair the now-stale Browser tests:
`browser_runtime_status.rs` and `browser_runtime_tests.rs` use old status-helper
arities, while `browser_inventory_read_model_tests.rs` and
`browser_runtime_tests.rs` construct private `BrowserManagedLaunch` fields.
Then write the missing retained-launch integration test root and the missing
same-launch CDP integration root, including owner mismatch, process
replacement, teardown, restart/expiry, malformed/oversized/timeout, target
disappearance/navigation, no-active-tab-claim, and unavailable Screen handoff.
WP07/WP09 remain open and blocked; no proof or completion is inferred.

## Scope

This folder is the single working plan location for managed browser evidence, browser policy authoring, unmanaged browser fallback, browser intervention, and parent-facing browser UI/UX requirements.

## Current ownership interpretation

```text
crates/schema:
  Canonical shared browser/evidence/read-model/intervention contracts when browser shapes cross package, crate, app, or plan boundaries.

browser-core:
  Child-local Rust browser observation, evidence-event, AI-request, policy-request, and source-readiness boundary in `crates/browser-core` and its generated/runtime companions.

agent-protocol and agent-service:
  Wire/service/read-model boundaries when selected. They are not default owners for every browser contract.

AI plan:
  Consumes stored browser evidence, source refs, or structured digest refs. AI does not import browser runtime, scrape browser state, or decide enforcement.

Policy/enforcement plans:
  Consume source-ready browser evidence and parent rules. They own deterministic decisions/actions; browser owns source truth and browser-specific intervention handoff readiness.

portal-domain and apps/portal:
  Parent-visible browser status and activity projections. They do not capture browser state, infer exact URLs, run policy, or enforce.

Network, screen, app-game, tracking, LAN, remote, account, data-custody, and setup plans:
  Adjacent sibling owners or handoff consumers. They must not re-own browser source truth.
```

## Current coupling risks

```text
- Older plan-local source ownership notes still reference legacy `packages/activity-domain/src/browser*.ts` paths that do not exist in this checkout.
- Network/process/window evidence must not be promoted into exact URL, active tab, page title, or browser-game proof without selected browser-source proof.
- Managed intervention harness proof does not prove product-level warning/block readiness unless policy decision refs, action refs, audit refs, child delivery state, and portal proof exist.
- Browser reference/control inventory workpacks are not implementation scope by themselves.
```

## Current proof interpretation

```text
All numbered workpacks remain open in their own files.
Plan-local proof roots under output/browser-plan-proof/<workpack-file-stem>/ are currently absent in this checkout.
CDP target-list proof is not active-tab proof.
Unmanaged browser process detection is not exact URL evidence.
Portal display proof is not browser source capture or service proof.
Policy authoring proof is not intervention/action proof.
Platform preflight proof is not platform parity.
Reference/settings inventories are not runtime implementation proof.
```

## Resume route

1. Read this file.
2. Read `NEXT_ACTIONS.md` when starting/resuming.
3. Read `WORKPACK_INDEX.md`.
4. Open only the assigned workpack.
5. Use `CHECKLIST_INDEX.md` for exact checklist sections.
6. Use `PROOF_INDEX.md` for proof artifacts.

## Current snapshot source

- Snapshot: [current-browser-snapshot.md](current-browser-snapshot.md)

## What is already present / proved

- browser family/channel;
- capability status;
- managed session status;
- active tab state;
- custody/query visibility labels;
- browser tab evidence;
- browser read model;
- browser intervention rows/read model.
- browser control identifiers;
- browser control catalog values;
- authoring manifest shapes;
- browser policy value/update contracts;

Current implementation is concentrated in `crates/schema`,
`crates/browser-core`, `crates/agent-protocol`, `crates/agent-core`,
`crates/agent-service`, `packages/portal-domain`, `apps/portal`, and
`scripts/test`.

## Open gaps / missing product runtime

- Browser inventory is not a complete product read model across installed, running, supported, unsupported, managed, unmanaged, packaged, and portable browsers.
- Browser WP06 source-only branch `codex/browser-wp06-custody-repair-aug24` at
  `63d913b93` carries the bounded repair for the 2026-08-24 persisted-binding,
  corruption, atomicity, serialization, filesystem-indirection, and dev-helper
  findings. Focused compile/static guards pass, but independent source review,
  canonical integration, all expected tests, retained proof, pre-commit, CI,
  runtime composition, and acceptance remain open.
- Active tab proof is still separate from target-list proof. `/json/list` target rows should remain `unknown` active state until focus/activation proof exists.
- Managed browser intervention proof exists as a harness, but product-level warning/blocking still needs typed policy decision refs, journaled action refs, audit refs, child-facing delivery state, and portal proof.
- Unmanaged browser URL evidence remains not claimed. Unmanaged process terminate/warn states exist only as scoped proof paths, not broad OS blocking.
- AppLocker/App Control prevention remains real platform proof/manual-required.
- Firefox, Safari, Android, iOS, extension/native-host, owned browser shell, managed configurations, FamilyControls, and mobile browser support remain separate adapter/platform proof work.
- The plan-local source ownership notes still reference legacy
  `packages/activity-domain/src/browser*.ts` paths that do not exist in this
  checkout.
- The expected plan-local proof roots under
  `output/browser-plan-proof/<workpack-file-stem>/` are absent in this
  checkout.

## Checklist summary

- Full checklist: [implementation-checklist.md](implementation-checklist.md) (not default context).
- Table rows detected in `implementation-checklist.md`: 97 total, 0 checked, 0
  partial/manual-required, 97 open.
- Checklist index: [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md).

## Workpack summary

- Numbered workpacks indexed: 24.
- Numbered workpacks with open checkboxes: 24.
- Numbered workpacks with all detected boxes checked: 0.
- Reference/control-routing workpacks with no checkbox status: 6.

### Active/open workpacks

- Every numbered workpack remains open in its own file.
- Browser control settings inventory, coverage matrix, schema proposal, policy questionnaire forest, policy settings catalog, and managed/unmanaged browser reference workpacks remain open as doc/reference material.
- The expected plan-local proof roots under
  `output/browser-plan-proof/<workpack-file-stem>/` are absent in this
  checkout.
- Use `WORKPACK_INDEX.md` to choose the exact assigned row and avoid opening the giant browser inventories by default.

## Default no-read list

- `README_FULL_ORIGINAL.md` unless you need historical full README context.
- Full `implementation-checklist.md` unless `CHECKLIST_INDEX.md` names exact section/row.
- All workpacks; use `WORKPACK_INDEX.md`.
- `WORKPACK_FAMILIES.md` unless the selected workpack owner/proof family is unclear.
- Source inventories and pasted-content audits unless source ownership is unclear.
- Historical checkpoint/proof docs unless `PROOF_INDEX.md` or the assigned workpack names them.

## Health / consistency

- See `PLAN_HEALTH.md` before claiming the whole plan is complete or stale.
- Use the E2E tiers in `TEST_PROOF_EXPECTATIONS.md` before any feature-complete or PR_READY claim.
- Use `WORKPACK_FAMILIES.md` only to classify the selected workpack; do not use it as permission to scan a whole family.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log (or explicit known blocker) from the assigned implementation boundary,
  - a proof pack under the current plan-local proof root from `PROOF_INDEX.md`.
- Current proof-root note:
  - the historical HID-era `docs/proof/browser-plan/slice-*` path is stale for
    this checkout.
  - use `output/browser-plan-proof/<workpack-file-stem>/` plus the selected
    workpack's required artifacts instead.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
