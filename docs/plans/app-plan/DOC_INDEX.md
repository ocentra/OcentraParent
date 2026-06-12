<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-plan`
> Doc: `DOC_INDEX.md`
> Kind: local source/feature doc router.
> Read when: Only when this plan needs product source docs.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Doc source changes require feature/checklist/proof sync.

<!-- /agent-capsule -->

# Native Apps Plan Doc Index

| Doc group                                                                                      | Read when                                               |
| ---------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| Feature docs: child-agent-local-service.md, policy-schedules-approvals.md, app-game-control.md | Product goal, status, and acceptance source are needed. |
| Expectation docs: platforms.md, policy.md                                                      | Acceptance contract or expected behavior is needed.     |
| [PLAN_STATE.md](PLAN_STATE.md)                                                                 | Current route state and open gaps are needed.           |
| [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md)                                       | Test/proof selection is needed.                         |
