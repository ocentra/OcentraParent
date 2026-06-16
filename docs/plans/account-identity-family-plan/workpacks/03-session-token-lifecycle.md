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

## Fill before DONE

```text
Workpack id and branch:
Credential/session changes:
Touched files:
Validation commands and results:
Proof artifacts:
Known gaps/manual-required states:
No-claim boundaries:
```
