<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `Account Identity Family Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Account Identity Family Plan Test Proof Expectations

## General rule

Use focused commands first. Broader validation is allowed only after focused commands pass or a precise blocker is recorded.

If a required package/test path does not exist yet, write a blocker artifact and leave the checklist row open.

## Common command set

Use the subset relevant to the selected workpack:

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/family-domain
npm run type-check --workspace @ocentra-parent/family-domain
npm run lint:architecture -- --files packages/family-domain
cargo test -p ocentra-parent-agent-protocol account
cargo test -p ocentra-parent-agent-service account
npm run test --workspace @ocentra-parent/portal -- account
npm run test:e2e --workspace @ocentra-parent/portal -- account
```

Run through `npm run agent:run --` when collecting proof if the logging/evidence wrapper is available.

## WP01 Auth Provider Decision

Expected focused commands:

```bash
node -e "console.log('provider-decision-docs-only')"
npm run lint:architecture -- --files docs/plans/account-identity-family-plan
```

Expected proof:

```text
provider decision record
rejected options
custody boundary proof
custom claims/data-minimization proof
provider outage/degraded proof
migration path proof
```

Required negative cases:

```text
Firebase/Auth.js cannot own household membership
custom claims cannot store product/family data
provider outage cannot unlock privileged flows
production cannot run in dev-mode auth bypass
```

## WP02 Identity Household Role Model

Expected focused commands:

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/family-domain -- household
npm run test --workspace @ocentra-parent/family-domain -- authority
npm run lint:architecture -- --files packages/family-domain
```

Expected coverage:

```text
account user is not household member by default
child profile is not child device
parent owner/co-parent/observer/child/support roles are distinct
revoked/disabled/pending/invited states deny or degrade correctly
cross-family id guessing is denied
support/admin actor is minimized and audited
```

## WP03 Session Token Lifecycle

Expected focused commands:

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/family-domain -- session
npm run test --workspace @ocentra-parent/family-domain -- token
npm run lint:architecture -- --files packages/family-domain
```

Expected coverage:

```text
credential type matrix
login/session creation
refresh rotation
logout and global revoke
session expiry and clock skew
replay rejection
stolen/old token denial
CSRF/origin/fetch-metadata proof for state-changing browser flows or explicit blocker
device credential is not browser user session
invite/recovery/controller lease tokens are not sessions
sensitive action requires freshness
redacted audit log emitted
```

## WP04 Invites Recovery Lifecycle

Expected focused commands:

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/family-domain -- invite
npm run test --workspace @ocentra-parent/family-domain -- recovery
npm run lint:architecture -- --files packages/family-domain
```

Expected coverage:

```text
invite state machine
single-use invite
expired/revoked/replayed invite rejected
wrong-household/wrong-role invite rejected
co-parent and observer scopes distinct
child-device pairing invite scope distinct
forgot-login, lost-parent-device, compromised-account, child-reinstall, household-transfer flows modeled
recovery owner approval and support audit
enumeration-resistant response and timing strategy
rate limiting or exact blocker
account delete/export handoff to data custody
```

## WP05 Device Ownership AuthZ

Expected focused commands:

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/family-domain -- device
npm run test --workspace @ocentra-parent/family-domain -- authorization
cargo test -p ocentra-parent-agent-protocol device
cargo test -p ocentra-parent-agent-service device
npm run lint:architecture -- --files packages/family-domain crates/agent-protocol crates/agent-service
```

Expected coverage:

```text
actor/household/role/device/session/capability matrix
parent controller authority
observer read-only behavior
child agent authority only for its own device scope
pending/trusted/revoked/disabled/stale states
wrong household denied
controller lease required/expired/revoked
remote view and remote control capability separation
export/delete owner-only
billing parent-owner-only
all decisions emit audit refs
```

## WP07 Parent Account Family Setup UI

Expected focused commands:

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run build --workspace @ocentra-parent/portal-domain
npm run test --workspace @ocentra-parent/portal -- account
npm run test --workspace @ocentra-parent/portal -- family
npm run test:e2e --workspace @ocentra-parent/portal -- account
npm run lint:architecture -- --files packages/family-domain packages/portal-domain apps/portal
```

Expected coverage:

```text
sign-in/no-household/create-household/join-household states
add child profile
pair child device
co-parent invite
observer invite
role visibility matrix
device trust status
revoked device and expired session status
recovery status
support access status
manual-required status
source labels: live local, LAN, parent cache, parent-owned storage, stale, degraded, unavailable, manual-required
UI does not imply login equals device trust
UI does not show fake child activity data
```

## WP06 Security Proof And Route Gate

Expected focused commands:

```bash
npm run validate:logging
npm run build --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/portal -- account
cargo test -p ocentra-parent-agent-protocol account
npm run lint:architecture -- --files packages/family-domain packages/portal-domain apps/portal crates/agent-protocol
```

Expected proof aggregation:

```text
WP01 proof root exists
WP02 proof root exists
WP03 proof root exists
WP04 proof root exists
WP05 proof root exists
WP07 proof root exists or UI blocker recorded
route sync proof names consumers and handoffs
manual-required gap register exists
```

Required negative cases:

```text
missing provider decision blocks auth-ready claim
cross-family access denied
revoked actor denied
replayed token rejected
replayed invite rejected
open redirect/origin/CSRF state-changing request blocked or explicit blocker recorded
provider outage degrades safely
support/admin cannot act as owner
child profile cannot authorize child device
login cannot authorize policy/payment/remote/export without role/device/freshness gates
```
