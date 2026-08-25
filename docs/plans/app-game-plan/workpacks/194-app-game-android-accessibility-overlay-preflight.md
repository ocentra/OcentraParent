# WP194 App/Game Android Accessibility Overlay Preflight

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP194 App/Game Android Accessibility Overlay Preflight`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Turn physical Android Accessibility settings into a redacted app/game overlay
preflight for warning, block, request, and usage-context overlay actions.

This proves that Accessibility overlay actions have explicit readiness rows and
remain blocked before adapter dispatch until an enabled service, overlay
runtime proof, and child delivery proof exist.

## Implementation

The source phase extends
`platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidAccessibilityRuntimeService.java`
and composes its redacted preflight from the manifest-declared child service.
The service reads enabled services as `ComponentName` values, counts only
redacted entries, tracks window-state events under a synchronized lock, and
persists state through a bounded, coalescing worker with settings-read,
pending, and durable-write failures kept distinct. The accessibility service is
exported for the Android system binding contract, while the no-context status
path reports no runtime or durable readiness. The launcher activity renders the
redacted preflight from the child service lifecycle. Tests and proof remain
absent.

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

- Android Accessibility settings and runtime state are exposed as synchronized,
  durable, redacted counts and preflight states.
- Overlay execution, adapter dispatch, platform enforcement, and child delivery
  remain unclaimed.

Not proved:

- Accessibility service enablement or physical-device proof.
- Warning, blocking, request, or usage-context overlay execution.
- Adapter dispatch, platform enforcement, provider delivery, child-device
  delivery, raw Accessibility service/component names, or raw overlay content.
