<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `Parent Client Runtime Distribution Plan Agent Route`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX.md selects the plan.
> Stop rule: Do not continue into sibling plans unless the selected workpack names a handoff.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

# Parent Client Runtime Distribution Plan Agent Route

Canonical scope: parent client runtime distribution. This folder keeps the historical `parent-desktop-runtime-package-plan` path, but it now covers parent web, parent desktop, parent Android, and parent iOS distribution boundaries. Child agent runtime/package distribution belongs to `child-agent-runtime-distribution-plan`.

Use this file only when `docs/PLAN_INDEX.md` or a hub assignment selects `docs/plans/parent-desktop-runtime-package-plan`.

## High-density execution contract

- Task: work only the assigned slice for this plan.
- Route first from `PLAN_STATE.md`.
- Choose exactly one workpack.
- Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
- Do not inspect sibling plans unless the selected workpack names a handoff.
- Proof must contain command log, negative case, artifact path, updated row, and skipped-risk note when applicable.
- Do not claim desktop, web, Android, or iOS parity from scaffold or launch smoke alone.

## Ownership, Import, And Boundary Contract

This plan owns distribution proof for parent client artifacts. It does not own child-agent runtime distribution, setup journey state, account provider behavior, device trust bootstrap, policy behavior, billing behavior, remote access, data custody, or child capture/enforcement adapters.

Module roles:

```text
apps/portal: parent web portal source/projection surface and web distribution target when selected.
portal-domain: shared portal contracts/projections when the selected parent-client proof needs them.
parent-domain: parent client package/handoff contracts only when public exports exist and the selected workpack names them.
scripts/dev: parent desktop dev launch helpers and local launch proof anchors.
scripts/release: package/build/proof helpers for parent desktop/mobile artifacts when selected.
setup-install-provisioning-plan: setup journey, install readiness, first-run state, and setup-side handoff owner.
child-agent-runtime-distribution-plan: child package/runtime, child package lifecycle, tamper/uninstall, and child-specific artifact owner.
device-trust-bootstrap-plan: trusted-device bootstrap, local sealed trust, and parent presence/approval owner.
account-identity-family-plan, payment-subscription-plan, policy-control-plane-plan, remote-access-plan, and data-custody-storage-plan: sibling owners for their respective product behavior and handoffs.
```

Direct imports are allowed only for explicit public helper surfaces:

```text
apps/portal and portal-domain public build/projection contracts when selected
parent-domain public contract exports when they actually exist and are selected
scripts/dev and scripts/release proof helpers for selected artifact proof
neutral schema, evidence, logging, and protocol helpers that do not own sibling product behavior
```

Forbidden direct imports and claims:

```text
child-agent package/runtime internals imported into parent client distribution proof
setup journey or account/device-trust internals imported to claim package readiness
portal shell UX proof upgraded into distribution proof without build/artifact proof
web build upgraded into production account portal readiness
desktop launch smoke upgraded into desktop product readiness
mobile scaffold upgraded into Android/iOS platform support
package artifact upgraded into setup completion
route bridge upgraded into child-agent authority
unsigned, unnotarized, unpublished, or side-loaded artifacts upgraded into release readiness
CI success upgraded into release proof without artifact, negative case, and rollback/no-claim evidence
```

If parent-client distribution needs setup, account, device trust, payment, policy, remote access, data custody, portal UX, or child runtime behavior, it must use typed handoffs, proof roots, and explicit no-claim boundaries. Do not solve cross-plan behavior by importing another feature owner's runtime internals.

## Research gate

Before DONE or PR_READY, inspect only the selected slice of:

- the current repo parent client surfaces in `apps/portal`
- the parent desktop launch and mobile proof scripts in `package.json`
- official docs for Tauri v2 distribution and security capabilities when desktop packaging is selected
- Windows signing / MSI / MSIX guidance when Windows desktop release is selected
- macOS signing / notarization / Developer ID guidance when macOS desktop release is selected
- Linux package / signing guidance when Linux desktop release is selected
- Android App Bundle / Play App Signing guidance when parent Android release is selected
- iOS signing / provisioning / TestFlight / App Store guidance when parent iOS release is selected
- Cloudflare Pages or Worker deploy docs if the web portal is hosted there

## Decision tree

| If the task is about...                          | Open                                                     |
| ------------------------------------------------ | -------------------------------------------------------- |
| Parent client scope correction and setup handoff | `workpacks/01-parent-client-scope-and-route-boundary.md` |
| Parent web portal distribution                   | `workpacks/02-parent-web-portal-distribution.md`         |
| Parent desktop shell/package                     | `workpacks/03-parent-desktop-shell-package.md`           |
| Parent Android package                           | `workpacks/04-parent-android-package.md`                 |
| Parent iOS package                               | `workpacks/05-parent-ios-package.md`                     |
| Parent local-service route bridge                | `workpacks/06-parent-local-service-route-bridge.md`      |
| Signing/store/notarization matrix                | `workpacks/07-parent-client-signing-store-matrix.md`     |
| Update and rollback                              | `workpacks/08-parent-client-update-rollback.md`          |
| Launch smoke matrix                              | `workpacks/09-parent-client-launch-smoke-matrix.md`      |
| Setup handoff contracts                          | `workpacks/10-setup-handoff-contracts.md`                |
| Proof, CI, and release gate                      | `workpacks/11-proof-ci-release-gate.md`                  |

## Architecture decisions

- Parent client distribution owns parent web, parent desktop, parent Android, and parent iOS artifacts only.
- Child agent package/runtime distribution is a separate plan.
- Setup is a handoff into install state, not package proof.
- Parent web portal is a parent client and must have build, route, auth, cache, and env separation proof.
- Parent desktop shell/package is not product readiness.
- Parent Android and iOS scaffold states remain manual-required until real build, device, and store proof exists.
- Signing, notarization, and store claims must be explicit per artifact.
- Route bridge behavior must stay separate from setup and package claims.

## Handoffs

- `setup-install-provisioning-plan` owns journey, install code, and readiness state.
- `child-agent-runtime-distribution-plan` owns child agent packages and tamper/uninstall proof.
- `device-trust-bootstrap-plan` owns trusted-device bootstrap and local sealed trust.
- `portal-ux-household-surfaces-plan` owns the generic household shell; this plan owns distribution proof for the client artifacts.

## Failure conditions

- Do not claim mobile parity from scaffold-only proof.
- Do not claim desktop readiness from launch smoke alone.
- Do not claim setup completion from package creation alone.
- Do not hide manual-required gaps.
- Do not conflate parent client distribution with child agent runtime distribution.
