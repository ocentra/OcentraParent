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

Generated from the existing `browser-plan` docs. This is the default resume/status file; large historical docs are linked, not embedded.

## Scope

This folder is the single working plan location for managed browser evidence, browser policy authoring, unmanaged browser fallback, browser intervention, and parent-facing browser UI/UX requirements.

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

## Open gaps / missing product runtime

- Browser inventory is not a complete product read model across installed, running, supported, unsupported, managed, unmanaged, packaged, and portable browsers.
- Managed profile store repair, custody, redaction, and restart semantics need explicit workpack proof.
- Active tab proof is still separate from target-list proof. `/json/list` target rows should remain `unknown` active state until focus/activation proof exists.
- Managed browser intervention proof exists as a harness, but product-level warning/blocking still needs typed policy decision refs, journaled action refs, audit refs, child-facing delivery state, and portal proof.
- Unmanaged browser URL evidence remains not claimed. Unmanaged process terminate/warn states exist only as scoped proof paths, not broad OS blocking.
- AppLocker/App Control prevention remains real platform proof/manual-required.
- Firefox, Safari, Android, iOS, extension/native-host, owned browser shell, managed configurations, FamilyControls, and mobile browser support remain separate adapter/platform proof work.

## Checklist summary

- Full checklist: [implementation-checklist.md](implementation-checklist.md) (not default context).
- Checkbox rows detected: 138 total, 97 checked, 41 unchecked.
- Checklist index: [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md).

## Workpack summary

- Workpacks indexed: 24.
- Workpacks with open checkboxes: 0.
- Workpacks with all detected boxes checked: 24.
- Workpacks with no checkbox status: 0.

### Active/open workpacks

- Browser control settings inventory, coverage matrix, schema proposal, policy questionnaire forest, policy settings catalog, and managed/unmanaged browser reference workpacks remain open.
- These open rows are reference/control-routing workpacks, not proof that browser implementation is incomplete or complete by themselves.
- Use `WORKPACK_INDEX.md` to choose the exact assigned row and avoid opening the giant browser inventories by default.

## Default no-read list

- `README_FULL_ORIGINAL.md` unless you need historical full README context.
- Full `implementation-checklist.md` unless `CHECKLIST_INDEX.md` names exact section/row.
- All workpacks; use `WORKPACK_INDEX.md`.
- Source inventories and pasted-content audits unless source ownership is unclear.
- Historical checkpoint/proof docs unless `PROOF_INDEX.md` or the assigned workpack names them.

## Health / consistency

- See `PLAN_HEALTH.md` before claiming the whole plan is complete or stale.
