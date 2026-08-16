# 11 Assistant Action Preview Flow

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `11 Assistant Action Preview Flow`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md), [test blueprint](../portal-ux-household-surfaces-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Ownership boundary

```text
portal UX owns rendered assistant explanation/action-preview UI, cited state, denied-action state, and no-claim boundaries.
ai-plan owns provider/runtime/model behavior and evidence-context construction.
policy-control-plane-plan owns typed policy action preview and parent confirmation semantics.
v0-8-enforcement-control-plan owns enforcement action authority and adapter execution.
account-identity-family-plan owns role/session authority.
```

## Where We Are

Assistant contracts and evidence context proof exist, but a finished portal-based action preview flow is not done.

## Where We Want To Be

Assistant output becomes cited explanation, proposed typed action, preview, parent confirmation, and child-agent validation/result state.

## Decision Tree

| If the assignment touches... | Read next                                            | Required proof                           |
| ---------------------------- | ---------------------------------------------------- | ---------------------------------------- |
| Assistant/provider runtime   | `../../ai-plan/AGENTS.md`                            | provider/degraded/evidence-context proof |
| Policy action proposal       | `../../policy-control-plane-plan/AGENTS.md` and WP05 | typed preview/confirmation proof         |
| Enforcement or child command | `../../v0-8-enforcement-control-plan/AGENTS.md`      | adapter authority proof                  |
| Portal chat/action UI        | this workpack and exact route/source                 | UI state and denied-action proof         |
| Account/role authorization   | `../../account-identity-family-plan/AGENTS.md`       | role/session/token proof                 |

## Required Flow States

- Informational answer: cited explanation only, no action.
- Proposed action: typed action draft with target, child/device, schedule, evidence refs, risk, and expected effect.
- Preview: deterministic validation result before write.
- Parent confirmation: explicit action with role/session authority.
- Submitted: service accepted request but child/device result may still be pending.
- Result: delivered, active, denied, expired, superseded, unavailable, or rollback-required.
- Degraded provider: assistant unavailable or low-confidence; action creation disabled.

## Required proof fields

The selected proof must name, at minimum:

```text
assistant_surface_state
citation_state
evidence_ref_state
typed_action_preview_state
parent_confirmation_state
role_authority_state
provider_degraded_state
low_confidence_state
prompt_injection_boundary_state
child_agent_validation_state
result_state_separation
enforcement_boundary_state
no_direct_mutation_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Requirement Checklist

- [ ] Show citations/evidence refs for answers.
- [ ] Show proposed rule/report/action preview before writes.
- [ ] Require parent confirmation.
- [ ] Show provider degraded/unavailable state.
- [ ] Keep assistant from direct enforcement.
- [ ] Prove low-confidence and prompt-injection/safety-boundary behavior.
- [ ] Prove denied role/session cannot confirm action.
- [ ] Show child-agent/service validation result separately from assistant proposal.

## Acceptance And Proof

Assistant UI cannot write policy or enforcement state without typed preview and confirmation flow.

Expected proof names:

- `portal.assistant.answer-citations`
- `portal.assistant.typed-action-preview`
- `portal.assistant.parent-confirmation-required`
- `portal.assistant.provider-degraded-disabled`
- `portal.assistant.prompt-injection-negative`
- `portal.assistant.denied-role-negative`
- `portal.assistant.result-state-separation`

Proof must include chat/action screenshots or DOM snapshots, typed action fixture/live response, evidence refs, denied-role case, low-confidence case, and prompt-injection/safety-boundary case.

## Failure Conditions

- Do not let assistant output directly mutate policy, enforcement, billing, account, or child-device state.
- Do not treat model confidence as authority.
- Do not omit evidence refs for safety-relevant recommendations.
- Do not claim assistant action readiness without parent confirmation and owning-plan handoff proof.

## Parallel Ownership Notes

Provider/runtime routing belongs outside C unless the user explicitly assigns it.
