# Update and Rollback Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `UPDATE_ROLLBACK_MODEL.md`
> Kind: plan reference document.

<!-- /agent-capsule -->

Update and rollback are artifact claims, not just feature flags.

## Required states

- channel
- checksum or signature verification
- update available / unavailable / manual-required
- rollback available / unavailable / manual-required
- failure recovery and teardown state

## Rules

- Update proof must show both success and failure paths.
- Rollback proof must show a teardown or revert path.
- No artifact may claim a production update channel without a real signed or packaged release state.
- Child runtime distribution does not inherit parent update claims.
