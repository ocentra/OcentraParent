# 54. Policy Readiness Portal Renderer

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `54. Policy Readiness Portal Renderer`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Owner And Branch

- Owner/lane: `codex-c`
- Branch: `codex/app-game-policy-readiness-portal-renderer`
- Scope: portal App/Game Sessions rendering for the service-backed app/game
  policy readiness read model.

## Goal

Render the existing app/game policy readiness read-model event in the parent
portal so families and testers can see which evidence, approval authority,
platform authority, and classifier inputs are ready, missing, or manual-required
before any policy evaluator or adapter path is allowed to consume them.

## In Scope

- Parse the latest
  `agent.activity.app-game.policy-readiness.read-model.reported` event through
  the existing TypeScript protocol parser in portal live state.
- Add a portal-domain route intent that turns the parsed readiness read model
  into summary rows, readiness-kind rows, evidence references, and parser-fail
  visibility.
- Add an App/Game Sessions route panel and refresh command button for the
  existing service command.
- Add focused portal-domain, text-domain, and portal tests plus proof output.

## Out Of Scope

- Rust service, Rust protocol, or activity-store changes.
- Central product capability checklist edits while another lane owns that lock.
- Live policy evaluator execution, policy authoring UI, persistence, timers, or
  enforcement.
- Notification delivery, child-device UX, adapter dispatch, broad installed-app
  blocking, or platform support claims.

## Proof

- `scripts/test/app-game-policy-readiness-portal-renderer-proof.mjs`
- `output/app-game-plan-proof/54-policy-readiness-portal-renderer`
- `output/app-plan-proof/54-policy-readiness-portal-renderer`
- `test-results/app-game-policy-readiness-portal-renderer-proof/proof.json`

## DONE Checklist

- [ ] Hub lock covers the exact portal, domain, text, docs, proof, and validation
      paths.
- [ ] Portal live state parses the latest policy readiness event with the
      existing protocol parser.
- [ ] Portal-domain intent renders service-backed summary and row details
      without policy execution or adapter dispatch claims.
- [ ] App/Game Sessions route renders the readiness panel and keeps missing or
      parser-failed state visibly non-ready.
- [ ] Proof pack records no service, product-checklist, evaluator, adapter,
      broad blocking, or platform support claim.
