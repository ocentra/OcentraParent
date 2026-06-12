# Workpack 01: Auth Provider Decision

Goal: decide the identity provider architecture before login/user work spreads across setup, portal, or backend docs.

Options to evaluate:

- Cloudflare-first with Auth.js/D1 or equivalent app-owned auth.
- Cloudflare app/data with Firebase Auth only as identity provider/token issuer.
- Full Firebase/Firestore product data ownership.

Default recommendation for first implementation: Cloudflare app/data plus Firebase Auth only if it materially reduces auth risk/time. Keep family product data in Cloudflare-owned domains and parent-owned custody systems.

Expected decision criteria:

- Privacy/data custody.
- Account recovery and MFA/passkey/email-link support.
- Mobile/web SDK maturity.
- Token verification from Workers/local services.
- Cost, lock-in, export/delete, and audit.
- Migration path to app-owned auth if Firebase is used first.

Expected proof:

- Source-backed decision record.
- Threat model.
- Rejected-option notes.
- AuthN/authZ boundary map.

Failure: choosing a provider because it is convenient without proving custody, recovery, token lifecycle, and migration boundaries.

## Execution Detail

Minimum context:

- `E:\ocentra-games\infra\cloudflare\wrangler.toml`
- `E:\ocentra-games\infra\cloudflare\src\utils\auth-middleware.ts`
- `E:\ocentra-games\infra\cloudflare\src\utils\firebase-service-auth.ts`
- `E:\ocentra-games\infra\firebase\README.md`
- Current official provider docs when choosing final API behavior.

Games-project evidence to account for:

- Cloudflare Worker owns API boundary.
- Firebase project id and service-account credentials are used for auth/admin role integration.
- Test mode and disabled-auth modes are explicit dev-only states.
- Cloudflare Durable Objects/KV/R2 are used for app state and coordination.

Decision record must answer:

- Is Firebase Auth only an identity provider, or does it own any parent product data?
- Does Cloudflare D1/Durable Object state own users/households/roles after token verification?
- What migration path exists if Firebase is used first and later replaced?
- What data can identity provider see?
- What MFA/passkey/email-link/phone/recovery capabilities are required at MVP versus later?

Expected tests/proof names:

- `auth-provider.decision-record`
- `auth-provider.firebase-as-idp-only`
- `auth-provider.cloudflare-state-owner`
- `auth-provider.migration-path`
- `auth-provider.dev-mode-not-production`

Failure: silently adopting games Firebase usage without deciding what belongs in Parent identity versus data custody.
