# WP08 redacted correlated authority proof

Evidence is correlated by packet identity
`account-identity-family/WP08/2026-08-04`. It names only source and test
identifiers, never a session credential, provider claim, invite/recovery secret,
child activity record, or worker data.

The proof covers account/household/device authorization through
`household_authority`, invite/recovery through `setup_lifecycle`, and session
decisions through `session_lifecycle`. The family-identity implementation also
models a redacted audit state for session authority.

This is retained source-and-test proof, not an assertion that a Cloudflare
worker emitted a production audit event.

The 2026-08-05 handoff recovery makes the correlation boundary explicit through
`HouseholdAuthorityEvaluationId` and records
`HouseholdAuthorityHandoffRedactionState::IdentifiersOnly`. Its serializable
decision contains only stable identifiers, action/decision state, and schema
version; contract coverage asserts that a child display name is absent.
