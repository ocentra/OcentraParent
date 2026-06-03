# 16 New App And Unknown App Approval Flow

Sources: [full scope plan](../v0-5-native-apps-full-scope-plan.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
`docs/expectations/app-install-purchase-approval.md`.

## Where We Are

App approval and policy foundations exist, but native new/unknown app detection
and parent approval are not yet a complete evidence-backed workflow.

## Where We Want To Be

New inventory apps, unknown runtime processes, portable executables, installers,
and ambiguous app candidates can create parent review/approval requests with
evidence refs. Enforcement remains capability-gated and manual-required when no
adapter proof exists.

## Scope

- New app detected candidate.
- Unknown app launched candidate.
- Portable app launched candidate.
- Installer/updater detected candidate.
- Approval request, allow once, allow always, ask child, block/manual-required,
  expiry, and audit states.
- Child-facing request flow where applicable.

## Touched Paths

- `packages/parent-domain/src/app-game-control-approval-flow.ts`
- `packages/parent-domain/src/app-game-control-authority.ts`
- `packages/parent-domain/src/app-game-control-authority-rules.ts`
- `packages/parent-domain/tests/app-game-unknown-approval-flow.test.ts`
- `packages/parent-domain/tests/app-game-control-authority.test.ts`
- `docs/plans/app-game-plan/workpacks/17-unknown-app-game-approval-flow.md`
- `output/app-plan-proof/16-new-app-and-unknown-app-approval-flow`
- `output/app-game-plan-proof/17-unknown-app-game-approval-flow`

## Tests And Proof

- [x] New inventory app creates a `new-inventory-app` candidate.
- [x] Unknown runtime process approval requests carry evidence and child status
      refs.
- [x] Parent approval request includes evidence refs and expiry.
- [x] Allow once expires; persistent allow carries audit-backed replay state.
- [x] Parent block returns `manual-required` if no adapter proof exists.
- [x] Approval survives restart/replay when storage exists through
      `replayable`/`replayed` persistence state and audit refs.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [x] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-app-snapshot.md), [full scope plan](../v0-5-native-apps-full-scope-plan.md), [platform deep dive](../v0-5-native-apps-platform-deep-dive.md), [test blueprint](../v0-5-native-apps-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [x] Confirm this is native/installed-app scope, not browser pages, browser games, or game-specific product semantics unless the source docs explicitly route that handoff.
- [x] Hub lock covers this workpack and exact implementation/docs paths.
- [x] Existing app/game source layout inspected; no parallel app-control truth created.
- [x] Before-state source snapshot recorded in `output/app-plan-proof/16-new-app-and-unknown-app-approval-flow/00-source-snapshot.md`.
- [x] Contracts updated first where this workpack changes behavior.
- [x] Rust/service/portal parity not updated because this slice does not add Rust-crossing, service, or portal payloads.
- [x] Raw evidence artifacts captured as contract-valid approval request/action-result JSON in `05-policy-action-proof.json`; live inventory/runtime/foreground/journal rows are N/A for this contract-only slice.
- [x] Tests/proof listed in this workpack and [test blueprint](../v0-5-native-apps-test-blueprint.md) are implemented or explicitly marked manual-required with reason.
- [x] Required fixtures are present for contract-level approval, replay, and manual-required states; live platform/UI fixtures are N/A with reason.
- [x] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [x] UI snapshots marked N/A because no parent portal or child-facing UI changed.
- [x] Security/no-claim negative proof captured: unknown app/game evidence cannot dispatch, manual-required cannot call adapters, and weak game fallback cannot deny as if proved.
- [x] Manual platform proof marked N/A because no platform claim stronger than observe-only/manual-required was added.
- [x] Platform limitations use capability status language: observe-only, manual-required, or not-claimed until proof exists.
- [x] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [x] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [x] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Approval flows can ask, allow, and record audit state before they can hard block
or launch-control an app on a platform.
