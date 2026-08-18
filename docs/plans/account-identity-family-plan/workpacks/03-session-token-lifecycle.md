<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `WP03 Session Token Lifecycle`
> Kind: assigned implementation workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: do not open sibling workpacks; do not implement provider decision, invite/recovery, UI, payment, or policy work here.
> Proves: session and credential lifecycle only after tests/proof pass.
> Does not prove: provider selection, household role model, invite/recovery readiness, or product login readiness.
> Proof rule: before DONE, write all WP03 proof artifacts and command log.

<!-- /agent-capsule -->

# WP03 Session Token Lifecycle

## Goal

Define browser sessions, refresh, logout, revocation, expiry, replay resistance, state-changing request safety, and credential class separation.

## Required inputs

```text
workpacks/01-auth-provider-decision.md
workpacks/02-identity-household-role-model.md
RESEARCH_AND_DECISIONS.md
packages/family-domain/src/session-lifecycle.ts
packages/family-domain/src/household-authority.ts
packages/family-domain/tests/unit/session-lifecycle.test.ts
```

## Credential classes

These must be separate and not interchangeable:

```text
browser user session
parent trusted-device credential
child-device agent credential
invite token
recovery token
controller lease
remote capability grant
support/admin session
```

## Required lifecycle

```text
login/session creation
session refresh
refresh rotation or equivalent replay-safe transition
logout
global revoke
session expiry
clock-skew tolerance
sensitive action freshness check
device credential issuance boundary
redacted session audit event
```

## Expected source changes

Likely paths:

```text
packages/family-domain/src/session-lifecycle.ts
packages/family-domain/src/household-authority.ts
packages/family-domain/tests/unit/session-lifecycle.test.ts
packages/family-domain/tests/unit/household-authority.test.ts
```

## Required proof root

```text
output/account-identity-family-plan-proof/03-session-token-lifecycle/
```

Required artifacts:

```text
00-credential-type-matrix.md
01-session-lifecycle-proof.md
02-token-expiry-replay-proof.md
03-refresh-revocation-proof.md
04-session-freshness-proof.md
05-csrf-origin-proof.md
06-token-redaction-proof.md
16-validation-commands.log
```

## Acceptance criteria

- [ ] Credential type matrix exists.
- [ ] Browser session lifecycle is defined/tested.
- [ ] Refresh rotation or equivalent replay-safe transition is defined/tested.
- [ ] Logout and global revoke are defined/tested.
- [ ] Expiry and clock-skew are defined/tested.
- [ ] Reuse/stale-token negative cases are covered.
- [ ] Device, invite, recovery, controller-lease, and remote-grant credentials are not accepted as browser sessions.
- [ ] Sensitive actions require freshness.
- [ ] State-changing browser request safety proof or blocker exists.
- [ ] Session audit logs are redacted.
- [ ] Focused commands pass or blockers are recorded.

