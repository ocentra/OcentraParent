# Workpack 06: Rollout Proof and Route Gate

Goal: define the proof package required before setup/install/provisioning can be marked ready.

Owns: plan route sync, proof manifest, expected tests, screenshots, command logs, skipped-risk notes, and current product-status wording.

Expected proof pack:

- Family site build/deploy preview or explicit not-yet-implemented state.
- Registration/login handoff proof.
- Parent install journey proof.
- Child install/permission journey proof.
- Pairing readiness/recovery proof.
- Data custody and account handoff notes.
- Feature route and plan index sync.
- Public/private boundary proof: `family.ocentra.ca` information pages, parent portal login, desktop installer, child-agent installer, and support docs each route to the correct owner.
- Production readiness state per platform: preview, internal test, signed production, store-listed, manual-required, or unsupported.

Failure: PR_READY without named proof artifacts, negative cases, and remaining gaps.

## Decision Tree

| If rollout claim mentions... | Required owner/proof                                                                                |
| ---------------------------- | --------------------------------------------------------------------------------------------------- |
| Public family website        | WP01 plus deployment/build artifact; no account data collection claim unless account plan proves it |
| Registration/login           | account-identity-family-plan provider/session proof                                                 |
| Parent desktop installer     | parent-desktop-runtime-package-plan package/signing/launch proof                                    |
| Child install/permissions    | app/parent runtime/platform owner proof; platform limitations visible                               |
| Pairing/first-run            | LAN/setup workpack proof and recovery states                                                        |
| Payment/subscription entry   | payment-subscription-plan test/live-mode boundary proof                                             |
| Data storage/export/delete   | data-custody-storage-plan proof                                                                     |

Required rollout states:

- `notImplemented`: route is documented but no build/proof exists.
- `previewOnly`: internal build exists; production claims forbidden.
- `manualRequired`: platform/store/permission/human step is required.
- `readyForTest`: proof artifacts exist for a controlled test path.
- `productionReady`: signed/released/support/privacy/legal proof exists.
- `blocked`: missing owner, missing proof, or unresolved Sujan decision.

## Execution Detail

Minimum context:

- This plan's `PLAN_STATE.md`, `WORKPACK_INDEX.md`, `TEST_PROOF_EXPECTATIONS.md`, and `PROOF_INDEX.md`.
- Adjacent plan proof indexes only when a selected setup slice names them.

Required output:

- Setup proof manifest with artifact paths.
- Route/index sync note.
- Product-status wording update.
- Remaining gaps list with owner plan.
- Manual-required state list.

Expected tests/proof names:

- `setup.rollout.route-sync`
- `setup.rollout.proof-manifest`
- `setup.rollout.no-overclaim`
- `setup.rollout.adjacent-handoff-complete`
- `setup.rollout.remaining-gaps-owned`
- `setup.rollout.public-private-boundary`
- `setup.rollout.platform-readiness-matrix`
- `setup.rollout.manual-required-visible`

Proof artifact expectations:

- Command logs for docs/link checks.
- Screenshots or rendered artifacts for touched UI.
- Exact adjacent plan/workpack proof refs.
- Explicit "not implemented yet" state where proof is absent.
- Version/build/deploy IDs for public site, portal, installer, and child-agent artifacts where claimed.
- Negative proof for unsupported platform, expired login/session, failed install, failed pairing, revoked permission, and missing payment/entitlement when relevant.

## Failure Conditions

- Do not call setup ready because one route renders.
- Do not hide missing installer, pairing, account, payment, or custody proof behind marketing copy.
- Do not claim production download/support without signed artifact, update path, rollback path, and support/privacy proof.
