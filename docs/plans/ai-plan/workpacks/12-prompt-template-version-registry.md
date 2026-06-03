# 12 - Prompt Template Version Registry

## Target State

Prompts and templates are versioned, minimized, contract-owned, and auditable.

## Where We Are

Prompt/template version is required by expectations. The implementation must
avoid hidden prompt behavior becoming policy.

## Checklist

- [ ] Define prompt/template version contract.
- [ ] Add task-specific prompt ids.
- [ ] Include input minimization rules.
- [ ] Record prompt version in AI result.
- [ ] Add migration/deprecation policy.

## Proof

- Prompt version parser tests.
- Prompt minimization security test.
- Result journal includes prompt/template version.
