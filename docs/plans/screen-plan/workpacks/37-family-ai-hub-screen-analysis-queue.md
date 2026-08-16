# 37 Household Mesh Screen Analysis Queue

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `37 Household Mesh Screen Analysis Queue`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Target State

Heavier analysis moves to a local trusted household mesh provider before any
remote/API path.

## MVP Boundary

This is AI-pass and architecture-alignment work. Capture MVP should expose route
state for household-provider-required cases.

## Checklist

- [ ] Define trusted household provider availability state.
- [ ] Define local-network custody boundary.
- [ ] Define summary/image transfer rules if any.
- [ ] Prefer redacted/cropped input.
- [ ] Record parent approval requirements.
- [ ] Add fallback to manual-required when no trusted household provider is
      available.

## Proof

- Household mesh provider route contract.
- Custody and no-remote-default proof.

Proof command:

```powershell
node scripts/test/screen-family-ai-hub-routing-proof.mjs
```

Proof artifact:

```text
output/screen-plan-proof/37-family-ai-hub-screen-analysis-queue/proof-summary.json
```

## Non-Claims

- No real LAN household mesh runtime, discovery protocol, or relay is
  implemented by this screen-plan proof.
- No production OCR/VLM model quality is claimed.
- No remote/API child-safety route, policy decision, portal UI, or enforcement
  adapter is claimed.
