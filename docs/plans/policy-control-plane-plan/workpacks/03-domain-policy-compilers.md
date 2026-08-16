# Workpack 03: Domain Policy Compilers

Goal: define how the control plane hands typed policy to domain owners through deterministic, versioned compiler contracts.

Owns: domain compiler inputs and outputs, target coverage, unsupported/manualRequired handling, version compatibility, no-runtime-mutation guarantees, and no-silent-drop rules.

Handoff: domain plans own runtime effects. This workpack defines the policy compiler contract matrix only.

## Ownership boundary

```text
policy-control-plane-plan owns compiler contract matrix, versioning, deterministic output, unsupported/manual-required state, rollback refs, and no-claim boundaries.
domain plans own app/game, browser, network, tracking, screen, AI, notification, and enforcement runtime effects.
enforcement-control plan owns action authority and execution.
```

## Required compiler outputs

```text
source policy version
domain target
compiled artifact version
supported capability state
manual-required state
schedule/time metadata
evidence/custody requirements
delivery target
rollback/supersede ref
audit refs
no-claim flags
```

## Required proof fields

The selected proof must name, at minimum:

```text
source_policy_version
domain_target
compiler_output_version
supported_capability_state
manual_required_state
schedule_metadata_state
evidence_custody_requirement_state
delivery_target_state
rollback_ref_state
audit_ref_state
version_compat_state
deterministic_output_state
runtime_mutation_state
source_truth_boundary_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Required domain coverage

```text
app/game policy compiler
browser policy compiler
network/domain policy compiler
tracking/location/geofence policy compiler
screen policy compiler
AI evaluator context compiler
enforcement handoff compiler
notification/ask-parent compiler
```

## Required behavior

- Compilers consume parent policy source documents and produce versioned domain-specific policy artifacts.
- Unsupported capability remains explicit and never silently ignored.
- Compiler output is deterministic and versioned.
- Compilers cannot mutate runtime directly.
- Compilers cannot claim enforcement.
- Compiled artifacts are never source truth.

## Required proof IDs

```text
policy-compiler.contract-matrix
policy-compiler.app-game-fixture
policy-compiler.browser-fixture
policy-compiler.network-fixture
policy-compiler.tracking-fixture
policy-compiler.screen-fixture
policy-compiler.ai-context-fixture
policy-compiler.enforcement-handoff-fixture
policy-compiler.unsupported-manual-required
policy-compiler.version-compat
policy-compiler.deterministic-output
policy-compiler.no-runtime-mutation
policy-compiler.no-unsupported-silent-drop
policy-compiler.domain-cache-not-truth
policy-compiler.rollback-ref-present
```

## Negative cases

```text
compiler silently drops unsupported target
compiler mutates runtime directly
compiler output becomes source truth
same input produces different output
compiled artifact lacks source policy version
compiled artifact lacks rollback ref
manual-required state missing
```

## Proof artifact expectations

```text
docs/proof/policy-control-plane-plan/03-domain-compiler-matrix-proof.md
docs/proof/policy-control-plane-plan/03-domain-fixture-proof.md
docs/proof/policy-control-plane-plan/03-unsupported-manual-required-proof.md
docs/proof/policy-control-plane-plan/03-version-compat-proof.md
docs/proof/policy-control-plane-plan/03-deterministic-output-proof.md
```

## Failure

Do not let parent policy directly manipulate domain runtime state without a compiler and audit boundary. Do not claim domain runtime effects from compiler proof.
