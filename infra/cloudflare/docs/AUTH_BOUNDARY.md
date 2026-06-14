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

- the verifier only enforces header presence and role shape
- the real account/session adapter is still `account-auth-adapter-manual-required`
