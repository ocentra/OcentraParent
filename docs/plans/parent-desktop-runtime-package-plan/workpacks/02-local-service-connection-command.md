# 02 Local Service Connection Command

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `02 Local Service Connection Command`
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

Parent desktop proof needs to show real service connectivity rather than a Vite
backend assumption.

## Where We Want To Be

The Tauri command reports local service availability, route state, controller
state, and package metadata through typed output.

## Decision Tree

| If the assignment touches...   | Read next                                                                   | Required proof                               |
| ------------------------------ | --------------------------------------------------------------------------- | -------------------------------------------- |
| Local service endpoint/command | nearest protocol/service command source and `../TEST_PROOF_EXPECTATIONS.md` | typed response and negative connection proof |
| Account/session gate           | `../../account-identity-family-plan/AGENTS.md`                              | authN/session state boundary proof           |
| Setup/pairing readiness        | `../../setup-install-provisioning-plan/AGENTS.md`                           | first-run handoff proof                      |
| LAN/local child route          | WP03 LAN route and controller state                                         | route-state transition proof                 |

## Expected Response Shape

- Connection state: `available`, `unavailable`, `degraded`, `timeout`, `unauthorized`, `versionMismatch`, or `manualRequired`.
- Source state: local service path, route type, freshness, custody label, and proof tier.
- Parent authority state: observer/controller role, account/session status, household/device binding status, and missing-permission reason.
- Package state: app version, build channel, update channel, platform, and support diagnostic capability.
- No child-private data, tokens, raw logs, raw evidence, or policy internals in the shell response.

## Requirement Checklist

- [ ] Connect to configured Rust service path.
- [ ] Return unavailable/degraded state when service is missing.
- [ ] Include controller/observer/source state where available.
- [ ] Avoid hardcoded success responses.
- [ ] Add focused script tests.
- [ ] Prove unauthorized, timeout, version mismatch, and stale service states.
- [ ] Preserve account/session and device-binding boundary without implementing auth here.
- [ ] Redact logs and avoid raw child/evidence payloads.

## Acceptance And Proof

The proof script shows available and unavailable service outcomes.

Expected proof names:

- `parent-desktop.local-service.available`
- `parent-desktop.local-service.unavailable`
- `parent-desktop.local-service.unauthorized-negative`
- `parent-desktop.local-service.timeout-retry`
- `parent-desktop.local-service.version-mismatch`
- `parent-desktop.local-service.redaction-proof`

Proof must include command output, response fixture/snapshot, port/path, auth/session state used, and explicit missing-service behavior.

## Failure Conditions

- Do not hardcode available/success for demo UI.
- Do not store or print access tokens, refresh tokens, child identifiers, raw evidence, or raw logs.
- Do not claim the child service exists or is healthy unless a real service response/proof exists.

## Parallel Ownership Notes

This does not prove the child service itself; it proves the parent shell bridge.
