# Parent Desktop Runtime Package Plan

This folder is the single working plan location for D lane: parent desktop
runtime shell, local service connectivity, package proof, platform claim matrix,
support diagnostics, and release-readiness boundaries.

- [Parent Desktop Runtime Package 20-Step Plan](parent-desktop-runtime-package-20-step-plan.md)
- [Parent Desktop Runtime Package Test Blueprint](parent-desktop-runtime-package-test-blueprint.md)
- [Runtime Package Requirements Guide](runtime-package-requirements-guide.md)

The rule remains:

```text
Package proves the shell. Service proves the child device. Release claims name proof.
```

## Where We Are

- `apps/parent-desktop` is the Tauri parent desktop shell candidate. It packages
  the parent portal for parent-owned devices and connects through typed service
  paths.
- D has implementation/validation work in progress on
  `codex/parent-desktop-runtime-package-proof`.
- D is currently blocked on `docs/product-capability-checklist.md` because A
  holds a checklist lock. The implementation path should not be reduced to that
  blocker; D still owns the full desktop/package/runtime proof program.
- Production signing, store distribution, notarization, mobile child-agent
  parity, remote relay, and privileged platform behavior remain unclaimed until
  real proof exists.

## Where We Want To Be

Ocentra Parent needs a parent desktop/package subsystem that:

- packages the parent portal without turning the desktop shell into a
  child-device agent;
- connects to local, LAN, relay, cache, or parent-owned storage paths through
  typed contracts;
- exposes controller, observer, route, custody, package, update, and support
  states clearly;
- proves real package mechanics through build/smoke tests and CI artifacts;
- keeps signing, stores, notarization, mobile, and privileged platform claims
  manual-required or scaffold-only until platform proof exists;
- gives primary a branch that can be reviewed, PR-created, CI-watched, merged,
  and reported without guessing what is product-ready.

## Coverage Check Against Product Docs

This plan was grounded in:

- `docs/features/production-distribution-support.md`
- `docs/features/child-agent-local-service.md`
- `docs/features/remote-lan-mobile-platforms.md`
- `docs/expectations/platforms.md`
- `docs/expectations/release-installer.md`
- `docs/expectations/real-evidence-proof.md`
- `apps/parent-desktop/README.md`
- `crates/agent-service/README.md`
- `crates/agent-protocol/README.md`

The repeated theme is platform honesty. CI/package preview can prove mechanics;
real machines, signing credentials, stores, entitlements, and OS permissions
prove privileged product claims.

## Parallel Coordination Rules

- D owns this parent desktop/runtime/package program until primary merges or
  retargets it.
- D should continue implementation/proof work while the checklist row is locked,
  then reconcile the checklist after A/primary releases the conflicting lock.
- Do not split D into tiny packaging tasks. Use the workpacks below as the
  durable backlog and report which workpacks changed.
- Parent desktop may display and route state. It must not execute capture,
  local AI, policy evaluation, enforcement, timers, or scripts.
- Every `DONE` report must name workpacks, touched paths, validation, product-doc
  updates, package proof, known platform gaps, and whether checklist sync is
  complete or blocked.

## Workpack Checklist

| Step | Workpack                                                                                       | Target State                                                                             |
| ---- | ---------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| 01   | [Tauri shell contract boundary](workpacks/01-tauri-shell-contract-boundary.md)                 | Desktop shell owns packaging/connectivity, not child-agent authority.                    |
| 02   | [Local service connection command](workpacks/02-local-service-connection-command.md)           | Tauri command checks real Rust service state through typed paths.                        |
| 03   | [LAN route and controller state](workpacks/03-lan-route-and-controller-state.md)               | Desktop shows local/LAN/controller/observer route state honestly.                        |
| 04   | [Parent observer read-only state](workpacks/04-parent-observer-read-only-state.md)             | Observer state cannot write or imply controller authority.                               |
| 05   | [Custody and source labels](workpacks/05-custody-and-source-labels.md)                         | Desktop surfaces label live local, LAN, relay, cache, parent-owned, or unavailable data. |
| 06   | [Parent mobile bridge boundary](workpacks/06-parent-mobile-bridge-boundary.md)                 | Parent mobile scaffold is separated from child mobile agent claims.                      |
| 07   | [Windows installer and preview](workpacks/07-windows-installer-and-preview.md)                 | Windows package/launch smoke is proved without claiming signing.                         |
| 08   | [Cross-platform package preview matrix](workpacks/08-cross-platform-package-preview-matrix.md) | Windows/macOS/Linux/Android/iOS package states are split by proof level.                 |
| 09   | [Update channel and rollback scaffold](workpacks/09-update-channel-and-rollback-scaffold.md)   | Update/rollback states are represented honestly before production signing.               |
| 10   | [Signing, notarization, and store claims](workpacks/10-signing-notarization-store-claims.md)   | Credentials/store claims remain manual-required until artifacts exist.                   |
| 11   | [Support diagnostics and redaction](workpacks/11-support-diagnostics-and-redaction.md)         | Support bundles/copy output are useful and redacted.                                     |
| 12   | [Privacy and release docs](workpacks/12-privacy-and-release-docs.md)                           | Public/support docs say what is packaged, signed, stored, or unavailable.                |
| 13   | [Desktop launch smoke](workpacks/13-desktop-launch-smoke.md)                                   | Tauri launch proof verifies the shell starts and reaches service state.                  |
| 14   | [Tauri build and dev scripts](workpacks/14-tauri-build-and-dev-scripts.md)                     | Managed scripts build/dev/package without taking over unrelated ports.                   |
| 15   | [Platform capability matrix](workpacks/15-platform-capability-matrix.md)                       | Platform capability rows match docs, proof JSON, and release notes.                      |
| 16   | [Release branch boundary](workpacks/16-release-branch-boundary.md)                             | `main` previews only; production release remains explicit promotion.                     |
| 17   | [GitHub Actions artifact proof](workpacks/17-github-actions-artifact-proof.md)                 | CI artifact status is checked and reported before package claims.                        |
| 18   | [Manual platform proof runbook](workpacks/18-manual-platform-proof-runbook.md)                 | Real host/device proof requirements are explicit.                                        |
| 19   | [Product checklist and feature doc sync](workpacks/19-product-checklist-feature-doc-sync.md)   | Feature docs/checklist update when runtime/package proof status changes.                 |
| 20   | [PR, CI, and rollout gate](workpacks/20-pr-ci-rollout-gate.md)                                 | PR body, validation, CI, merge notes, and post-merge report carry detailed scope.        |

## Blocker Rule

If `docs/product-capability-checklist.md` remains locked by A, D should report
the exact desired row update and continue non-overlapping package/runtime proof.
Primary reconciles checklist status during integration after lock conflicts are
resolved.