## Focused commands

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/family-domain -- session
npm run test --workspace @ocentra-parent/family-domain -- token
npm run lint:architecture -- --files packages/family-domain
```

## Negative cases

- Expired session denied.
- Revoked session denied.
- Old refresh credential denied after rotation or equivalent lifecycle step.
- Device credential cannot be used as browser user session.
- Invite token cannot be used as user session.
- Recovery token cannot be used as user session.
- Controller lease cannot be used as user session.
- Sensitive action denied when freshness is missing.
- State-changing browser request without the required safety signal is denied or explicitly blocked from claim.

## Manual-required gaps

Provider implementation remains tied to WP01. Device trust/step-up proof remains tied to device-trust-bootstrap-plan.

## 2026-08-17 current code/test correction

The Rust session evaluator and its focused tests cover credential separation,
expiry/skew classification, replay/revocation rejection, freshness, creation,
rotation decisions, and scoped issuance. Provisioning consumes the pairing
decision. This remains a decision library: callers provide lifecycle/replay and
freshness facts, and credential issuance has no production caller.

Production source still required:

- durable token-digest/session/refresh-family storage;
- repository-owned rotation generation, replay registry, logout/global-revoke
  epoch, issued/expiry calculation, and audit emission;
- real account browser/session routes consuming server-derived WP08 identity;
- controller-lease and support/admin credential classes where the owning routes
  require them.

Expected test source still required:

- concurrent atomic refresh rotation and replay-after-restart;
- logout/global revoke, malformed/backdated state, and clock-skew edges;
- exact redacted audit records and recovery from partial persistence;
- CSRF, origin, and fetch-metadata negatives on the actual account route.

### Accepted replacement source delta

The accepted `35edb2830` source stores session identity, generation, expiry,
freshness, and revocation in the current Account authority repository and adds
an owner-derived session lifecycle record. Callers no longer supply freshness
or replay authority as trusted booleans. A shipped provider/account session
route, refresh/revoke orchestration, and the complete expiry/skew/replay/
restart/concurrency/browser-negative expected-test family remain open.

The remote packet `ac03afee3a` is rejected/quarantined: its public
deserializable session record accepted caller-provided replay/freshness state,
had no token custody or durable repository, and allowed terminal/backdated
rewrites. It is not WP03 progress.

## 2026-08-18 candidate production source boundary

The candidate Cloudflare source packet supplies the WP03 production/runtime
composition, without reviving caller-minted session facts or evaluators.
Independent review remains open; the reachable source is:

- `infra/cloudflare/migrations/account-identity/0005_account_browser_session_custody.sql`
  and `0006_account_browser_session_refresh_custody.sql` remain historical
  custody definitions; forward `0007_account_browser_session_custody_hardening.sql`
  rebuilds them as STRICT, quarantines and aborts on invalid legacy rows, and
  publishes the runtime schema-version sentinel only after the full copy;
- `infra/cloudflare/src/storage/account-browser-session-codec.ts` and
  `account-browser-session-store.ts` for opaque cookies, non-forgeable
  Account authority capabilities, same-boundary currentness revalidation,
  refresh-family CAS, exact CSRF digest checks, refresh-bound logout/revoke,
  and redacted audit custody;
- `infra/cloudflare/src/auth/account-identity-authority-caller.ts`,
  `verifier.ts`, and `providers/firebase-auth.ts` for the final WP06 provider
  result distinction and provider-to-Account authority caller; and
- `infra/cloudflare/src/auth/browser-session-routes.ts` plus `routes.ts` for
  origin/fetch-metadata request safety, bounded correlation, login/refresh/
  logout/global-revoke reachability, and secure `__Host-` cookies where legal.

The session store rejects structurally forged capabilities at runtime, permits
only parent/controller/support browser roles (observer and child roles never
map to parent), binds refresh and CSRF credentials to one session family, and
uses a durable generation fence for global revoke. Rotation is one D1 CAS
sequence: the old refresh digest is rotated first, then consumed only by the
new generation/current digest, then audited; a failed CAS commits neither
consumed custody nor audit. Session mutation/audit custody is guarded so a
successful mutation without its audit outcome rolls back. Access expiry does
not block refresh-bound logout or global revoke, while an optional access
cookie must still bind to the same session.

Every public store operation captures trusted `Date.now()` inside the store;
callers may provide only bounded request correlation. The forward `0007`
custody rebuild is SQLite `STRICT` with digest, timestamp, generation,
lifetime, status, and correlation checks; the store requires its exact version
sentinel before any authority-bearing read or mutation. D1 rows are decoded
through an exact runtime validator that rejects malformed types, digest reuse,
timestamp or generation violations, inconsistent revocation state, and
incomplete or mismatched support receipt bindings before any identity or
mutation is accepted. The verified capability carries the complete support
receipt provenance and the create CAS compares every field.

The expected runtime test source is still absent and must be added by the
test/proof phase:

- `infra/cloudflare/tests/unit/account-browser-session-store.test.ts`
- `infra/cloudflare/tests/unit/account-browser-session-routes.test.ts`
- `infra/cloudflare/tests/security/account-browser-session-request-safety.test.ts`
- `infra/cloudflare/tests/integration/account-browser-session-real.test.ts`

The candidate is rebased onto Cloudflare WP06 final head `56a4faa37`. It does
not claim applied D1 migrations, live Worker deployment, tests, retained proof,
precommit, CI, PR, or DONE. The final WP06 mutation-readiness seam remains
parameterless/manual-required; no caller-side authority is fabricated.

## Superseded historical record (not current status)

> Everything in this historical block predates the Cloudflare runtime packet.
> Its artifact list, command results, and completion wording are provenance
> only; they do not establish current WP03 acceptance, production reachability,
> retained proof, or DONE status. The candidate boundary above is authoritative.

- Workpack id and branch: `WP03 Session Token Lifecycle`; `codex/tracking-plan-full-continuation-a`.
- Historical status (superseded): it claimed completion for a local contract/proof slice and listed prior output artifacts; that claim is not current WP03 source or proof status.
- Contract/source changes in this slice: no new WP03-owned production TypeScript or Rust logic was required. The owned session contract was already present in `packages/family-domain/src/session-lifecycle.ts`, and the proof closure is derived from existing TypeScript and Rust session/token coverage plus an explicit blocker note where this slice does not own a real browser request surface.
- Touched files:
  - `docs/plans/account-identity-family-plan/CHECKLIST_INDEX.md`
  - `docs/plans/account-identity-family-plan/PLAN_STATE.md`
  - `docs/plans/account-identity-family-plan/WORKPACK_INDEX.md`
  - `docs/plans/account-identity-family-plan/workpacks/03-session-token-lifecycle.md`
  - `output/account-identity-family-plan-proof/03-session-token-lifecycle/00-credential-type-matrix.md`
  - `output/account-identity-family-plan-proof/03-session-token-lifecycle/01-session-lifecycle-proof.md`
  - `output/account-identity-family-plan-proof/03-session-token-lifecycle/02-token-expiry-replay-proof.md`
  - `output/account-identity-family-plan-proof/03-session-token-lifecycle/03-refresh-revocation-proof.md`
  - `output/account-identity-family-plan-proof/03-session-token-lifecycle/04-session-freshness-proof.md`
  - `output/account-identity-family-plan-proof/03-session-token-lifecycle/05-csrf-origin-proof.md`
  - `output/account-identity-family-plan-proof/03-session-token-lifecycle/06-token-redaction-proof.md`
  - `output/account-identity-family-plan-proof/03-session-token-lifecycle/16-validation-commands.log`
- Validation commands and results:
  - `command: npm run build --workspace @ocentra-parent/family-domain`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: family-domain build passed after the local WP04 repair and before WP03 proof closure`
  - `command: npm run test --workspace @ocentra-parent/family-domain -- tests/unit/session-lifecycle.test.ts tests/unit/token-lifecycle.test.ts`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: direct session/token contract suite passed with 10 tests for session lifecycle, issuance, freshness, and redaction`
  - `command: cargo test -p ocentra-family-identity-core session_lifecycle`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: Rust parity session_lifecycle subset passed with 13 tests covering expiry, replay, creation, rotation, revocation, and scoped issuance`
  - `command: npm run lint:architecture -- --files packages/family-domain`
  - `exit: 0`
  - `result: pass`
  - `artifact: n/a`
  - `notes: focused TypeScript architecture gate passed for the touched family-domain scope`
