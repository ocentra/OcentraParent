<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `DOC_INDEX.md`
> Kind: local source/feature doc router.
> Read when: Only when this plan needs product source docs.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Doc source changes require feature/checklist/proof sync.

<!-- /agent-capsule -->

# Data Custody Storage Plan Doc Index

| Doc group                                                                                            | Read when                                               |
| ---------------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| Feature docs: evidence-store-query.md, reports-notifications-sync.md, remote-lan-mobile-platforms.md | Product goal, status, and acceptance source are needed. |
| Expectation docs: data-custody.md, evidence-storage.md, sync-export.md, cloud.md                     | Acceptance contract or expected behavior is needed.     |
| [PLAN_STATE.md](PLAN_STATE.md)                                                                       | Current route state and open gaps are needed.           |
| [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md)                                             | Test/proof selection is needed.                         |
