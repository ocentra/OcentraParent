# Workpack 01: Auth Provider Decision

Goal: decide the identity provider architecture and custody boundary before login/user work spreads across setup, portal, or backend docs.

Expected shape:

- Cloudflare-first custody with D1 for family metadata, Durable Objects for short-lived coordination, KV for non-authoritative hints, and R2 only for explicitly encrypted artifacts if later approved.
- Firebase Auth may be used only as an external identity provider/token issuer if it stays adapter-only.
- Auth.js or other app-owned auth is acceptable only if it does not move family product data out of Cloudflare-owned custody.

Expected proof:

- Source-backed decision record.
- Rejected-option notes.
- Custody boundary proof.
- Dev-mode negative proof.

Failure: choosing an auth provider or provider adapter that becomes the owner of household membership, child profiles, devices, invites, or recovery state.

## Execution Detail

Minimum context:

- `docs/features/family-setup-device-roles.md`
- `docs/expectations/family-setup.md`
- `docs/expectations/cloud.md`
- `docs/expectations/platforms.md`
- current official provider docs for the final API behavior

Decision record must answer:

- Is Firebase Auth only an identity provider, or does it own any parent product data?
- Does Cloudflare D1/DO state own users, households, roles, and sessions after token verification?
- What migration path exists if Firebase is used first and later replaced?
- What data can the identity provider see?
- What MFA/passkey/email-link/password capabilities are required at MVP versus later?

Expected tests/proof names:

- `account-identity.provider.decision-record`
- `account-identity.provider.cloudflare-state-owner`
- `account-identity.provider.firebase-idp-only`
- `account-identity.provider.no-firebase-product-data`
- `account-identity.provider.custom-claims-minimized`
- `account-identity.provider.dev-mode-not-production`
- `account-identity.provider.token-verification-boundary`
- `account-identity.provider.provider-outage-degraded`
- `account-identity.provider.migration-path`
- `account-identity.provider.rejected-options`

Proof artifact expectations:

- `01-provider-decision-record.md`
- `01-provider-rejected-options.md`
- `01-provider-custody-boundary-proof.md`
- `01-dev-mode-negative-proof.md`
