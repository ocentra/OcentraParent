# 42 - Inference Settings Template Governance Lane

## Target State

Inference settings and prompt/template changes are governed, versioned, tested,
and auditable.

## Where We Are

Runtime generation args exist. Prompt/template registry and inference governance
need product rules.

## Checklist

- [ ] Define allowed inference settings per task.
- [ ] Add max token/time/resource guards.
- [ ] Version prompt templates.
- [ ] Record settings in AI result.
- [ ] Add regression fixtures for task prompts.

## Proof

- Settings parser tests.
- Prompt version tests.
- Regression fixture output tests.
