# Next Actions

## Scope and phase

- Plan owner: `child-agent-runtime-distribution-plan`.
- Current program phase: finish coherent production source packets before writing the missing test source.
- This plan owns child package/runtime distribution, installed child startup, platform lifecycle, updater distribution consumption, and platform removal callbacks.
- Setup owns its producer/UI journey, Device Trust owns current trust material, Account owns current household authority, and parent-client distribution stays separate.
- No test run, proof refresh, precommit, CI, PR, READY, DONE, or release claim is authorized by this routing packet.

## Source-wave order

- [ ] WP06: replace the actual iOS parent product/project/bundle/release identity with the canonical child capability-package identity while preserving every no-daemon/manual-required limit.
- [ ] WP10: after the graph confirms reviewed Device Trust WP01 implementation, compose current trust into shipped child startup, own authenticated external ingress and health, and connect the typed handoff/update consumer. Do not add a reverse dependency on Setup WP07.
- [ ] WP02: finish the Windows package/runtime identity and installed child startup boundary against WP10.
- [ ] WP03: finish the macOS child package identity, lifecycle source, and signing/notarization ownership against WP10.
- [ ] WP04: finish the Linux child package identity and fail-closed service lifecycle against WP10.
- [ ] WP05: compose Android JNI startup with current trust and platform lifecycle/authority boundaries against WP10.
- [ ] WP07: add health-aware lifecycle/supervision truth after WP02-WP06 and WP10 source exist.
- [ ] WP08: connect Account-owned current household authority to child revocation and platform cleanup callbacks after WP07/WP10; never let the child mint authority.
- [ ] WP09: connect the updater to WP10's handoff and finish platform-specific signing/store/update ownership after package identities are canonical.
- [ ] WP11: add the executable aggregate source gate only after WP01-WP10 production source exists.

WP06 and WP10 are the first disjoint source packets. WP02-WP05 may proceed in parallel only after the reviewed WP10 implementation roots exist. WP07, WP08, WP09, and WP11 remain ordered downstream.

## Expected test-source wave after production source

- [ ] WP02-WP04: child-labelled package/lifecycle tests for install, startup authority, health, restart, disable/remove, and cleanup on the target desktop host.
- [ ] WP05: correct the JNI bridge expectation to fail closed without trust, then cover current-trust startup, foreground lifecycle, authenticated ingress, removal, and device-owner/manual-required states.
- [ ] WP06: child application identity plus simulator/device capability-limit tests.
- [ ] WP07: bounded respawn, deliberate-stop, reboot, loop-guard, teardown, and health-observer tests by platform.
- [ ] WP08: parent-authority mismatch/replay/restart plus platform cleanup callback/idempotency tests.
- [ ] WP09: updater handoff, installer result/restart, signing/store, and platform matrix behavior tests.
- [ ] WP10: trust-source/currentness, authenticated ingress, external health, durable handoff replay/expiry, updater callback, and crash/restart tests.
- [ ] WP11: aggregate negative fixtures that keep release blocked for every missing or manual-required child path.

## Later validation sequence

After all selected production and test source is present:

1. run focused formatter, syntax, architecture, and crate/package tests per workpack;
2. run platform lifecycle tests only on suitable hosts/devices;
3. run the child plan's focused Enforcer and graph gates;
4. regenerate proof for the complete plan slice;
5. run precommit once for the consolidated branch;
6. open one consolidated PR and let CI reproduce the required validation.

## Failure conditions

- Treating package-script presence, manager declarations, mapped files, contract tests, or historical proof as production completion.
- Starting a graph-blocked source packet or bypassing an implementation dependency.
- Making Child WP10 depend on Setup WP07 and creating a dependency cycle.
- Claiming health from an in-process Rust/Android method without a shipped external endpoint.
- Claiming authenticated ingress from the current in-process queue.
- Claiming uninstall from durable revocation state without a platform cleanup callback and receipt.
- Claiming the iOS child package while the actual app, bundle, scheme, artifact, or smoke defaults retain the parent identity.