- Proof artifacts:
  - `output/account-identity-family-plan-proof/03-session-token-lifecycle/00-credential-type-matrix.md`
  - `output/account-identity-family-plan-proof/03-session-token-lifecycle/01-session-lifecycle-proof.md`
  - `output/account-identity-family-plan-proof/03-session-token-lifecycle/02-token-expiry-replay-proof.md`
  - `output/account-identity-family-plan-proof/03-session-token-lifecycle/03-refresh-revocation-proof.md`
  - `output/account-identity-family-plan-proof/03-session-token-lifecycle/04-session-freshness-proof.md`
  - `output/account-identity-family-plan-proof/03-session-token-lifecycle/05-csrf-origin-proof.md`
  - `output/account-identity-family-plan-proof/03-session-token-lifecycle/06-token-redaction-proof.md`
  - `output/account-identity-family-plan-proof/03-session-token-lifecycle/16-validation-commands.log`
- Known gaps/manual-required states: `05-csrf-origin-proof.md` is an explicit blocker note, not proof of real CSRF/origin/fetch-metadata enforcement; a real browser request surface remains outside this slice and must be closed later in the owning runtime surfaces. Provider implementation remains tied to WP01, device trust/step-up proof remains external, and WP07/WP06 still need their own proof roots.
- No-claim boundaries: do not claim real browser request safety, provider/runtime completion, invite/recovery completion, UI readiness, or route-gate completion from this WP03 closure.
