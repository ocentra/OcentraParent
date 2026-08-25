# 07 Managed Chromium Launcher

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `07 Managed Chromium Launcher`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Where We Are

Launch planning already builds Chromium arguments for managed profile, loopback
debug address, debug port, profile directory, and first-run suppression.
WP07 now also has a loopback-only random bridge-port reservation helper, launch
plans/results carry the reserved port, managed launch spawn failures are
covered, and service status helpers report launch-pending/running-managed
without claiming bridge connectivity before CDP polling succeeds.

## Where We Want To Be

The child service can launch Edge/Chrome/Chrome for Testing through managed
profiles with tracked session/process/bridge state and no default-profile
attachment.

### Reviewed production-readiness boundary (2026-08-19)

Canonical `f80b47c6a` removes the unreachable service launch state,
environment/dev profile authority, and placeholder bridge poll. The websocket
status path now reports explicit manual-required/unavailable state and cannot
claim a retained launch or connected bridge. Core launch authority remains
private, but no service owner mounts it. The next packet must use a private
owner-issued start/stop boundary, retain launch/process/profile/bridge custody,
revalidate it around I/O, and confirm teardown. WP07 remains blocked/open and
is not PR-ready.

Required tests remain open: retained launch lifecycle and restart/expiry,
process exit and bridge disconnect, owner/custody mismatch, and no default or
unowned profile attachment through the real service route. The existing
launcher planning and failed-spawn tests do not cover this integration seam.
The current unit tests also use old status-helper arities and private launch
fields; repair them only in the later test-source wave, without adding a public
constructor or fixture authority.

## Scope

- Edge/Chrome executable identity.
- Random reserved local bridge port.
- Loopback-only debugging address.
- Managed session id, profile id, process id, browser family/channel.
- Policy revision id and custody labels.
- Launch-pending, running-managed, bridge-connected/disconnected, adapter-error
  states.

## Touched Paths

- `crates/agent-core/src/browser_managed_session.rs`
- `crates/agent-core/src/browser_managed_discovery.rs`
- `crates/agent-service/src/browser_runtime*.rs`
- `scripts/test/managed-browser-profile-matrix.mjs`

## Tests And Proof

- Test doubles are forbidden by repo rules, so fake launcher integration tests
  were not added. Coverage uses real launch planning, loopback socket
  reservation, and real failed-spawn behavior for a missing supported executable.
- Missing executable, unreserved port, default-profile, unowned-profile,
  unsupported executable, and failed spawn tests are covered in
  `crates/agent-core/src/browser_managed_session_tests.rs`.
- A constrained real-browser harness run covered installed Chrome and Edge with
  one managed profile and one requested URL:
  `OCENTRA_PARENT_MANAGED_BROWSER_MATRIX_BROWSERS=chrome,edge`,
  `OCENTRA_PARENT_MANAGED_BROWSER_MATRIX_URLS=https://example.com/`, and
  `OCENTRA_PARENT_MANAGED_BROWSER_MATRIX_PROFILES=managed-browser-profile-a`.
- Final harness evidence is
  `test-results/managed-browser-profile-matrix/2026-06-02T20-11-09-106Z.json`;
  the generated evidence redacts raw DevTools URLs as `loopback-redacted`.
- The default broad matrix timed out locally and was cleaned up, so it is not
  used as final proof for this workpack.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/07-managed-chromium-launcher/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after contracts exist.
- [ ] Raw evidence artifacts captured where applicable: bridge/CDP payloads, managed session state, unmanaged process rows, journal entries, SQLite/read-model rows, policy decisions, and action results.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; if no UI changed, `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: no default profile attach, no unowned bridge, no unmanaged exact URL claim, no raw debugger URL exposure, and no AI direct enforcement.
- [ ] Manual platform proof captured for real browser/OS claims, including OS/browser version, command steps, screenshots/logs, and manual-required labels.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Launcher proof does not by itself prove active-tab focus, exact URL product
actions, intervention, or bridge custody/security. CDP target proof,
active-tab proof, policy action, and portal UI status surfaces remain owned by
later browser-plan workpacks.

## WP07 Implementation Notes

- Proof pack: `output/browser-plan-proof/07-managed-chromium-launcher/`.
- Validation summary: `output/browser-plan-proof/07-managed-chromium-launcher/14-validation-summary.md`.
- Product checklist decision: no `docs/product-capability-checklist.md` update
  for WP07 alone because the work adds launcher/session state and manual harness
  evidence without upgrading real bridge connectivity, exact URL, active-tab,
  intervention, UI, or platform support claims.
