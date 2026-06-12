# 13 Desktop Launch Smoke

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `13 Desktop Launch Smoke`
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

The desktop shell needs focused launch proof to support package claims.

## Where We Want To Be

Launch smoke proves the shell starts, handles service available/unavailable
states, and does not claim backend authority.

## Decision Tree

| If the smoke is...            | Required proof                                                     | Stop condition                                   |
| ----------------------------- | ------------------------------------------------------------------ | ------------------------------------------------ |
| Headless/CI launch            | build log, process start result, noninteractive smoke output       | stop before claiming rendered UI                 |
| Local rendered desktop launch | screenshot/log path, version, account/session state, service state | stop before claiming child-device runtime        |
| Service available             | typed service response fixture or live local service proof         | stop before claiming remote child proof          |
| Service unavailable/degraded  | visible unavailable state, retry/support copy, log redaction       | stop before treating unavailable as failure-free |

## Expected Smoke Coverage

- App starts without crashing in the selected mode.
- Shell reports version/build/channel and environment.
- Account/session state is visible or explicitly unavailable.
- Local service connection state is visible and source-labelled.
- Pairing/setup route is reachable or explicitly manual-required.
- Diagnostics/log export is redacted and does not expose child/private data.

## Requirement Checklist

- [ ] Build or launch desktop shell where feasible.
- [ ] Check service available state.
- [ ] Check service unavailable/degraded state.
- [ ] Record commands and environment.
- [ ] Avoid interactive-only proof as the sole artifact.
- [ ] Capture version/build/channel and account/session state.
- [ ] Capture setup/pairing route availability or manual-required state.
- [ ] Save screenshot/log proof when rendered UI is part of the claim.

## Acceptance And Proof

Focused smoke tests and reports show package launch behavior clearly.

Expected proof names:

- `parent-desktop.launch-smoke.ci`
- `parent-desktop.launch-smoke.local-rendered`
- `parent-desktop.service-available.state-proof`
- `parent-desktop.service-unavailable.state-proof`
- `parent-desktop.diagnostics-redaction.proof`
- `parent-desktop.setup-route.visibility-proof`

Proof must include command output, screenshot/log path where applicable, OS/version, app build identity, selected account/session state, and missing-proof notes.

## Failure Conditions

- Do not claim launch smoke proves installer, signing, update, child service, or platform adapter readiness.
- Do not rely on a human-observed window without saved artifact when making PR/DONE claims.
- Do not expose secrets, account tokens, child identifiers, or raw logs in proof artifacts.

## Parallel Ownership Notes

If GUI launch is not possible in CI, record local/manual proof requirements.
