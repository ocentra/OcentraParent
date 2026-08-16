# Plan and Feature Audit (Pass 2, Owner-Verified)

## Scope

Purpose:

- Verify that each feature in `docs/features` has a **real owning plan path**.
- Identify where ownership is **detailed** vs **first-pass**.
- Flag gaps that block implementation claims before status changes.

Sources read for this pass:

- `docs/feature-list.md`
- `docs/FEATURE_ROUTE_INDEX.md`
- `docs/PLAN_INDEX.md`
- `docs/PLAN_HEALTH_INDEX.md`
- plan `PLAN_STATE.md` files for owners marked first-pass or high-gap

## Baseline Facts

- `docs/features`: 18 feature docs.
- `docs/plans`: 21 plans.
- Each feature has at least one explicit owning route row in `FEATURE_ROUTE_INDEX.md`.
- The child-agent runtime route now has a physical workpack tree; the missing-tree gap has been removed.
- `docs/PLAN_HEALTH_INDEX.md` is now a pre-reset snapshot. Use `docs/PLAN_QUALITY_MATRIX.md` for the current documentation-quality view.
- The six long-running planning lanes below are still open implementation routes even though their docs are execution-grade:
  - `setup-install-provisioning-plan`
  - `account-identity-family-plan`
  - `data-custody-storage-plan`
  - `payment-subscription-plan`
  - `policy-control-plane-plan`
  - `remote-access-plan`

Decision confirmation:

- Keep `app-game-plan` and `app-plan` separate.
  - `app-game-plan`: app/game-control policy, enforcement sessions, social/video claims, and app-level safety behavior.
  - `app-plan`: child service runtime, local package process lifecycle, and related household policy entrypoints.
- `docs` routing + workpack structure is currently sufficient for one-shot navigation. The issue is not missing owners, it is first-pass lanes that are still open-gate research.

## Feature Coverage Matrix (high-density)

Legend:

- **Detailed**: owner plan exists with implementation or structured proof scaffolding.
- **First-pass**: owner plan exists but is explicitly research-gate and not implementation-complete.
- **Gap risk**: explicit open gap in feature/plan docs; do not treat as done.

| Feature                               | Owner plan(s)                                                                                                                          | Coverage depth                   | Gap risk                                                                                                                                                |
| ------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| App and game control                  | `app-game-plan`, `app-plan`, `v0-8-enforcement-control-plan`                                                                           | Detailed                         | Partial — app/game evidence and policy-readiness are advanced; platform execution, policy/live app UX, and full cross-platform enforcement remain open. |
| App install and purchase approval     | `app-game-plan`                                                                                                                        | Detailed                         | Partial — store request/approval contracts exist; platform parity and approval UX proof are still incomplete.                                           |
| Browser and web control               | `browser-plan`, `v0-8-enforcement-control-plan`                                                                                        | Detailed                         | Partial — managed flow and intervention have proof rows; unmanaged flow and several adapters remain open.                                               |
| Child agent local service             | `app-plan`, `child-agent-runtime-distribution-plan`                                                                                    | Detailed + first-pass dependency | Partial — child-service architecture exists; pairing runtime stability, packaging, and adapter execution still incomplete.                              |
| Enforcement, integrity, and tamper    | `app-game-plan`, `network-plan`, `v0-8-enforcement-control-plan`                                                                       | Detailed                         | Partial — policy-action integrity edges and rollback/eviction proof are still open.                                                                     |
| Evidence store and query              | `data-custody-storage-plan`, `eventing-plan`                                                                                           | First-pass + existing eventing   | High gap — ownership for custody, migration, export, delete/tombstone, and parent-owned sync is explicitly in research.                                 |
| Family setup and device roles         | `account-identity-family-plan`, `lan-plan`, `setup-install-provisioning-plan`, `portal-ux-household-surfaces-plan`                     | First-pass + detailed            | High gap — no single implemented identity-and-setup contract; install/pairing/recovery still incomplete.                                                |
| Local AI safety evaluator             | `ai-plan`, `screen-ai-pipeline-plan`                                                                                                   | Detailed                         | Partial — local model path and social/video/parent handoff quality gates are open.                                                                      |
| Location, geofence, and device status | `tracking-plan`                                                                                                                        | Detailed                         | Partial — platform adapters, UI, geofence runtime, and retention/delete execution remain open.                                                          |
| Network and domain control            | `network-plan`                                                                                                                         | Detailed                         | Partial — live capture execution, adapter enforcement, and production hardening are open despite deep checklist coverage.                               |
| Parent assistant actions              | `ai-plan`, `portal-ux-household-surfaces-plan`                                                                                         | Detailed                         | Partial — portal/chat execution boundary exists in contract form, but typed action execution is still missing.                                          |
| Policy, schedules, and approvals      | `policy-control-plane-plan`, `portal-ux-household-surfaces-plan`, `account-identity-family-plan`, `v0-8-enforcement-control-plan`      | First-pass + detailed            | High gap — no single policy source-of-truth; compiler/delivery/override model not implemented.                                                          |
| Production distribution and support   | `setup-install-provisioning-plan`, `parent-client-runtime-distribution-plan`, `payment-subscription-plan`, `data-custody-storage-plan` | First-pass + detailed            | High gap — public site, signing, update rollout, billing, and support evidence are incomplete.                                                          |
| Remote, LAN, and mobile platforms     | `remote-access-plan`, `lan-plan`, `parent-client-runtime-distribution-plan`, `account-identity-family-plan`                            | First-pass + detailed            | High gap — remote desktop/control authority, relay, and mobile parity are not done.                                                                     |
| Reports, notifications, and sync      | `data-custody-storage-plan`, `eventing-plan`, `portal-ux-household-surfaces-plan`                                                      | First-pass + existing eventing   | High gap — report parent UX, notification runtime behavior, sync/export safety, and deletion proof remain open.                                         |
| Screen evidence analysis              | `screen-ai-pipeline-plan`, `screen-plan`                                                                                               | Detailed                         | Partial — OCR/VLM coverage and parent proof for live/setting/retention still incomplete.                                                                |
| Screen visibility and live view       | `remote-access-plan`, `screen-plan`, `screen-ai-pipeline-plan`, `data-custody-storage-plan`                                            | First-pass + detailed            | High gap — this feature explicitly requires remote-screen decision/risk model and relay lifecycle first.                                                |
| Social and video control              | `browser-plan`                                                                                                                         | Detailed                         | High gap — contracts exist but source adapters, confidence thresholds, and end-to-end policy actions are incomplete.                                    |
| App/game + policy runtime crossovers  | (covered above under app install and app control + policy feature rows)                                                                | —                                | —                                                                                                                                                       |

