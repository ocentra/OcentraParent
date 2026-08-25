# Child Agent Runtime Distribution Plan Health

## Route health

- Canonical child distribution route exists.
- Production source, expected test source, artifact parity, runtime reachability, and proof remain open.
- All workpacks remain open until their selected proof roots contain real artifacts, command logs, negative cases, and no-claim boundaries.

## Consistency warnings

- Scaffold/package-script presence can be overclaimed as runtime parity.
- Respawn can be overclaimed as product readiness.
- Signing/store/device-owner status can be hidden or collapsed into ready states.
- Setup can be merged incorrectly with package proof.
- Parent-client claims can leak into this plan.
- Uninstall/tamper proof is missing.
- Shipped desktop and Android startup do not supply a current Device Trust source.
- Current child ingress is in-process only; no authenticated product ingress or external health endpoint is composed.
- Public removal APIs and the setup-handoff projection have no production caller.
- The iOS source identity is child-owned; its smoke/workflow consumers, Apple signing/provisioning, device/store proof, and expected tests remain open.
- Static service-manager restart declarations are not a live lifecycle implementation.
- WP11 has documentation/proof aggregation but no executable aggregate gate.
- Current proof-root routing is `output/child-agent-runtime-distribution-plan-proof/<workpack-file-stem>/`; stale legacy proof paths must not raise status.

## Required source-first hygiene before PR_READY

- Finish the graph-legal coherent production-source packet.
- Add its complete expected test-source packet before running focused validation.
- Update the assigned workpack.
- Update the relevant checklist/proof row.
- Update `PLAN_STATE.md` and `NEXT_ACTIONS.md` if current state changes.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not claim READY from package build or release script presence alone.
- Do not claim READY from checksum, SBOM, or signing proof alone.
- Do not claim READY from Android debug APK proof as device-owner, managed-profile, store, runtime transport, or privileged capability proof.
- Do not claim READY from iOS simulator/provisioning proof as background-service, supervision, or privileged runtime parity.
- Do not claim READY from parent-client proof.
- Do not claim READY from setup journey completion without typed setup-to-child-install handoff and package/runtime proof for the selected claim.
- Do not claim READY from scaffold/manual-required rows.
- Do not claim tamper, uninstall, revocation, resilience, or respawn readiness without the selected platform proof.
- Do not claim release readiness until WP11 aggregates only structured proof roots with explicit no-claim boundaries.

## Agent route walkthrough

- Landing decision: root plan routing selects this plan only for child runtime package distribution, managed respawn where supported, parent-authorized uninstall/revocation, signing/store/device-owner matrix, and setup-device-trust handoff consumption.
- Scope split: parent client distribution, setup journey, trusted-device bootstrap, account identity, policy, enforcement, AI, notification, portal, LAN, remote, payment, and data custody stay in sibling plans unless the selected workpack names a handoff.
- Minimum read set: `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `WORKPACK_FAMILIES.md` only when owner/proof family is unclear, one workpack, `TEST_PROOF_EXPECTATIONS.md`, and `PROOF_INDEX.md` when validating proof.
- Test/proof decision: require artifact, checksum/signing, install/lifecycle, runtime/service, respawn, uninstall/revocation, device-owner/managed-profile/supervision, setup-trust handoff, and release gate tiers only where the selected workpack claims them.
- DONE blocker: no child distribution claim may move unless proof distinguishes artifact build, install state, runtime state, platform capability, setup trust, and no-claim boundaries.

## High-information-density gate

### Scope and ownership

- Scope and ownership: this file governs documentation routing, state, and proof expectations for `child-agent-runtime-distribution-plan`.
- Ownership path: this plan is coordinated through `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, selected workpack files, `TEST_PROOF_EXPECTATIONS.md`, and `PROOF_INDEX.md`.

### State

- Current state: canonical scope and several bounded components exist, but trusted startup, authenticated ingress/health, canonical iOS identity, platform lifecycle, removal callbacks, live handoff/update consumption, expected tests, and the executable aggregate gate remain open.
- Current action: execute WP06 and graph-legal WP10 source first, then follow the dependency order in `WORKPACK_INDEX.md`.

### Decision routes and failure controls

- Decision route: follow the selected workpack path and `WORKPACK_FAMILIES.md` only when the owner/proof family is unclear.
- Failure controls: block completion when package proof, runtime proof, setup proof, platform proof, or release proof are mixed without explicit handoff and no-claim boundaries.

### Proof mapping

- Required proof before READY: selected proof root, command log, artifact pointers, negative cases, platform/manual-required notes, selected workpack updates, and explicit no-claim language.
- WP11 may aggregate proof only after upstream proof roots exist and name their remaining gaps.
