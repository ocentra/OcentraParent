# Workpack 06: Rollout Proof And Route Gate

Goal: define the proof package required before `setup-install-provisioning-plan` can be described as execution-grade or ready for any downstream handoff.

Owns: route/index sync, proof manifest, exact test ids, proof artifact inventory, manual-required states, skipped-risk notes, and product-status wording.

## What This Workpack Must Prove

- The public family site, registration/login handoff, parent bootstrap install, child bootstrap install/permission flow, pairing readiness/recovery, and first-run setup state machine each have named proof artifacts.
- The plan docs, `docs/PLAN_INDEX.md`, and `docs/FEATURE_ROUTE_INDEX.md` agree on the current route and gap wording.
- The public/private boundary is explicit: information pages, parent portal login, desktop installer, child-agent installer, and support docs each route to the correct owner.
- Platform readiness is spelled out per target as `previewOnly`, `manualRequired`, `readyForTest`, `productionReady`, or `unsupported`.
- No downstream doc may claim ready/production status without a matching proof artifact and negative-case coverage.

## Required Output

- Setup proof manifest with artifact paths.
- Route/index sync note.
- Product-status wording update.
- Remaining gaps list with owner plan.
- Manual-required state list.

## Minimum Context

- This plan's `PLAN_STATE.md`, `WORKPACK_INDEX.md`, `TEST_PROOF_EXPECTATIONS.md`, and `PROOF_INDEX.md`.
- Adjacent plan proof indexes only when a selected setup slice names them.

## Decision Tree

| If rollout claim mentions... | Required owner/proof |
| --- | --- |
| Public family website | WP01 plus deployment/build artifact; no account-data claim unless the account plan proves it |
| Registration/login | `account-identity-family-plan` provider/session proof |
| Parent client installer | `parent-client-runtime-distribution-plan` package/signing/launch proof |
| Child install/permissions | runtime/platform owner proof; platform limitations visible |
| Pairing/first-run | setup/LAN workpack proof and recovery states |
| Payment/subscription entry | `payment-subscription-plan` test/live-mode boundary proof |
| Data storage/export/delete | `data-custody-storage-plan` proof |

## Required Rollout States

- `notImplemented`: route is documented but no build/proof exists.
- `previewOnly`: internal build exists; production claims are forbidden.
- `manualRequired`: platform/store/permission/human step is required.
- `readyForTest`: proof artifacts exist for a controlled test path.
- `productionReady`: signed/released/support/privacy/legal proof exists.
- `blocked`: missing owner, missing proof, or unresolved decision.

## Expected Tests

- `setup.rollout.route-sync`
- `setup.rollout.proof-manifest`
- `setup.rollout.no-overclaim`
- `setup.rollout.adjacent-handoff-complete`
- `setup.rollout.remaining-gaps-owned`
- `setup.rollout.public-private-boundary`
- `setup.rollout.platform-readiness-matrix`
- `setup.rollout.manual-required-visible`
- `setup.rollout.privacy-copy-reviewed`
- `setup.rollout.product-status-safe`

## Proof Artifacts

- `06-rollout-proof-pack.md`
- `06-route-sync-proof.md`
- `06-platform-readiness-matrix.md`
- `06-public-private-boundary-proof.md`
- `06-manual-required-gap-register.md`

## Failure Conditions

- PR_READY or DONE without named proof artifacts.
- Any rollout note that collapses parent bootstrap, child bootstrap, or pairing into one claim.
- Any ready/production wording without a matching negative case or manual-required note.
