# Checklist Index

> **Live-code audit:** [Plan state](PLAN_STATE.md) and the [Project Progress Matrix](../../PLAN_CODE_STATUS_MATRIX.md) separate committed source, missing source, expected test source, runtime reachability, and strict completion. This checklist is intentionally unchecked; mapped files and historical proof do not close rows.

Status: source routing reconciled; implementation and test rows open.

## Production-source wave

- [ ] WP01 route/index truth remains aligned with the executable graph.
- [ ] WP02 Windows package starts with current trust and exposes authenticated ingress/health through canonical child-owned package identities.
- [ ] WP03 macOS package identity, signing/notarization ownership, trusted startup, health, and lifecycle source exist.
- [ ] WP04 Linux package identity, fail-closed lifecycle, trusted startup, health, signing/feed, and cleanup source exist.
- [ ] WP05 Android JNI startup consumes current trust and owns authenticated ingress, usable health, platform authority, and removal integration.
- [x] WP06 actual iOS app/project/bundle/scheme/release identity is the canonical child identity while capability limits remain explicit. Reviewed source: `c71becbcfd4f07eb98a118f10dbf261320f6b54e`.
- [ ] WP07 health-aware cross-platform supervision, bounded restart/backoff, deliberate stop, and teardown source exist.
- [ ] WP08 a production Account-authority caller and platform cleanup callbacks/receipts exist without child-minted authority.
- [ ] WP09 live handoff/update composition and platform signing/store/update owners exist.
- [ ] WP10 trusted startup, authenticated ingress, external health, durable handoff delivery/replay, and live updater consumption exist.
- [ ] WP11 an executable aggregate release gate exists and fails closed on every open/manual-required path.

## Expected test-source wave

- [ ] WP02-WP04 child-labelled desktop package/startup/health/lifecycle/cleanup tests exist.
- [ ] WP05 trust-currentness, JNI/foreground lifecycle, ingress, health, removal, and device-authority tests exist.
- [ ] WP06 child identity and iOS capability-limit tests exist.
- [ ] WP07 kill/reboot/manager-restart/disable/teardown/loop-guard tests exist per platform.
- [ ] WP08 authority mismatch/replay/restart and platform cleanup callback/idempotency tests exist.
- [ ] WP09 updater handoff/install/restart and platform signing/store/device-owner tests exist.
- [ ] WP10 trust-source, authenticated ingress, health, handoff replay/expiry, updater callback, and crash/restart tests exist.
- [ ] WP11 aggregate negative fixtures and release-blocker tests exist.

## Validation and completion wave

- [ ] Focused formatter, architecture, and static gates pass for each source packet.
- [ ] Focused crate/package tests pass after the test-source wave.
- [ ] Suitable desktop hosts and physical/simulator mobile targets produce the required lifecycle evidence.
- [ ] Enforcer and graph validation agree with code/test topology and dependency order.
- [ ] Plan proof is regenerated once for the consolidated implementation and tests.
- [ ] Precommit passes once on the consolidated branch.
- [ ] One PR receives fresh CI, review, and merge approval.

Proof artifacts belong under `output/child-agent-runtime-distribution-plan-proof/<workpack-file-stem>/`, never in this plan folder.
