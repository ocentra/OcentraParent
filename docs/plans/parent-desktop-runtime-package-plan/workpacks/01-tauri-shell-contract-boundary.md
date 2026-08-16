# 01 Tauri Shell Contract Boundary

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `01 Tauri Shell Contract Boundary`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

The Tauri app is the parent desktop shell candidate. It must not drift into
child-device authority.

## Where We Want To Be

The desktop shell owns packaging, launch, connection, and display of typed
service state only.

## Decision Tree

| If the assignment touches...       | Read next                                                                       | Do not read by default                                           |
| ---------------------------------- | ------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| Tauri command surface              | nearest Tauri command/service bridge source and `../TEST_PROOF_EXPECTATIONS.md` | child capture, AI, policy, timer, or enforcement implementations |
| Parent shell state display         | portal/desktop shell route named by the assignment                              | all portal UX workpacks                                          |
| Service unavailable/degraded state | local service connection workpack and exact proof row                           | LAN, remote relay, or child runtime plans                        |
| Packaging or installer entry       | WP07 Windows installer and setup-install handoff                                | signing/store docs unless packaging claim needs them             |

## Expected Contract Shape

- Shell may request typed parent-visible state from the local service.
- Shell may display connection, account/session, device, update, diagnostics, and package status.
- Shell must not execute capture, AI classification, policy compilation, timer scheduling, enforcement, or network/app/browser/tracking control directly.
- Shell must surface `available`, `unavailable`, `degraded`, `manualRequired`, and `unsupported` states without converting them into success.
- Shell must preserve source labels so parent UI cannot confuse local preview state with live child-device proof.

## Requirement Checklist

- [ ] Keep capture, AI, policy, enforcement, and timers out of Tauri commands.
- [ ] Use typed service/protocol output.
- [ ] Document shell ownership in README/docs.
- [ ] Test command output boundaries.
- [ ] Label unavailable service states.
- [ ] Prove shell cannot bypass service authority for child-device work.
- [ ] Record exact package/shell files inspected before claiming boundary coverage.

## Acceptance And Proof

Tests and docs show the shell connects to child-agent state instead of executing
child-agent work.

Expected proof names:

- `parent-desktop.tauri-command-boundary.audit`
- `parent-desktop.service-state.shape-test`
- `parent-desktop.unavailable-state.render-proof`
- `parent-desktop.no-child-authority-negative`
- `parent-desktop.source-labels.proof`

Proof must include command/test output, inspected file list, rejected command examples or negative cases, and any missing source/package path.

## Failure Conditions

- Do not claim child-agent runtime, capture, AI, policy, timer, or enforcement work from a shell command.
- Do not call a rendered desktop state "live" unless the service read-model and child-device proof exist.
- Do not add implementation recipes; state the expected boundary and proof only.

## Parallel Ownership Notes

D owns this boundary. A/B own enforcement/LAN runtime claims.
