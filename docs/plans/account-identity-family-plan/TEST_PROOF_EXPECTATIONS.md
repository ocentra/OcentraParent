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
# TypeScript account/family helper/projection scope
npm run build --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/family-domain
npm run type-check --workspace @ocentra-parent/family-domain
npm run lint:architecture -- --files packages/family-domain

# Rust account/family parity scope, only when family-identity-core is touched or the workpack names Rust parity proof
cargo test -p ocentra-family-identity-core household_authority
cargo test -p ocentra-family-identity-core session_lifecycle

# Protocol/service scope, only when the selected workpack touches protocol or service transport boundaries
cargo test -p ocentra-parent-agent-protocol account
cargo test -p ocentra-parent-agent-service account

# Portal/setup UI scope, only for WP07 or an explicitly selected UI handoff slice
npm run test --workspace @ocentra-parent/portal -- account
npm run test:e2e --workspace @ocentra-parent/portal -- account
```

Run through `npm run agent:run --` when collecting proof if the logging/evidence wrapper is available.

## Command ownership notes

- `crates/schema` is required when canonical account/family/session/device-authority shapes, literals, brands, parser outputs, route/action/read-model DTOs, or encoded wire shapes change.
- `packages/schema-domain` is required only when a temporary generated-validation or edge-decoder surface changes.
- `packages/family-domain` proves TypeScript helper/projection behavior for account/family authority. It must not re-own canonical shared shapes that belong in `crates/schema` or the owning Rust crate.
- `crates/family-identity-core` proves Rust account/family parity and runtime authority semantics. It must not invent field names, discriminants, nullability, status values, or action names that differ from canonical shared contracts.
- `packages/setup-domain`, `crates/provisioning-core`, `packages/portal-domain`, and `apps/portal` are consumer/projection scopes. Run them only when the selected workpack touches setup/provisioning or UI surfaces.
- `crates/agent-protocol` and `crates/agent-service` are not default proof for every account/family workpack. Use them only when the selected workpack touches protocol envelopes, service handlers, or transport-visible account/family authority.
- Adjacent plans consume account/family authority by handoff. Do not validate or edit their implementation unless the selected workpack explicitly names a route-sync or consumer proof.

## Structured harness logging expectations

Every implementation or proof slice that touches account/session/authZ behavior must preserve both product-safe logging and local harness logging.

Product/runtime-safe logging:

```text
redact tokens, session secrets, provider raw claims, recovery tokens, invite secrets, and child activity data
log actor class, household scope, device scope, action, decision, reason, and audit reference when safe
separate support/admin actor events from parent-owner events
never route child activity evidence into account/identity logs
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, owner module, exit code, result, artifact pointer, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

The logging evidence should let a human, Codex, or project MCP ask what failed, where it failed, what artifact contains raw output, and what the result proves without reading an entire terminal dump.

## Account/family E2E meaning

Do not use one test family to overclaim the whole feature. For this plan, E2E has separate meanings:

```text
contract E2E: Rust-owned canonical shape -> generated DTO or temporary edge decoder -> family-domain helper/projection -> TypeScript tests.
Rust parity E2E: canonical shape/protocol expectation -> family-identity-core parity behavior -> Rust tests.
UI E2E: family/setup state -> portal-domain projection -> apps/portal route -> Playwright or focused browser proof.
runtime E2E: provider/session/account adapter -> D1/DO/KV schema or blocker -> runtime route proof.
cross-plan E2E: setup/payment/policy/data-custody/device-trust/LAN/remote consume account authority through named handoff contracts without owning it.
```

A workpack can be complete for its local slice while runtime E2E or cross-plan E2E remains open. Record that as a no-claim boundary instead of broad DONE.

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

## WP08 Rust Schema And Workers-D1 Runtime Migration

Expected focused commands:

```bash
# Canonical Rust schema crate plus focused account/family authority coverage.
cargo test -p ocentra-schema --test contract
cargo test -p ocentra-family-identity-core household_authority
npm run lint:architecture -- --files crates/schema crates/family-identity-core

# Real Workers-D1 persistence/migration scope.
npm --prefix infra/cloudflare exec wrangler d1 migrations apply <account-identity-d1-database> --local
node --import tsx --test infra/cloudflare/tests/integration/account-identity-d1-migration.test.ts
npm --prefix infra/cloudflare run test:unit
npm --prefix infra/cloudflare run test:integration
npm --prefix infra/cloudflare run test:contract
npm --prefix infra/cloudflare run lint
npm run lint:architecture -- --files infra/cloudflare
```

The migration command and focused test path become runnable only after the
selected account-identity D1 binding, migrations, and test are added. Record a
missing binding, migration environment, or focused test path as a blocker; a
TypeScript D1 test double cannot replace any of them. Run protocol/service
commands only when a typed account-family handoff changes.

Expected proof:

```text
Rust canonical schema authority and compatibility boundary
real Workers-D1 binding and persistence adapter proof
migration apply/compatibility/rollback-or-forward-only proof
account-family integration and negative-path proof
Durable Object/KV non-authority proof
Cloudflare handoff and no-claim boundary
redacted correlated runtime logging and operation-specific negative proof
compact focused command log
```

Required negative cases:

```text
wrong household or revoked/stale actor cannot read or mutate authority state
malformed/duplicate/schema-incompatible records reject or degrade safely
unavailable storage does not invent a successful authority result
D1 test double cannot be reported as production Workers-D1 proof
Durable Objects or KV cannot become relational account-family authority
account, household, device, invite, recovery, and session operations have focused negative coverage
runtime logs redact sensitive values and preserve a safe correlation ID
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
cargo test -p ocentra-family-identity-core household_authority
cargo test -p ocentra-parent-agent-protocol device
cargo test -p ocentra-parent-agent-service device
npm run lint:architecture -- --files packages/family-domain crates/family-identity-core crates/agent-protocol crates/agent-service
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
cargo test -p ocentra-family-identity-core household_authority
cargo test -p ocentra-family-identity-core session_lifecycle
cargo test -p ocentra-parent-agent-protocol account
npm run lint:architecture -- --files packages/family-domain packages/portal-domain apps/portal crates/family-identity-core crates/agent-protocol
```

Expected proof aggregation:

```text
WP01 proof root exists
WP02 proof root exists
WP03 proof root exists
WP04 proof root exists
WP05 proof root exists
WP07 proof root exists or UI blocker recorded
WP08 real Workers-D1 migration, redacted correlated logging, and authority-operation negative proof root exists or precise blocker recorded
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
WP08 D1 test-double or unredacted/correlation-free logs cannot satisfy the WP06 final gate
```
