<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `Parent Client Runtime Distribution Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: package/runtime readiness without matching artifacts.

<!-- /agent-capsule -->

# Parent Client Runtime Distribution Test Proof Expectations

## General rule

Use focused commands first. Broader validation is allowed only after focused commands pass or a precise blocker is recorded.

If a required package/test path does not exist yet, write a blocker artifact and leave the checklist row open.

Run through `npm run agent:run --` when collecting proof if the wrapper is available.

## Common command families

Use only the subset relevant to the selected workpack:

```bash
npm run build --workspace @ocentra-parent/portal
npm run test --workspace @ocentra-parent/portal
npm run test:e2e --workspace @ocentra-parent/portal
npm run test:parent-mobile-shell-runtime-proof
npm run test:parent-mobile-package-source-artifact-proof
npm run test:parent-android-package-proof
npm run test:parent-mobile-service-bridge
npm run test:parent-mobile-controller-observer-handoff
npm run test:parent-desktop-release-support-proof
npm run release:package:parent-android
npm run release:package:parent-ios
npm run lint:architecture -- --files apps/portal packages/portal-domain scripts/dev scripts/release docs/plans/parent-desktop-runtime-package-plan
```

## Command ownership notes

- `apps/portal` owns parent web source/projection proof when selected.
- `portal-domain` owns public portal contracts/projections when selected.
- `scripts/dev` owns parent desktop dev launch anchors when selected.
- `scripts/release` owns selected parent package/build proof helpers.
- Setup, child runtime, device trust, account, payment, policy, remote access, data custody, and portal UX scopes run only when the selected workpack names the handoff.

## Parent client E2E meaning

Do not use one proof family to claim the whole parent-client path. For this plan, E2E has separate meanings:

```text
web portal distribution E2E: portal build -> route/auth/cache/env proof -> hosted/deploy state or blocker.
desktop shell/package E2E: desktop launch/package artifact -> service/degrade state -> artifact hash -> no product-readiness claim.
Android parent package E2E: parent Android package source/build/install state -> device/store/manual-required state.
iOS parent package E2E: parent iOS package source/build/install state -> simulator/device/store/manual-required state.
local service route bridge E2E: parent client route request -> local service response/degrade state -> no setup or child-runtime authority claim.
signing/store/notarization E2E: artifact -> platform signing/notarization/store/provisioning state -> manual-required gaps.
update/rollback E2E: version/channel/checksum/SBOM -> update path -> rollback/teardown proof.
launch smoke E2E: selected artifact/platform launch -> expected health/degrade state -> no broader readiness claim.
setup handoff E2E: package artifact -> setup handoff request/response contract -> setup-owned readiness remains separate.
release gate E2E: accepted proof roots + carried blockers -> release claims allowed/blocked -> no-claim boundary.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## Expected proof focus by workpack

| Workpack | Expected proof focus |
| --- | --- |
| WP01 | canonical parent-client scope and setup handoff boundary |
| WP02 | parent web build, route, auth/cache/env separation, no child-agent execution claim |
| WP03 | desktop shell/package, local service bridge, launch smoke, no product-readiness overclaim |
| WP04 | real parent Android package build/artifact proof, explicit install/device state, and manual-required/store blockers |
| WP05 | parent iOS package/build/install state and manual-required/store blockers |
| WP06 | parent client route bridge and local-service boundary without setup-complete claim |
| WP07 | signing/store/notarization matrix by artifact/platform |
| WP08 | update channel, rollback path, checksum, SBOM proof |
| WP09 | launch smoke matrix by artifact/platform and manual-required gaps |
| WP10 | setup handoff request/response contract only |
| WP11 | proof/CI/release gate and product-status wording |

## Structured harness logging expectations

Product/runtime-safe logging:

```text
redact credentials, signing secrets, provisioning secrets, package signing material, store tokens, account data, device private identifiers, and support-private diagnostics unless a selected proof explicitly allows a bounded field
log workpack, artifact kind, platform, package state, signing state, store state, notarization state, launch state, route bridge state, setup handoff state, update state, rollback state, manual-required note, and no-claim boundary when safe
separate web, desktop, Android, iOS, route bridge, setup handoff, child runtime, device trust, signing/store, update/rollback, launch smoke, and release gate states
never treat scaffold logs, launch logs, package metadata, or CI logs as readiness proof without selected artifact proof and no-claim boundaries
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, artifact kind, platform, exit code, result, artifact pointer, diagnostics summary, manual-required note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## Required negative states

```text
web build is not production account portal readiness
launch smoke is not desktop product readiness
mobile scaffold is not mobile platform support
installer/package artifact is not setup complete
route bridge is not child-agent execution authority
unsigned/unnotarized/unpublished artifacts remain manual-required
update channel without rollback/checksum/SBOM proof is blocked
CI success is not release proof without artifact, negative case, and rollback/teardown evidence
```

## Proof storage

Proof artifacts live under:

```text
output/parent-client-runtime-distribution-plan-proof/<workpack-id>/
```

## Failure conditions

- Do not mark DONE or PR_READY until code, tests, validation, and proof are complete for the selected slice.
- Do not store proof inventories inside this plan folder.
- Do not claim child agent runtime distribution from parent client packaging work.
