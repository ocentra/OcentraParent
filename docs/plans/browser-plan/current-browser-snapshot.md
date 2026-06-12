# Current Browser Snapshot - 2026-06-02

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `Current Browser Snapshot - 2026-06-02`
> Kind: current snapshot; read for status/gap claims.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This snapshot records current browser source, proof, UI, and gap state before
the browser plan is split into implementation workpacks.

## Product Claim Boundary

Current docs already require this boundary:

```text
Managed browser proves exact URL/tab.
Unmanaged browser proves bypass/process use only.
Network/domain proves destination only.
Extension is optional helper, not foundation.
```

The current feature doc says exact URL/tab knowledge requires a managed browser
or proved browser bridge. Process/window and network metadata cannot claim
exact page activity.

## Contracts That Exist

`packages/activity-domain` already defines:

- browser family/channel;
- capability status;
- managed session status;
- active tab state;
- custody/query visibility labels;
- browser tab evidence;
- browser read model;
- browser intervention rows/read model.

`packages/parent-domain` already defines:

- browser control identifiers;
- browser control catalog values;
- authoring manifest shapes;
- browser policy value/update contracts;
- browser questionnaire forest data/contracts;
- V0.8 browser/domain adapter proof contracts.

`packages/agent-protocol-domain` already defines:

- browser policy update command creation;
- browser policy event parsing;
- typed command/event bridge into Rust protocol names.

## Feature Routing Snapshot

The owning feature is `docs/features/browser-web-control.md`. It owns the
managed/unmanaged browser claim boundary and the current checklist for managed
launch/profile, exact URL/tab evidence, unmanaged bypass status,
site/domain/category targets, schedules/exceptions, dry-run evidence refs,
adapter capability status, real blocking/terminate proof where claimed, and
exact active-tab/host-domain proof.

Adjacent feature docs reference browser as shared context:

- `enforcement-integrity-tamper.md` owns enforcement integrity and broad-adapter
  non-claims.
- `evidence-store-query.md` owns the common journal/query-store posture.
- `network-domain-control.md` owns weaker domain/flow metadata and explicitly
  cannot claim exact active tab from network metadata.
- `policy-schedules-approvals.md` owns policy evaluation and approval UX
  boundaries.
- `screen-evidence-analysis.md` and `social-video-control.md` may later consume
  browser evidence, but they do not own browser proof.
- `remote-lan-mobile-platforms.md` owns Android/iOS/mobile platform gaps.
- `production-distribution-support.md` owns support/redaction and release proof
  constraints.

This plan folder centralizes browser task planning while those shared feature
docs remain in place.

## Rust Runtime That Exists

`crates/agent-core` already has:

- managed browser executable identity helpers;
- managed launch plan generation;
- default/unowned profile rejection;
- Chromium launch arguments with user data dir, profile directory, loopback
  debug address, debug port, no-first-run, and no-default-browser-check;
- CDP bridge HTTP helpers;
- CDP `/json/version` and `/json/list` polling;
- page-target filtering;
- tab target mapping to activity events with `activeState: unknown`;
- activity store browser/intervention helpers.

`crates/agent-service` already has:

- managed browser status reporting;
- configured bridge-port polling;
- activity journal/SQLite recording path for browser events;
- missing browser/profile/bridge/unmanaged status helpers;
- browser policy API/runtime/store/compiler files;
- browser intervention payload/report files.

`crates/agent-protocol` already has:

- browser protocol structs;
- browser read model structs;
- browser managed state structs;
- browser policy/intervention values;
- constants for browser command/event/value shapes.

## Portal That Exists

`apps/portal` already renders:

- browser managed status;
- browser evidence summary;
- browser intervention state;
- browser protection summary cards.

`vendor/ocentra-parent-core-ui` already has a browser rules questionnaire
component used as a richer UI source for browser policy authoring surfaces.

## Proof That Exists

Root scripts already include:

```text
npm run test:managed-browser-matrix
npm run test:managed-browser-service-proof
npm run test:managed-browser-intervention
```

Additional proof scripts include:

```text
node scripts/test/v0-8-browser-domain-adapter-proof.mjs
node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs
node scripts/test/browser-performance-health-proof.mjs
node scripts/test/browser-plan-e2e-manual-proof-artifacts.mjs
```

The managed-browser matrix and intervention scripts are real installed-browser
proof harnesses, not default CI unit tests. The performance-health and
artifact-manifest scripts are fixture/index proof gates that keep runtime and
manual-required claims explicit.

## Current Gaps

- Browser inventory is not a complete product read model across installed,
  running, supported, unsupported, managed, unmanaged, packaged, and portable
  browsers.
- Managed profile store repair, custody, redaction, and restart semantics need
  explicit workpack proof.
- Active tab proof is still separate from target-list proof. `/json/list`
  target rows should remain `unknown` active state until focus/activation proof
  exists.
- Managed browser intervention proof exists as a harness, but product-level
  warning/blocking still needs typed policy decision refs, journaled action
  refs, audit refs, child-facing delivery state, and portal proof.
- Unmanaged browser URL evidence remains not claimed. Unmanaged process
  terminate/warn states exist only as scoped proof paths, not broad OS blocking.
- AppLocker/App Control prevention remains real platform proof/manual-required.
- Firefox, Safari, Android, iOS, extension/native-host, owned browser shell,
  managed configurations, FamilyControls, and mobile browser support remain
  separate adapter/platform proof work.

## Where We Want To Be

The browser subsystem should become a service-backed product flow from code to
UI:

```text
browser inventory
-> managed profile/session
-> Ocentra-owned bridge
-> typed evidence
-> encrypted journal
-> SQLite read model
-> policy/AI evidence refs
-> portal status and actions
-> proof artifacts and manual-required gaps
```

Every visible parent claim should answer:

- What browser/source produced this?
- Is it managed or unmanaged?
- What exact evidence exists?
- How fresh is it?
- Is active tab known or unknown?
- What can Ocentra do now?
- Which actions are manual-required?
- What proof artifact backs the claim?

## Enhancement Rule

Future browser work should enhance the existing code layout:

- add missing contracts to the existing domain packages;
- mirror them in the existing Rust protocol crate;
- extend current `agent-core` browser helpers instead of replacing them;
- wire through current `agent-service` command/read-model paths;
- render through current portal/browser/core-ui surfaces;
- reuse existing proof scripts where they already cover a claim;
- add new proof only where current scripts cannot cover the claim.
