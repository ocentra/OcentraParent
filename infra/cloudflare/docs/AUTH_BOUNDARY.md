# Auth Boundary

Auth states:

- `public`
- `parent-session-required`
- `trusted-parent-device-required`
- `admin-required`
- `support-required`
- `provider-webhook-signature-required`
- `internal-queue-only`

Current blocker:

- the verifier enforces route-level auth state, bearer/session shape, support/admin role checks, webhook signatures, and internal queue secrets
- the real account/session authority and trusted-device source are still dependency-gated for later workpacks
