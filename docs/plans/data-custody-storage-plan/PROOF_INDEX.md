<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `PROOF_INDEX.md`
> Kind: proof artifact router.
> Read when: Only when validating proof, DONE, PR_READY, or broad status claims.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: New proof artifacts must include command/log evidence, negative cases, owner rows, and remaining gaps.

<!-- /agent-capsule -->

# Data Custody Storage Plan Proof Index

No implementation proof is claimed by this routing layer. Future proof must be recorded by workpack.

| Proof kind            | Expected evidence                                                                                   |
| --------------------- | --------------------------------------------------------------------------------------------------- |
| Contract/source proof | Source/feature/checklist rows, schema/protocol shape, negative cases, owner path.                   |
| Runtime/service proof | Real service/adapter/read-model output, logs/traces, degraded and failure states.                   |
| UI proof              | Screenshot or Playwright/browser artifact, state fixture/source, click/action log where applicable. |
| Security/policy proof | Auth matrix, replay/idempotency, stale/unauthorized/cross-family negatives.                         |
| Platform/manual proof | Device/OS/version, permission/enrollment state, adapter output, limitation note.                    |
