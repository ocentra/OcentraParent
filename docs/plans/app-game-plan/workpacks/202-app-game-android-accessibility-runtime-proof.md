# WP202 App/Game Android Accessibility Runtime Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP202 App/Game Android Accessibility Runtime Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Move Android app/game warning, block, request, and usage-context overlay support
from settings-only preflight into a real package-declared Accessibility runtime
boundary.

This proves that the Android debug package declares an Ocentra
AccessibilityService, exposes declaration/runtime/sample state through
MainActivity, and keeps overlay execution blocked until service enablement,
runtime event sample proof, child-device delivery, and platform enforcement
proof exist.

## Implementation

- Added
  `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/AppGameAndroidAccessibilityRuntimeService.java`.
- Added
  `platforms/android/agent/app/src/main/res/xml/app_game_accessibility_service.xml`.
- Updated `platforms/android/agent/app/src/main/AndroidManifest.xml` to declare
  the service with `android.permission.BIND_ACCESSIBILITY_SERVICE`.
- Updated `platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java`
  to surface parent-safe Accessibility declaration, runtime, and event-sample
  states.
- Added focused real behavior coverage at
  `platforms/android/agent/app/src/test/java/ca/ocentra/parent/agent/AppGameAndroidAccessibilityRuntimeServiceTest.java`.

Historical `packages/parent-domain` and `scripts/test/app-game-*` paths were
removed during Rust-first convergence and are not current implementation or
test owners. Canonical `7339e7476` covers global/own-service settings,
bound-event state, interruption versus unbind/destroy lifecycle,
malformed/partial/future durable records, restart readback, and redaction.

## Validation

The focused Java behavior test is written but was not executed in this
code/test-source phase.

## Proof

No proof artifact was generated or retained in this phase. Physical service
enablement/event observation and the canonical workpack proof root remain open.

## Boundaries

Proved:

- Android package declares an Ocentra AccessibilityService bound by the Android
  Accessibility service permission.
- The service config listens only for window state changes and does not request
  window-content retrieval.
- The manifest child runtime exposes parent-safe declaration/runtime/sample
  state and the focused test covers lifecycle, durable failure, restart, and
  redaction behavior.

Not proved:

- Accessibility service enablement if Android settings do not include the
  package-local service.
- Runtime event sample observation if the service is not bound or no event was
  observed.
- Warning, block, request, or usage-context overlay execution.
- Adapter dispatch, platform enforcement, provider delivery, child-device
  delivery, Device Owner/Profile Owner authority, or Play policy proof.
