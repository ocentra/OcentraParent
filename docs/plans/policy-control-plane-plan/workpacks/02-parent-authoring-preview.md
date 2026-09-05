# Workpack 02: Parent Authoring Preview

Goal: define nontechnical parent authoring and preview so policy can be created without exposing implementation internals or pretending enforcement already happened.

Owns: templates, manual rule creation, preview, confirmation, conflict visibility, unsupported/manualRequired states, stale/offline visibility, accessibility, mobile behavior, and assistant-draft preview-only behavior.

Handoff: portal UX owns the rendered surfaces. AI may draft only. This workpack defines the expected policy-authoring contract, not styling.

## Ownership boundary

```text
policy-control-plane-plan owns authoring contract, preview states, confirmation requirements, no-claim boundaries, and proof route.
portal-ux-household-surfaces-plan owns rendered UI surfaces and accessibility implementation.
ai-plan owns draft suggestions only; AI cannot confirm or apply policy.
account-identity-family-plan owns parent actor/session authority.
device-trust-bootstrap-plan owns high-risk step-up where selected.
```

## Required UI states

```text
draft
previewLoading
previewReady
previewFailed
conflictDetected
unsupportedTarget
manualRequired
staleDevice
offlineChild
scheduleAmbiguous
scheduleInvalid
confirmationRequired
confirmed
queued
delivered
acknowledged
active
partiallyActive
rejected
rolledBack
superseded
expired
```

## Required behavior

- Parent can create rules from templates or manual input.
- Preview explains target, time, condition, action, exception, unsupported/manualRequired state, and expected domain effect before save.
- Preview does not enforce.
- Save requires confirmation.
- Cancel does not mutate source truth.
- Unsupported, offline, stale, and manualRequired states stay visible.
- AI drafts remain preview-only until parent confirmation.
- Mobile and accessibility coverage are required, not implied.

## Required proof fields

The selected proof must name, at minimum:

```text
authoring_surface_state
template_state
manual_rule_state
preview_state
conflict_state
unsupported_state
stale_offline_state
manual_required_state
confirmation_state
cancel_mutation_state
assistant_draft_state
mobile_accessibility_state
portal_handoff_state
no_enforcement_claim
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Required proof IDs

```text
policy-authoring.template-create
policy-authoring.manual-rule-create
policy-authoring.preview-before-save
policy-authoring.preview-no-enforcement
policy-authoring.conflict-visible
policy-authoring.unsupported-target-visible
policy-authoring.stale-device-visible
policy-authoring.offline-child-visible
policy-authoring.manual-required-visible
policy-authoring.no-fake-green
policy-authoring.mobile-accessible-state
policy-authoring.copy-parent-readable
policy-authoring.cancel-draft-no-mutation
policy-authoring.confirmed-version-created
policy-authoring.assistant-draft-preview-only
```

## Negative cases

```text
rule saves without preview
preview enforces runtime behavior
unsupported target hidden
offline child shown as ready
stale device shown as successful
manual-required hidden behind green check
cancelled draft still changes policy
assistant draft saves without confirmation
```

## Proof artifact expectations

```text
docs/proof/policy-control-plane-plan/02-authoring-preview-proof.md
docs/proof/policy-control-plane-plan/02-conflict-visible-proof.md
docs/proof/policy-control-plane-plan/02-unsupported-target-proof.md
docs/proof/policy-control-plane-plan/02-no-fake-green-proof.md
docs/proof/policy-control-plane-plan/02-assistant-draft-preview-only-proof.md
```

## Current evidence

`02-assistant-draft-preview-only-proof.md` is present and records the live
Rust request contract, parent confirmation boundary, and Rust-owned portal
projection. This closes only the assistant-draft preview-only evidence slice;
the workpack remains open for authoring, save confirmation UX, accessibility,
mobile, and opaque confirmed-request relay proof.

## Production code pass status

The Rust-owned parent UI bridge now validates and stages the untrusted draft,
issues a bounded one-shot opaque handle, constructs the typed confirmed-request
command, and dispatches it through the existing Rust agent boundary. The portal
preview route submits draft input only to staging; confirmation submits only
the Rust-issued handle, while cancel invalidates it. Rust projects exact
household, child/profile, policy/source, actor, timestamp, and audit context
from the trusted preview row; missing context fails closed for manual review.
The handle is marked in flight before dispatch, restored on rejected/deferred
dispatch, and consumed only after accepted relay. This is code drafted only:
validation and tests are intentionally deferred, so no runtime completion claim
is made.

## Failure

Do not let the policy UI save ambiguous rules without preview, explanation, or conflict handling, and do not treat assistant-drafted text as applied policy.

Keep WP02 open until targeted portal/authoring/preview proof exists or dependency blockers are explicitly carried.
