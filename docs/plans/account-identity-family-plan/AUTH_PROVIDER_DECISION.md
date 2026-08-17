# Auth Provider Decision

## Decision

The default architecture is Cloudflare-first custody. The selected external
identity provider for the first Worker adapter is Firebase Auth, subject to the
fail-closed verification contract in Cloudflare WP05:

- Firebase Auth verifies the external user identity only. The Worker accepts a
  Firebase ID token only after RS256 signature, issuer, audience, time, and
  subject checks against configured trust material.
- The adapter returns only the verified provider subject. It never returns
  household, member, role, child, device, policy, billing, or session authority.
- Auth.js is not selected for this Worker path. It may not become a hidden
  family-data or authorization owner.

The default architecture is Cloudflare-first custody:

- Cloudflare D1 owns household/account/membership/device/invite/session metadata.
- Cloudflare Durable Objects own short-lived coordination, rate limits, and live setup rooms.
- Cloudflare KV is non-authoritative cache and hint state only.
- Cloudflare R2 is only for explicitly encrypted artifacts if a later decision approves it.

Firebase Auth is accepted only as an external identity provider/token issuer if,
and only if, it stays adapter-only and never becomes the family product data
store. This decision authorizes the Cloudflare WP05 adapter boundary only; it
does not authorize login/session routes, D1 migration, deployment, or proof
completion.

## Rejected Options

| Option | Verdict | Reason |
| --- | --- | --- |
| Firebase owns family product data | Rejected | Identity provider drift becomes custody drift and breaks the local-first storage model. |
| Auth.js as a hidden product-data owner | Rejected | App-owned auth is acceptable only if product data still lives in Cloudflare-owned custody, not inside auth state. |
| Cloudflare Access as the consumer family identity product | Rejected | Access is a boundary tool, not a household identity product. |
| Third-party IdP as the source of truth for household membership | Rejected | Household membership, child profiles, device ownership, and recovery must remain typed product data. |

## Boundary Rules

- Authentication answers "who is this?" only.
- Household membership answers "which family context is this account allowed to use?"
- Device trust answers "which device can act for this authority?"
- Recovery answers "how does the owner regain control?" and must remain auditable.
- Billing, policy, remote access, and data-custody decisions consume this boundary; they do not define it.

## Migration Path

- If Firebase Auth is used first, the product must keep a typed adapter boundary between IdP claims and family product state.
- The IdP may issue identity tokens, but the product must persist household membership, roles, device registrations, invites, and recovery state in Cloudflare-owned storage.
- A later replacement auth provider must not require moving family state out of Cloudflare-owned custody.

## Failure Conditions

- Any proof that suggests the IdP owns household membership, child profiles, or device authority is a plan failure.
- Any log that exposes raw tokens or claims without redaction is a plan failure.
- Any dev-only bypass that can be mistaken for a production auth path is a plan failure.

## Proof Expectations

- Source-backed decision record.
- Rejected-option notes.
- Custody boundary proof.
- Dev-mode negative proof.
