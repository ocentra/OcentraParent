# Workpack 02: Parent Authoring Preview

Goal: define nontechnical parent authoring and preview so policy can be created without exposing implementation internals or pretending enforcement already happened.

Owns: templates, manual rule creation, preview, confirmation, conflict visibility, unsupported/manualRequired states, stale/offline visibility, accessibility, mobile behavior, and assistant-draft preview-only behavior.

Handoff: portal UX owns the rendered surfaces. AI may draft only. This workpack defines the expected policy-authoring contract, not styling.

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

## Failure

Do not let the policy UI save ambiguous rules without preview, explanation, or conflict handling, and do not treat assistant-drafted text as applied policy.
