<!-- agent-capsule -->

> Agent Capsule
> Doc: Release and Production Branch Flow
> Kind: agent flow documentation; read only when selected by root AGENTS or TASK_ROUTER.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Release and Production Branch Flow

Use this when touching packaging, installer publishing, version tags, GitHub
Releases, production branch workflows, or package previews.

## Branch boundary

`main` is a CI and package-preview branch. It must not publish GitHub Releases.
Production installer publishing belongs to the `production` branch workflow and
only runs when the aligned version tag is missing.

## Package preview honesty

Package-preview jobs should build and smoke-check real Windows/Linux/macOS/mobile
artifacts, but must not claim signing, stores, device-owner policy, or iOS
Family Controls until those credentials and entitlements are actually wired.

## Before PR_READY

Run `VALIDATION_FLOW.md`, update release/expectation/product docs when claims
change, and include exact artifact names, platforms, and unproved platform gaps.
