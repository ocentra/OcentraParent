# 19 Product Checklist And Feature Doc Sync

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `19 Product Checklist And Feature Doc Sync`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

D is blocked on `docs/product-capability-checklist.md` because A holds the lock.
This should be treated as a coordination issue, not a reason to lose D work.

## Where We Want To Be

Feature docs and checklist rows accurately reflect parent desktop, package,
service, platform, signing, store, relay, and support status.

## Requirement Checklist

- [ ] Update `docs/features/production-distribution-support.md` when package
      proof changes.
- [ ] Update `docs/features/child-agent-local-service.md` when service connection
      status changes.
- [ ] Update `docs/features/remote-lan-mobile-platforms.md` when route/platform
      status changes.
- [ ] Update checklist rows when proof/status changes.
- [ ] If locked, report exact desired checklist language.

## Acceptance And Proof

D `DONE` report includes committed docs or a precise blocker note for primary to
reconcile.

Current status: feature docs are updated. `docs/product-capability-checklist.md`
is locked by A, so D must report desired row language for primary: parent
desktop shell has release-support proof for observer authority, update/rollback
posture, support diagnostics, manual platform proof, and CI artifact honesty;
signing, stores, production rollback, relay, and mobile child-agent parity
remain manual-required/not implemented.

## Parallel Ownership Notes

Primary resolves integration ordering after A releases or merges conflicting
checklist edits.
