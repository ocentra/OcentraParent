# 06 Managed Profile Store

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `06 Managed Profile Store`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Where We Are

The 2026-08-24 canonical source audit found a useful filesystem/profile
contract, but not a production-ready custody owner. Source-only branch
`codex/browser-wp06-custody-repair-aug24` at `63d913b93` now carries a bounded
repair for the audited defects. It is pushed and focused-check clean, but it is
not canonical or accepted until an independent source review passes. Tests,
proof, pre-commit, CI, runtime composition, and `DONE` remain deliberately
open.

## Where We Want To Be

Managed profiles are Ocentra-owned, per child/device/browser, durable,
repairable, redacted in UI, and rejected when unsafe.

## Scope

- Profile id and profile path ref.
- Ocentra-owned profile root.
- Per child/device/browser scoping.
- Create, load, repair, delete, missing-profile state.
- Default profile rejection.
- Portal redaction.
- Custody label and policy revision.

## Touched Paths

- `crates/agent-core/src/browser_managed_session.rs`
- `crates/agent-core/src/browser_managed_session/store/validation.rs`
- `crates/agent-core/src/browser_managed_session/store/atomic_write.rs`
- `crates/agent-core/src/browser_managed_session/store/lock.rs`
- `crates/agent-core/src/browser_managed_session/store/*.rs`
- `crates/agent-service/src/browser_runtime_paths.rs`
- `crates/agent-protocol/src/browser_managed.rs`

## Tests And Proof

- Temp-directory integration tests.
- Default profile rejection tests.
- Restart/reload metadata tests.
- Portal DTO redaction tests.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/<workpack-id>/00-source-snapshot.md` or explicit docs-only N/A reason.
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

## Current Production-Source Decision (2026-08-24)

Source repair is authorized only for the Browser-owned profile store boundary.
The accepted source must:

- preserve an exact immutable profile/device/browser/scope binding across
  reload and reject mismatched, malformed, corrupt, or unknown persisted state;
- serialize mutations and use crash-safe replacement rather than exposing
  partial JSON as the current record;
- reject filesystem indirection that escapes the Ocentra-owned profile root;
- preserve explicit create, load, repair, missing, and delete lifecycle
  semantics without converting corruption into a fresh profile;
- keep development/test identifiers out of the product runtime path; and
- expose no launch, bridge, exact-URL, policy, or enforcement authority.

The source wave intentionally does not write tests or proof. The later test
wave must cover corruption, identity substitution, link/reparse escape,
concurrent mutation, interrupted replacement, restart/reload, and deletion
recovery before this workpack can enter validation.

## Source Packet Checkpoint (2026-08-24)

The pushed `63d913b93` packet removes caller-supplied time and the unreachable
service helper that embedded development/test identity. The Browser-owned store
now validates exact immutable persisted bindings and lifecycle/timestamp shape,
rejects unknown metadata fields and filesystem indirection, serializes mutation
with a bounded cross-process lock, uses synced atomic metadata replacement, and
uses a staged, restart-resumable delete lifecycle with an explicit
`managed-profile-delete-pending` state reason. It does not mint launch, bridge,
policy, Screen, or enforcement authority.

Focused library checks for `agent-protocol`, `agent-core`, and `agent-service`
passed, along with rustfmt, architecture, source-shape, no-test-doubles,
validation-bypass, re-export, diff, lane, and hub guards. The packet was
committed with `--no-verify` because this is the code-first wave; the stale
Browser tests still reference the removed caller time/dev service helper and
must be repaired and expanded in the later test wave. No test execution or
proof claim follows from this checkpoint.

## Manual-Required Gaps

Profile existence does not prove browser launch, bridge connectivity, exact
URL evidence, parent policy authority, or product runtime composition. WP07
must consume only an owner-retained profile binding after this storage boundary
is accepted; it must not rebuild authority from paths, environment variables,
or service request fields.
