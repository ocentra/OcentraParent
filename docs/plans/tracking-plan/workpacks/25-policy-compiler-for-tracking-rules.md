# WP25 Policy Compiler For Tracking Rules

## Purpose

Compile location, geofence, expected-place, nearby-place, stale/offline,
battery, check-in, and escalation targets into parent policy decisions.

## Source Inputs

- `docs/expectations/policy.md`
- `docs/expectations/location-geofence.md`
- `docs/plans/tracking-plan/v0-5-location-ai-safety-analysis-plan.md`

## Target State

Policy decisions cite evidence, rule refs, schedule refs, AI candidate refs
when used, capability state, exception state, and audit refs.

## Tests And Proof

Proof root: `output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/`

- `01-contract-proof.log`
- `09-policy-alert-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`
- `proof-summary.json`
- `test-results/tracking-plan-policy-compiler-proof/proof.json`

## AI Worker Checklist

- [x] Parent policy is final action authority.
- [x] Compile observe, notify, ask child, ask parent ack, live tracking,
      escalate, critical alert, suppress, and manual-required.
- [x] Add deterministic conflict tests.
- [x] Prevent AI-only alert/escalation.
- [x] Preserve dry-run/preview where applicable.

## Where We Are

This workpack has focused contract proof from `codex/tracking-plan-full-scope`
and P1 compiler/evaluator proof from
`codex/tracking-policy-compiler-runtime-proof` under the proof root below. The
runtime proof compiles parent-rule decisions for observe, notify-parent,
ask-child-check-in, parent-acknowledgement request, temporary live tracking,
escalation, critical-alert, suppress/no-action, and manual-required paths. It
also proves AI evidence cannot become final action authority, stale/missing
confirmation/manual-required inputs degrade to non-enforcing outcomes, and
disabled rules compile to no-action. Platform adapters, provider delivery,
notification delivery, UI, physical-device behavior, and production enforcement
remain unclaimed beyond the proof state recorded in `proof-summary.json` and the
implementation checklist.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/25-policy-compiler-for-tracking-rules.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, production-enforcement, and physical-device
  claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch:
      `codex/tracking-policy-compiler-runtime-proof`.
- [x] Touched files: parent-domain compiler and focused tests, proof script,
      product docs, implementation checklist, this workpack doc, proof output,
      and root package script.
- [x] Validation commands and results:
      `cmd /c npm run test:tracking-plan-policy-compiler-proof` passed.
- [x] Proof artifacts under
      `output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/` and
      `test-results/tracking-plan-policy-compiler-proof/proof.json`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, and this workpack doc updated. Product capability checklist
      delta will be written to hub `doc-deltas.ndjson` because
      `docs/product-capability-checklist.md` is reserved.
- [x] Known gaps/manual-required states: provider delivery, notification
      delivery, portal UI, platform adapters, physical Android/iOS behavior,
      production enforcement, and authority-enrolled hard-control proof remain
      proof-gated as applicable.
