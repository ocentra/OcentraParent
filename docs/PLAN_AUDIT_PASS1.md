# Plan and Feature Audit (Pass 1)

## Context

- `docs/features` has 18 feature documents.
- `docs/expectations` has matching acceptance evidence files for those features.
- `docs/plans` currently has 19 plans, including 6 new first-pass owning lanes for setup/login/identity, data custody, payment, policy control, and remote access.
- The active plan set is no longer split into the prior "11 extra plan folders."

Use this file first for assignment triage: it answers only coverage and risk depth.

## High-density decision

- If a feature appears with a dedicated lane in first-pass mode, do not claim implementation-complete. Read that lane's `PLAN_STATE.md` + one workpack + `TEST_PROOF_EXPECTATIONS.md` before status changes.
- If a feature appears in a non-first-pass lane, still read the plan's `PLAN_STATE.md`, `WORKPACK_INDEX.md`, and test/proof row before making edits.
- Do not jump to implementation from this matrix alone.

## Feature coverage and depth

Legend:

- **Detailed**: plan is in-place and not flagged as first-pass research mode.
- **First-pass research**: owner lane exists but plan has not yet moved to implementation-ready shape.
- **Gap remains**: explicit `Current gap` shows open planning/proof debt in `FEATURE_ROUTE_INDEX.md`.

| Feature                               | Owning plan(s)                                                                                                                     | Coverage depth                          | Status          |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------- | --------------- |
| App and game control                  | `app-game-plan` (detailed), `app-plan`, `v0-8-enforcement-control-plan`                                                            | Detailed + first-pass dependencies      | **Partial gap** |
| App install and purchase approval     | `app-game-plan`                                                                                                                    | Detailed                                | **Partial gap** |
| Browser and web control               | `browser-plan`, `v0-8-enforcement-control-plan`                                                                                    | Detailed                                | **Partial gap** |
| Child agent local service             | `app-plan`, `parent-desktop-runtime-package-plan`                                                                                  | Detailed + process ownership            | **Partial gap** |
| Enforcement, integrity, and tamper    | `app-game-plan`, `network-plan`, `v0-8-enforcement-control-plan`                                                                   | Detailed                                | **Partial gap** |
| Evidence store and query              | `data-custody-storage-plan`, `eventing-plan`                                                                                       | First-pass research + existing eventing | **Gap remains** |
| Family setup and device roles         | `account-identity-family-plan`, `lan-plan`, `setup-install-provisioning-plan`, `portal-ux-household-surfaces-plan`                 | First-pass research                     | **Gap remains** |
| Local AI safety evaluator             | `ai-plan`, `screen-ai-pipeline-plan`                                                                                               | Detailed                                | **Partial gap** |
| Location, geofence, and device status | `tracking-plan`                                                                                                                    | Detailed                                | **Partial gap** |
| Network and domain control            | `network-plan`                                                                                                                     | Detailed                                | **Partial gap** |
| Parent assistant actions              | `ai-plan`, `portal-ux-household-surfaces-plan`                                                                                     | Detailed                                | **Partial gap** |
| Policy, schedules, and approvals      | `policy-control-plane-plan`, `portal-ux-household-surfaces-plan`, `account-identity-family-plan`, `v0-8-enforcement-control-plan`  | First-pass research + domain controls   | **Gap remains** |
| Production distribution and support   | `setup-install-provisioning-plan`, `parent-desktop-runtime-package-plan`, `payment-subscription-plan`, `data-custody-storage-plan` | First-pass research + runtime ownership | **Gap remains** |
| Remote, LAN, and mobile platforms     | `remote-access-plan`, `lan-plan`, `parent-desktop-runtime-package-plan`, `account-identity-family-plan`                            | First-pass research + runtime ownership | **Gap remains** |
| Reports, notifications, and sync      | `data-custody-storage-plan`, `eventing-plan`, `portal-ux-household-surfaces-plan`                                                  | First-pass research + eventing          | **Gap remains** |
| Screen evidence analysis              | `screen-ai-pipeline-plan`, `screen-plan`                                                                                           | Detailed                                | **Partial gap** |
| Screen visibility and live view       | `remote-access-plan`, `screen-plan`, `screen-ai-pipeline-plan`, `data-custody-storage-plan`                                        | First-pass research + capture ownership | **Gap remains** |
| Social and video control              | `browser-plan`                                                                                                                     | Detailed                                | **Gap remains** |

## What to read next per workstream

- **Identity/setup**: start in `account-identity-family-plan` + `setup-install-provisioning-plan`.
- **Data custody/export/privacy**: start in `data-custody-storage-plan`.
- **Policy control**: start in `policy-control-plane-plan`.
- **Payment/subscription**: start in `payment-subscription-plan`, compare with `E:\\ocentra-games\\infra\\cloudflare` payment flow patterns.
- **Remote access**: start in `remote-access-plan`, then sync with `screen-plan` and `lan-plan`.

## Evidence already required before any PR-ready claim

- Route and plan claims must be synchronized in `PLAN_INDEX.md`, `FEATURE_ROUTE_INDEX.md`, `PLAN_HEALTH_INDEX.md`.
- No feature should move to DONE without explicit negative tests/proof per its plan lane.
- No "happy-path-only" status updates in any of:
  - auth/session,
  - policy/authority,
  - remote access,
  - custody/export,
  - payment lifecycle,
  - transport/relay.

## Open gaps to fill in next pass

- `setup-install-provisioning-plan`: no Vite/Cloudflare deployment manifest yet.
- `account-identity-family-plan`: no provider decision or session authority matrix implemented.
- `data-custody-storage-plan`: no implemented custody/encryption/export contract proof files.
- `payment-subscription-plan`: no parent product pricing/tier, webhook lifecycle, and entitlement proof yet.
- `policy-control-plane-plan`: no one policy source-of-truth contract yet.
- `remote-access-plan`: no remote session lifecycle/relay proof yet.
