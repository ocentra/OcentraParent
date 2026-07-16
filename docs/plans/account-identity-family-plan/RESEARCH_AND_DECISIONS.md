<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `Account Identity Family Research And Decisions`
> Kind: research-backed architecture decision map.
> Read when: WP01/WP03/WP04/WP05/WP06 touches provider, session, recovery, authorization, or security posture.
> Stop rule: do not browse again unless a provider/API fact has changed or this doc names an open research question.
> Proves: research basis only.
> Does not prove: implementation completion or provider readiness.

<!-- /agent-capsule -->

# Account Identity Family Research And Decisions

## Current accepted direction

```text
Cloudflare-first custody for Ocentra account/family authority.
External auth providers may verify identity but must not own Ocentra household/product truth.
Account-family authority lives in typed Ocentra contracts and storage.
```

## Decision D00: provider role and custody boundary

Accepted:

```text
Firebase Auth may be used as an external identity provider or token issuer for MVP only if it stays adapter-only and never becomes the family product data store.
Auth.js may be used only as a session/auth adapter if revocation, token size, custody, and adapter constraints fit Ocentra requirements.
Cloudflare D1 and Durable Objects own user, account, household, membership, role, child profile, device, invite, recovery, and session metadata after token verification.
Allowed auth methods for MVP and later include email link, password, OAuth, MFA, passkey, and device step-up, but each method remains an authentication method, not product custody.
```

Rejected:

```text
Firebase custom claims or IdP profile fields as household/product truth.
Auth.js database state as the family authority model.
Any provider arrangement that requires moving family state out of Cloudflare-owned storage.
```

Provider visibility:

```text
The IdP may see only the minimal identity data it needs to authenticate and issue tokens.
It must not receive household membership, child profiles, device registry rows, invite state, recovery state, policy state, or product readiness data.
```

Degraded and replaceability:

```text
Provider outages surface as degraded or manual-required states; they never unlock privileged family flows.
A later provider swap must keep family truth in Ocentra-owned storage and only swap the external identity adapter.
```

## External source anchors

- Cloudflare D1 is Cloudflare's managed serverless database with SQLite SQL semantics and Worker/HTTP API access. Use it as the default relational store for account, household, membership, child profile, device, invite, recovery, and session metadata when Cloudflare runtime is selected: https://developers.cloudflare.com/d1/
- Cloudflare Durable Objects provide stateful serverless coordination and durable storage close to a uniquely named object. Use them for serialized short-lived coordination such as setup rooms, invite/recovery/session state, rate limit buckets, and live join/pairing flows: https://developers.cloudflare.com/durable-objects/
- Firebase custom claims are carried in ID tokens, must be validated server-side, are size-limited, and are for access control only. Do not use custom claims for profile/product/family data: https://firebase.google.com/docs/auth/admin/custom-claims
- Auth.js supports JWT and database session strategies. Use only after deciding whether revocation, session size, custody, and adapter constraints fit Ocentra requirements: https://authjs.dev/concepts/session-strategies
- OWASP Authorization guidance requires deny-by-default and permission validation on every request. Use this as the authorization baseline: https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html
- OWASP Session Management guidance requires high-entropy meaningless session IDs and server-side session state. Use this as the session-token baseline: https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html
- OWASP Forgot Password guidance requires consistent responses, safe side-channel reset delivery, random single-use expiring tokens, secure storage, and rate limiting. Use this for invite/recovery analogs: https://cheatsheetseries.owasp.org/cheatsheets/Forgot_Password_Cheat_Sheet.html
- OWASP CSRF guidance recommends server-side token patterns for stateful apps and signed session-bound double-submit tokens for stateless patterns, with origin/fetch metadata defenses in depth. Use this for state-changing browser flows: https://cheatsheetseries.owasp.org/cheatsheets/Cross-Site_Request_Forgery_Prevention_Cheat_Sheet.html
- NIST SP 800-63B defines authentication assurance levels, step-up authentication, replay resistance, reauthentication, privacy, and session timeout considerations. Use it as an assurance model, not as a claim of federal compliance: https://pages.nist.gov/800-63-4/sp800-63b.html

## Decision D01: identity provider is not family authority

Accepted:

```text
External IdP proves account identity.
Ocentra-owned contracts/storage prove household membership, roles, child profiles, devices, invites, recovery, and authorization.
```

Rejected:

```text
Firebase custom claims as household membership source of truth.
IdP user profile as child profile/device registry.
Auth.js database model as the product family model without Ocentra contract ownership.
```

Proof required:

```text
00-provider-decision-record.md
02-provider-custody-boundary-proof.md
03-custom-claims-data-minimization-proof.md
```

## Decision D02: Cloudflare storage split

Accepted default when Cloudflare runtime is selected:

```text
D1: relational account/family truth.
Durable Objects: live setup/session/invite/recovery coordination.
KV: non-authoritative cache/rate-limit hints only.
R2: excluded from account authority; encrypted artifacts only if later approved.
```

Required proof:

```text
account-family schema or docs must prove which state lives in D1/DO/KV/R2.
KV entries cannot be authoritative account/family state.
R2 cannot contain account authority or raw child activity evidence.
```

## Decision D03: authorization is ABAC/ReBAC-style, not plain login/RBAC

Required decision inputs:

```text
actor identity
household membership
role
resource household id
resource child profile id
resource device id
device trust state
session freshness
capability grant
revocation status
```

Deny by default. Validate every request. Role alone is not enough.

## Decision D04: session and credential classes are separate

Credential classes:

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

Rules:

```text
session id is opaque and meaningless client-side
server-side session state owns business meaning
refresh rotates or records equivalent replay-safe transition
logout/global revoke blocks future privileged actions
freshness is required for sensitive changes
all privileged requests re-check role/device/freshness/capability
```

## Decision D05: invite/recovery is security-sensitive setup, not UX-only

Invites and recovery must prove:

```text
single purpose
single use
expiry
revocation
wrong-household denial
wrong-role denial
enumeration-resistant response
rate limit or blocker
support/admin minimized audit
account delete/export handoff to data custody
```

## Decision D06: first-run UI must show custody/source truth

UI states must distinguish:

```text
live local
LAN
parent cache
parent-owned storage
stale
degraded
unavailable
manual-required
```

UI must not imply:

```text
login means child device is protected
child profile means child device is trusted
hosted account page owns child activity data
support/admin can act as parent owner
```

## Open research questions

- Provider adapter swap details may still change, but the provider role itself is decided: external IdP/token issuer only, never family authority.
- Exact Cloudflare account auth library choice remains open until Cloudflare-control-plane handoff identifies worker scaffold/auth middleware constraints.
- Passkey/step-up auth implementation path remains open until device-trust-bootstrap-plan and setup-install-provisioning-plan route sync are available.
