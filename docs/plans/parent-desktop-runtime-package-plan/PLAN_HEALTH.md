# Parent Client Runtime Distribution Plan Health

Health: canonical parent client distribution route exists; artifact parity and proof remain open.

Known risks: scaffold being overclaimed as parity, desktop launch being overclaimed as product readiness, signing/store status being hidden, setup being merged with package proof, child-agent claims leaking into this plan, and update/rollback proof missing.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the matching checklist/proof row if one exists.
- Update `PLAN_STATE.md` and `NEXT_ACTIONS.md` if current state changes.
- Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
- Do not claim READY from scaffold, preview output, package metadata, launch smoke, web build, or CI success alone.
- Do not claim READY from unsigned, unnotarized, unpublished, side-loaded, simulator-only, or manual-required artifacts.
- Do not claim setup completion from package creation or route bridge proof.
- Do not claim child runtime readiness from parent client package work.
- Do not claim update readiness without checksum, SBOM, rollback, and stale update handling.
- Do not claim release readiness until WP11 consumes accepted proof roots or exact carried blockers.

## Known healthy boundaries

This plan intentionally separates:

```text
parent web portal distribution
parent desktop shell/package
parent Android package
parent iOS package
parent local-service route bridge
signing/store/notarization
update/rollback
launch smoke
setup handoff
proof/CI/release gate
```

Do not collapse those boundaries.

## Known incomplete areas

```text
artifact matrix
web distribution contract
parent desktop package artifact proof
parent Android device/store proof
parent iOS device/store proof
route bridge contract proof
signing/store/notarization matrix
update/rollback/checksum/SBOM proof
launch smoke matrix
setup handoff contract
release gate aggregation
WP01 status reconciliation between workpack text and index/proof state
```

## High-information-density gate

### Scope and ownership

- Scope and ownership: this file governs documentation routing, state, and proof expectations for `parent-client-runtime-distribution-plan`.
- Ownership path: this plan is coordinated via `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `WORKPACK_FAMILIES.md`, and the selected workpack files.

### State

- Current state: the folder path is preserved for compatibility, but the canonical scope is now parent client runtime distribution.
- Current action: keep this file and `PLAN_STATE.md` aligned before any DONE or PR_READY claim.

### Decision routes and failure controls

- Decision route: follow the AGENTS landing decision, the selected workpack path, and the proof inventory referenced in this file.
- Failure controls: do not claim completion when handoff routes are missing, checklist/workpack states diverge, or known risks remain unmitigated with no explicit deferral.

### Proof mapping

- Required proof before READY: explicit links from workpack/checklist rows, proof artifacts named in the proof inventory, and cross-plan handoff notes in AGENTS/NEXT_ACTIONS.
- At minimum, align `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, and the assigned plan workpacks before READY.

## PR-ready rule

The whole plan is PR-ready only when WP11 consumes or blocks every earlier proof root and updates PLAN_STATE.

A partial PR may be ready only when one selected workpack is closed with proof artifacts, command logs, blocker classification, no-claim language, and remaining open workpacks listed.
