# Decisions

This plan has nine canonical decisions. Each decision is owned by one detailed doc and one workpack.

| ID | Decision | Canonical doc | Workpack | Why it matters |
| --- | -------- | ------------- | -------- | -------------- |
| AIF-001 | Auth provider and custody boundary | [AUTH_PROVIDER_DECISION.md](AUTH_PROVIDER_DECISION.md) | [01-auth-provider-decision](workpacks/01-auth-provider-decision.md) | Prevents identity provider drift from becoming product-data drift. |
| AIF-002 | Household and role authority model | [IDENTITY_AUTHORITY_MODEL.md](IDENTITY_AUTHORITY_MODEL.md) | [02-identity-household-role-model](workpacks/02-identity-household-role-model.md) | Keeps users, households, child profiles, devices, and support access separate. |
| AIF-003 | Session and token lifecycle model | [SESSION_TOKEN_MODEL.md](SESSION_TOKEN_MODEL.md) | [03-session-token-lifecycle](workpacks/03-session-token-lifecycle.md) | Separates browser sessions, device credentials, invite tokens, and recovery tokens. |
| AIF-004 | Invite and recovery lifecycle model | [INVITE_RECOVERY_MODEL.md](INVITE_RECOVERY_MODEL.md) | [04-invites-recovery-lifecycle](workpacks/04-invites-recovery-lifecycle.md) | Prevents invite abuse and recovery bypass. |
| AIF-005 | Device authority matrix | [DEVICE_AUTHORITY_MATRIX.md](DEVICE_AUTHORITY_MATRIX.md) | [05-device-ownership-authz](workpacks/05-device-ownership-authz.md) | Makes parent, observer, and child-device authority explicit. |
| AIF-006 | Security proof and route gate | [PROOF_AND_TEST_INVENTORY.md](PROOF_AND_TEST_INVENTORY.md) | [06-security-proof-and-route-gate](workpacks/06-security-proof-and-route-gate.md) | Prevents PR-ready claims without negative proof and route sync. |
| AIF-007 | Parent family setup UI expectations | [UI_EXPECTATIONS.md](UI_EXPECTATIONS.md) | [07-parent-account-family-setup-ui](workpacks/07-parent-account-family-setup-ui.md) | Keeps the first-run UI honest about source, custody, and trust. |
| AIF-008 | Research anchors and UI guidance | [RESEARCH_AND_UI_GUIDANCE.md](RESEARCH_AND_UI_GUIDANCE.md) | [07-parent-account-family-setup-ui](workpacks/07-parent-account-family-setup-ui.md) | Pins the UI and provider choices to official references instead of guesses. |
| AIF-009 | Multi-owner effect fencing and recovery | [MULTI_OWNER_EFFECT_FENCING_DECISION.md](MULTI_OWNER_EFFECT_FENCING_DECISION.md) | [05-runtime-effect-fencing-coordinator](workpacks/05-runtime-effect-fencing-coordinator.md) | Prevents an Account-local snapshot ledger from pretending to atomically own Device Trust, step-up, capability, or lease truth. |

Execution summary:

- Cloudflare owns the family custody plane.
- Firebase Auth, if used, is adapter-only as an external IdP/token issuer.
- Household membership is not the same as user identity.
- Login is not authorization.
- Parent setup UI must label live local, LAN, cache, storage, stale, degraded, unavailable, and manual-required states honestly.
