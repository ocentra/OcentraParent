# Research And UI Guidance

## Research Anchors

Use official docs and primary specs for the provider and setup work:

- OAuth 2.0 for Native Apps, RFC 8252
- PKCE, RFC 7636
- OpenID Connect Core
- WebAuthn / passkeys
- NIST SP 800-63B
- OWASP authentication, session, password, and authorization cheat sheets
- Cloudflare D1, Durable Objects, KV, R2, and Access docs
- Firebase Auth docs for ID token verification, custom claims, session cookies, MFA, and email-link flows
- Android App Links and browser OAuth flow docs
- iOS Universal Links and ASWebAuthenticationSession docs
- Browser loopback and deep-link behavior docs

## Guidance From The Research

- Treat authentication as a boundary, not as family authority.
- Use the external browser or platform-auth flow when the provider requires it; do not invent a browser-only credential path if the official SDK or flow is different.
- Keep provider claims adapter-only. IdP claims may prove identity, but they do not prove household membership or device trust.
- Prefer typed callback handling over opaque string parsing.
- Keep login, household creation, child profile creation, device pairing, invite acceptance, and recovery as distinct steps in the UI.
- Show source and custody labels wherever a parent could mistake a cached or partial view for a live authority-bearing view.

## Platform Guidance

- Web is acceptable for the development scaffold, but the production parent portal should be a packaged parent-device surface.
- The first-run flow must work without exposing raw protocol fields.
- Mobile login flows must respect native-app guidance and link handling rules.
- If a provider cannot support a required capability safely, record that limitation instead of hand-waving it.

## Failure Conditions

- A provider choice made without reading the official docs is a failure.
- A UI flow that conflates login with trust is a failure.
- A UI flow that hides the custody source is a failure.
- A native login flow that ignores platform link guidance is a failure.