## What this means for agents (routing, no handwave)

1. **If a task says a feature**:
   - Open `FEATURE_ROUTE_INDEX.md` row.
   - Open the owning plan `AGENTS.md`.
   - Then `PLAN_STATE.md` + `WORKPACK_INDEX.md` + assigned workpack + `TEST_PROOF_EXPECTATIONS.md` if present.
2. **If a plan still has open implementation work** (`account-identity`, `data-custody`, `policy-control`, `payment-subscription`, `remote-access`, `setup-install-provisioning`):
   - Do not claim feature-complete.
   - Treat all first-pass workpacks as "design + evidence discovery", not implementation finish.
3. **If a feature is `planned/gap` in `FEATURE_ROUTE_INDEX.md`**:
   - Do not claim done in roadmap/checkpoint/PR until explicit proof rows are updated in plan route files.
4. **If assignment is implementation**:
   - Use the plan’s decision tree and read no sibling files unless the selected workpack names a handoff.

## Hard-gate check list before `DONE` / PR-ready in this domain

- `docs/PLAN_INDEX.md`, `docs/PLAN_HEALTH_INDEX.md`, and `docs/FEATURE_ROUTE_INDEX.md` updated and aligned.
- `PLAN_STATE.md` for all touched plans explicitly reflects current route and open work.
- No feature uses status “done/in progress” when its owner lane still has explicit security/privacy/identity/tamper/relay gaps.
- No status change that relies only on happy-path checks.
- For payment, identity, and remote access, proof must include lifecycle, replay, revocation, consent, and abuse controls.

## External reference signal: payment/infrastructure precedent

From `E:\\ocentra-games`:

- Cloudflare Worker + Stripe checkout + webhook stack exists with route/flow test coverage.
- Payment objects move through Durable Object state and validated signature/idempotency paths.
- This is reusable as a pattern reference for parent payment planning, but does **not** imply current Ocentra Parent implementation is complete.

## Open gaps to resolve in next pass (priority)

1. `setup-install-provisioning-plan`: first-run website → account route → installer/parent-child install.
2. `account-identity-family-plan`: provider decision, session lifecycle, role model, invite/recovery, cross-device authz.
3. `data-custody-storage-plan`: export/import/sync/delete boundary with encryption and retention proof.
4. `policy-control-plane-plan`: policy source-of-truth, compiler/delivery/rollback model, ask-parent/override flow.
5. `payment-subscription-plan`: pricing tiers, webhook lifecycle, entitlement ledger, and dispute/refund/cancellation model.
6. `remote-access-plan`: remote session grant model, consent, revocation, relay availability, and artifact retention policy.

## Expected next action by user priority

- If your next objective is **auth/setup**: complete `setup-install-provisioning-plan` + `account-identity-family-plan` first.
- If your next objective is **privacy/data**: complete `data-custody-storage-plan`.
- If your next objective is **billing**: complete `payment-subscription-plan` then hand back to install/setup + portal plans.
- If your next objective is **policy**: complete `policy-control-plane-plan` before moving domain policy rows.
- If your next objective is **remote**: complete `remote-access-plan` and link to `screen-plan`/`lan-plan`.
