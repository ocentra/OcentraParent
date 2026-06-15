<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `Setup Install Provisioning Plan Next Actions`
> Kind: resume queue and highest-open work.
> Read when: after PLAN_STATE.md.
> Stop rule: pick one workpack; do not broaden into sibling plans.
> Proves: next-action routing only.
> Does not prove: implementation completion or PR readiness.
> Proof rule: update this file only when queue state changes.

<!-- /agent-capsule -->

# Setup Install Provisioning Plan Next Actions

## How to use

1. Confirm branch and assignment.
2. Open `WORKPACK_INDEX.md`.
3. Select exactly one workpack.
4. Open only that workpack.
5. Use `TEST_PROOF_EXPECTATIONS.md` and `PROOF_INDEX.md` for proof obligations.
6. Implement narrow docs/source/test changes.
7. Update `CHECKLIST_INDEX.md`, selected workpack, and `PLAN_STATE.md` only after proof exists.

## Highest-priority queue

### 1. WP01 Family Web Info Site

Expected result:

```text
public route map
no-child-activity-data boundary
data collection matrix
download/register/support/privacy/status route expectations
Cloudflare Pages/Workers deploy shape or blocker
```

### 2. WP02 Registration Login Entry

Expected result:

```text
register/login/invite/resume/recovery route state matrix
handoff contract to account-identity-family-plan
expired/revoked/wrong-household invite states
no child data before household authority
```

### 3. WP03 Parent Install Journey

Expected result:

```text
parent bootstrap code flow
parent platform matrix
parent download/version/integrity display expectations
handoff to parent runtime distribution plan
unsupported/manual-required/update-required states
```

### 4. WP04 Child Install Permission Journey

Expected result:

```text
child bootstrap code flow
child platform/permission matrix
installed/running/permissioned/paired/trusted/policy-ready separation
child disclosure and degraded/manual-required states
handoff to app/runtime distribution/device-trust/LAN owners
```

### 5. WP05 Pairing Readiness Recovery

Expected result:

```text
pairing state machine
readiness matrix
recovery flows
no-fake-ready proof
redacted pairing/setup logs expectation
```

### 6. WP07 First-Run Setup UI And State Machine

Expected result:

```text
end-to-end first-run screen/state model
readiness cards
empty/error/degraded/manual-required states
source/custody/status labels
adjacent handoffs visible
```

### 7. WP06 Rollout Proof And Route Gate

Expected result:

```text
proof manifest
platform readiness matrix
route/index sync
public/private boundary proof
manual-required gap register
safe product-status wording
```

## Blocked execution rules

- Registration/login work is blocked if account provider/session decision is missing.
- Parent installer readiness is blocked if runtime distribution proof is missing.
- Child install readiness is blocked if child runtime/package/permission proof is missing.
- Pairing readiness is blocked if LAN/device-trust proof is missing.
- First-run setup complete is blocked if readiness matrix cannot show account, parent app, child agent, pairing, permissions, custody, and policy baseline states.

## PR readiness guard

A partial PR may be acceptable only when one workpack is fully closed and remaining workpacks are listed.

Do not create PR-ready claims from:

```text
website-only route map
login button without account handoff proof
download button without package/distribution proof
child installer without permission/pairing/readiness proof
UI mock without source/custody/degraded labels
```
