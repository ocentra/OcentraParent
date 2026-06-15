# 17 GitHub Actions Artifact Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `17 GitHub Actions Artifact Proof`
> Kind: proof reference; read only when validating matching claim.
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

Package preview jobs are part of production-readiness evidence, but CI is not a
substitute for real privileged platform proof.

## Where We Want To Be

PR reports name relevant GitHub Actions jobs, artifacts, status, and known
manual-proof gaps.

## Requirement Checklist

- [ ] Check package-preview workflow state when relevant.
- [ ] Record artifact names/URLs in PR or merge notes when available.
- [ ] Distinguish CI mechanic proof from OS permission proof.
- [ ] Route CI failures back to D unless integration-only.
- [ ] Pull main and rebase workers after merge.

## Acceptance And Proof

Package claims in merge notes cite CI status and manual-required gaps.

Current proof: the proof script verifies the package-preview workflow has
upload-artifact entries for Windows, Linux, macOS, Android, and iOS preview
artifacts, but records local/PR state as manual-required unless real Actions
run status and artifacts prove readiness.

## Parallel Ownership Notes

Primary watches CI. D fixes branch-owned failures.
