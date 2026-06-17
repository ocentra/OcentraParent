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
docs/proof/network-plan/01-network-foundation-shim-cleanup.md
output/network-plan-proof/01-network-foundation-shim-cleanup/
```

## Current audit truth

- `docs/proof/network-plan/` and `output/network-plan-proof/01-network-foundation-shim-cleanup/` were restored on 2026-06-17 for the active foundation shim cleanup slice.
- The proof root is still only a slice-01 skeleton; broader workpack proof bundles remain missing.
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

For the current `network-foundation-shim-cleanup` slice, the proof root is allowed to stay as a skeleton if the only unresolved item is the single `./network-control-catalog` public-export decision recorded by the slice proof doc.
