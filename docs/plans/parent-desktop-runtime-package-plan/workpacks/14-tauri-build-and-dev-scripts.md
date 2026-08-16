# 14 Tauri Build And Dev Scripts

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `14 Tauri Build And Dev Scripts`
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

Managed scripts exist for local dev and package checks. They must stay safe for
parallel lanes.

## Where We Want To Be

Tauri build/dev/package scripts use repo defaults and lane-specific ports without
taking over unrelated Ocentra processes.

## Decision Tree

| If the assignment touches... | Read next                                            | Required proof              |
| ---------------------------- | ---------------------------------------------------- | --------------------------- |
| Dev server command           | `../../agent/LOCAL_DEV_PORTS.md` and package scripts | port/lane proof             |
| Tauri build/package command  | package manifest and Tauri config                    | build log and artifact path |
| Local service bridge         | WP02 local service connection command                | service route proof         |
| CI command                   | workflow/script file named by assignment             | CI or local equivalent log  |

## Script Rules

- Commands must be repo-owned and documented; avoid ad hoc shell-only recipes.
- Ports must be lane-safe and must not kill unrelated Ocentra Games, portal, or service processes.
- Dev and package commands must distinguish parent desktop shell, parent web portal, child service, and public family site.
- Scripts must expose enough output for proof: version, target, artifact path, port, environment, and skipped optional pieces.
- If a required toolchain is missing, the result is `manualRequired` or `blockedByToolchain`, not a fake pass.

## Requirement Checklist

- [ ] Use managed repo scripts.
- [ ] Respect lane agent/portal ports.
- [ ] Avoid generic port assumptions.
- [ ] Document useful commands in README.
- [ ] Validate script changes.
- [ ] Separate desktop shell scripts from public family site and child service scripts.
- [ ] Preserve exact command output in proof.
- [ ] Record missing toolchain or platform-specific manual requirement.

## Acceptance And Proof

Script validation passes and reports mention exact commands.

Expected proof names:

- `parent-desktop.scripts.dev-lane-port-proof`
- `parent-desktop.scripts.tauri-build-log`
- `parent-desktop.scripts.package-artifact-proof`
- `parent-desktop.scripts.service-bridge-command-proof`
- `parent-desktop.scripts.missing-toolchain-manual-required`

Proof must include command, cwd, env/port, exit code, artifact path if any, and skipped-risk note.

## Failure Conditions

- Do not kill or reuse unrelated project ports/processes.
- Do not make desktop scripts responsible for public website deployment, child-agent install, or backend billing/auth infrastructure.
- Do not document commands that only work on one developer machine unless they are labelled local/manual proof.

## Parallel Ownership Notes

Do not alter Ocentra Games or unrelated project ports/processes.
