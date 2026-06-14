# Account Identity Family Plan State

Status: execution-grade architecture drafted; implementation and proof remain open.

Research status: current repo contract seeds and platform/identity expectations were inspected. Cloudflare-first custody is the current direction, and Firebase Auth is allowed only as an external identity provider/token issuer if it stays out of family data custody.

Current direction:

- Cloudflare D1 owns relational account, household, membership, device, invite, and session metadata.
- Cloudflare Durable Objects own short-lived coordination, rate limits, invite/recovery/session coordination, and live setup rooms.
- Cloudflare KV is non-authoritative cache and rate-limit hint state only.
- Cloudflare R2 is only for explicitly encrypted artifacts if a later decision approves it.
- Household membership, child profile, device trust, invite, recovery, and controller lease are separate typed boundaries.
- The first-run family setup UI must label live local, LAN, parent cache, parent-owned storage, stale, degraded, and unavailable states honestly.

Open gaps:

- No runtime implementation for provider selection, household membership, sessions, invites, recovery, device authority, or setup UI yet.
- No proof artifacts under `docs/proof/account-identity-family-plan/` yet.
- No route sync or PR-ready proof gate has been satisfied.
- No sibling plan handoff has been consumed for identity authority.

## Execution Gate

- Route and implementation continue from [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
- Update this plan only through the blueprint, the selected workpack, and matching proof rows.
- Do not mark this plan complete from checklist deltas alone.
