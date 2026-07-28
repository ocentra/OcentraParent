<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Network Plan Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Network Plan Proof Index

## Deterministic proof root

```text
output/network-plan-proof/<workpack-file-stem>/
```

## Active slice proof root

```text
docs/proof/network-plan/01-foundation-contracts-and-eventing.md
```

## Current audit truth

- `docs/proof/network-plan/01-network-foundation-shim-cleanup.md` and its output root retain the closed 2026-06-17 frontage-cleanup receipt.
- `docs/proof/network-plan/01-foundation-contracts-and-eventing.md` is the active tracked WP01 contract/eventing validation receipt.
- The active receipt proves only its reviewed contract/runtime slice; broader WP01 and sibling workpack proof bundles remain open.
- `test-results/` currently contains no network-plan proof artifacts in this checkout.
- Proof scripts exist under `scripts/test/`, but script presence alone is not proof completion.
- Stale doc references to `docs/proof/network-plan/` or `output/network-plan-proof/` must be rebuilt or removed before a row can be treated as proved.

## Real proof inputs that currently exist

```text
scripts/test/network-*.mjs
scripts/test/eventing-network-*.mjs
packages/network-domain/tests/unit/*.test.ts
crates/agent-protocol/src/network*_tests.rs
crates/agent-core/src/network*_tests.rs
crates/agent-service/src/network*_tests.rs
crates/ocentra-network-evidence/src/tests/*.rs
apps/portal/tests/live-activity-network-flow.test.ts
apps/portal/e2e/network-evidence-drawer-proof.spec.ts
```

## Host proof routing

- Windows proof is expected locally when a selected row requires it.
- Android proof is expected locally when a selected row requires it, using the emulator and the already-synced Samsung device when reachable.
- Linux proof is expected locally through WSL and/or Docker when a selected row requires it.
- Real macOS and iOS proof is an external-platform constraint from this Windows host and must not be faked.

## Required universal proof files

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
16-validation-commands.log
```

## Command log format

```text
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <path or n/a>
notes: <short note>
```

If blocked:

```text
blocker:
required environment:
why this does not prove completion:
next command:
```

## Structured proof metadata

For new proof artifacts and new command-log entries, include structured metadata when available:

```text
plan: network-plan
workpack: <workpack id and name>
owner: schema-domain | network-domain | ocentra-network-evidence | agent-protocol | agent-core | agent-service | portal | eventing-handoff | browser-handoff | screen-handoff | ai-handoff | policy-handoff | enforcement-handoff | lan-handoff | data-custody-handoff | docs-only
platform: windows | android | linux | macos | ios | cross-platform | n/a
evidence_kind: network-flow | dns | domain | packet | pcap | process | browser-correlation | app-correlation | screen-fallback | cascade | adapter-proof | policy-handoff | ai-audit | risk-budget | catalog-reference | n/a
source_kind: schema | fixture | pcap-replay | live-capture | service-read-model | portal-projection | catalog | lab-execution | manual | n/a
adapter_kind: dns | firewall | wfp | android-vpn | apple-network-extension | linux-mechanism | none | n/a
domain_attribution_state: attributed | unknown | ambiguous | stale | not-tested | n/a
process_attribution_state: attributed | unknown | ambiguous | stale | not-tested | n/a
browser_exact_url_state: not-claimed | browser-handoff | proved-by-browser | blocked | n/a
screen_fallback_state: not-claimed | screen-handoff | proved-by-screen | blocked | n/a
ai_runtime_state: not-claimed | ai-handoff | fixture-only | runtime-proved | blocked | n/a
policy_handoff_state: not-tested | mapped | rejected | policy-owner-required | n/a
enforcement_authority_state: not-claimed | enforcement-handoff | authority-proved | blocked | n/a
intervention_state: not-tested | capability-only | lab-proved | production-proved | unavailable | manual-required | n/a
rollback_state: not-tested | proved | blocked | manual-required | n/a
private_content_claim_state: not-claimed | blocked | explicitly-proved-by-owner | n/a
manual_required_note: <manual-required gap or n/a>
run_id: <wrapper run id or n/a>
command_id: <wrapper command id or n/a>
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <stdout/stderr artifact pointer, proof file, test result path, or n/a>
diagnostics_summary: <short unique failure or proof summary>
no_claim: <what this result does not prove>
```

The command log is a compact index, not a raw terminal transcript. Store raw command output, packet fixtures, test reports, screenshots, or long failure dumps under artifact paths and reference them by pointer. If no wrapper exists, write `run_id: n/a` and `command_id: n/a`; do not omit the proof row.

## Required proof themes

```text
source/custody labels
unknown-attribution state
adapter/platform limits
no private network-content claim without explicit product/legal proof
portal visibility proof when UI changes
service/protocol proof when runtime changes
manual-required states
```

## Current rule

Do not claim a proof complete from a planned artifact path. A proof becomes current only after:

1. the focused validation command runs from the real owning surface;
2. the resulting artifact exists in the committed proof location or the blocker is written explicitly;
3. the workpack/checklist row points at that exact artifact or blocker.

The historical `network-foundation-shim-cleanup` receipt remains bounded to frontage cleanup. The active WP01 receipt must stay aligned with `PLAN_PROOF_MANIFEST.md`, `PLAN_STATE.md`, and `PLAN_HEALTH.md`.

## No-claim language

Do not claim:

```text
live capture ready
exact URL ready
private content proof ready
AI runtime ready
policy decision ready
enforcement authority ready
production adapter ready
portal product surface ready
control catalog implemented
PR_READY
```

unless the selected workpack proof root proves the exact claim or records the exact blocker.
