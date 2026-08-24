# WP192 App/Game Android Authority Preflight

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP192 App/Game Android Authority Preflight`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Turn the physical Android policy-state proof into a machine-readable authority
preflight for package policy actions.

This proves that Android hide, suspend, uninstall-block, lock-task, and managed
configuration actions are explicitly blocked before adapter dispatch on the
current physical phone because Device Owner/Profile Owner proof is absent.

## Implementation

The source phase adds
`platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidAuthorityPreflight.java`
and composes it from the manifest-declared child service. It queries the real
`DevicePolicyManager` for this APK's device/profile-owner state and blocks
action rows before adapter dispatch when owner authority is absent. It records
that owner provisioning and a `DeviceAdminReceiver` are not wired; no caller
can mint owner authority. The launcher activity binds to the child service
and renders the redacted authority preflight state without creating authority.
Tests and proof remain absent.

## Validation

Source-only validation ran the focused Android Java compile (excluding the
unavailable `cargo-ndk` bridge task), architecture policy, Enforcer
source-shape, no-test-doubles, validation-bypass, and diff checks. No tests or
physical-device proof were written or executed.

## Proof

No proof artifact exists in this source phase. The graph remains planned and
the required tests/proof/checklist evidence remain open.

## Boundaries

Source packet semantics:

- Android policy actions have explicit Device/Profile Owner preflight rows and
  fail closed before adapter dispatch when the live manager reports no owner.
- Owner state is read from `DevicePolicyManager`; it is never accepted from a
  caller-provided bundle.

Not proved:

- Device Owner/Profile Owner enrollment or physical-device proof.
- DeviceAdminReceiver declaration and provisioning workflow.
- Android hide, suspend, uninstall block, lock task, or managed configuration
  execution.
- Adapter dispatch, platform enforcement, provider delivery, child-device
  delivery, raw package names, or raw device serial custody.
