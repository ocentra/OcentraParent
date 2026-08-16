# Support Bundle And Upload Workflow Contracts

## Support Bundle Redaction

`src/support-bundle-redaction.ts` owns the logging-domain schema proof for production-support bundle redaction and incident handoff rows. `src/support-bundle-redaction-read-model.ts` owns the current fixture rows. They record parent consent, release/package/runtime support metadata, support-safe diagnostic references, billing escalation manual-required state, account lookup manual-required state, and backend-upload/manual support boundaries.

This contract is metadata-only. It does not claim support backend upload, billing provider contact, account lookup execution, remote support sessions, production SLA, provider secrets, tokens, child activity, raw URLs, screenshots, journals, SQLite snapshots, private paths, command lines, keystrokes, clipboard data, or message contents.

The `production-support-status-backend-redaction-manifest-proof` slice extends this exported contract with status backend redaction manifest readiness rows. It proves only support-safe status target, queue/audit, redaction summary, and manual proof references. It does not claim status backend execution, status backend payload custody, durable payload storage, payload deletion, retry worker execution, audit persistence execution, public runtime execution, or child activity custody.

## Support Incident Workflow

`src/support-incident-workflow.ts` owns the logging-domain schema proof for the production support incident privacy/legal workflow. `src/support-incident-workflow-read-model.ts` owns the current fixture rows. They record parent consent gating, privacy/legal disclosure before export, redaction and custody audit refs, support-safe incident workflow state, backend upload manual-required state, billing escalation manual-required state, and account lookup manual-required state.

This contract is metadata-only. It does not claim support backend upload, billing provider contact, account lookup execution, remote support sessions, production SLA, public privacy policy publication, provider secrets, tokens, child activity, raw URLs, screenshots, journals, SQLite snapshots, private paths, command lines, keystrokes, clipboard data, message contents, or Ocentra-hosted child activity custody.

## Support Backend Upload Status

`src/support-backend-upload-status.ts` owns the logging-domain schema proof for production support backend upload status rows. `src/support-backend-upload-status-read-model.ts` owns the current fixture rows. They record parent-initiated and parent-consented queued, running, succeeded, failed, manual-required, backend-unavailable, and provider-unavailable states with redaction refs, audit refs, retry refs, abandon refs, failure refs, manual proof requirements, and package/runtime refs.

This contract is metadata-only. It does not claim raw child activity custody, provider secrets, remote support transcripts, real support backend upload execution, account lookup execution, billing provider execution, production SLA, or default Ocentra-hosted family data.

## Support Backend Upload Execution Runtime

`src/support-backend-upload-execution-runtime.ts` owns the logging-domain schema proof for production support backend upload execution/runtime boundary rows. `src/support-backend-upload-execution-runtime-read-model.ts` owns the current fixture rows. They record parent-consented request recording, redaction preflight readiness, manual dispatch requirements, backend/provider unavailable states, retry scheduling, and operator abandon states with status refs, runtime refs, redaction refs, audit refs, retry refs, abandon refs, and manual proof requirements.

This contract is metadata-only. It does not claim raw child activity custody, provider secrets, remote support transcripts, real support backend upload execution, account lookup execution, billing provider contact execution, remote support session execution, production SLA, or default Ocentra-hosted family data.
