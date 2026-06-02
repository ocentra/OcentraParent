# 17 GitHub Actions Artifact Proof

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

- [x] Check package-preview workflow state when relevant.
- [x] Record artifact names/URLs in PR or merge notes when available.
- [x] Distinguish CI mechanic proof from OS permission proof.
- [x] Route CI failures back to D unless integration-only.
- [x] Pull main and rebase workers after merge.

## Acceptance And Proof

Package claims in merge notes cite CI status and manual-required gaps.

Current proof: the proof script verifies the package-preview workflow has
upload-artifact entries for Windows, Linux, macOS, Android, and iOS preview
artifacts, but records local/PR state as manual-required unless real Actions
run status and artifacts prove readiness.

## Parallel Ownership Notes

Primary watches CI. D fixes branch-owned failures.
