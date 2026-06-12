# Screen AI Pipeline Plan Test and Proof Expectations

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `Screen AI Pipeline Plan Test and Proof Expectations`
> Kind: plan-local test and proof decision tree.
> Read when: After the assigned workpack/checklist row is known; use to choose required tests/proof.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: This file defines required local tests/proof; missing tests keep rows open.

<!-- /agent-capsule -->

Use this after the assigned screen AI pipeline workpack is known. This plan proves the trigger-to-capture-to-analysis pipeline, not broad screen capture product completion.

## Where tests should live

When the screen AI pipeline implementation crate/package exists, tests belong under its test tree and proof output under its proof folder. Until then, colocate with the owning screen/AI package and record paths in the workpack and `PROOF_INDEX.md`.

## Decision Tree

| If the assigned work is... | Read next                    | Expected tests or proof                                                                                         |
| -------------------------- | ---------------------------- | --------------------------------------------------------------------------------------------------------------- |
| Trigger-to-capture gate    | assigned workpack            | trigger source authenticity, custody, idempotency, replay, no capture without policy/permission proof.          |
| OCR/VLM pipeline           | assigned workpack            | fixture regression, output schema invariants, redaction, resource/battery bounds, invalid-output handling.      |
| AI safety analysis         | assigned workpack; AI matrix | prompt injection, hallucination regression, safety boundary, temperature sensitivity, no enforcement authority. |
| Parent-facing result/proof | assigned workpack            | screenshot/crop artifact path, redacted summary, log/trace refs, proof manifest.                                |
| Final rollout/PR gate      | `PROOF_INDEX.md`             | selected risk rows, command logs, screenshot/proof completeness, skipped risk notes.                            |

## Expected test/proof inventory

Use these names as proof intent labels in the assigned workpack/proof note. Implementers choose the actual crate/package test names after the owning implementation boundary exists.

- `screen-ai.ocr-output.invariants`: OCR output is bounded, schema-valid, confidence-aware, and rejects malformed results.
- `screen-ai.vlm-output.invariants`: VLM/classifier output stays schema-valid and cannot create policy authority.
- `screen-ai.prompt-injection.boundary`: visible text or prompt-like screen content cannot override safety rules.
- `screen-ai.hallucination.regression`: known fixtures catch hallucination, overclaiming, and unsupported category output.
- `screen-ai.redaction.custody`: pipeline proof preserves screenshot custody and redaction boundaries.
- `screen-ai.temperature.sensitivity`: model variability stays within accepted output invariants.
- `screen-ai.degraded-model-state`: missing/slow/failed model paths produce safe degraded results and logs.

## Required proof contents

- Trigger source, capture artifact, analysis output, and redaction result.
- Command logs and fixture IDs.
- Explicit boundary between AI advisory output and parent/policy authority.

## Failure conditions

Do not claim DONE or PR_READY if any apply:

- The expected test/proof row for the touched work type is missing.
- The implementation crate/package test folder does not exist and the missing expected location is not recorded.
- Only happy-path tests pass for a trust, policy, persistence, protocol, UI, AI, platform, security, performance, or observability boundary.
- A product/checklist row moved without command logs and proof artifact path.
- A manual-required/platform limitation was converted into a runtime capability claim.
- A proof artifact lacks negative cases, logs/traces where relevant, or exact workpack/checklist linkage.
- A sibling plan or broad source tree was read without a route reason recorded in the workpack/proof note.
