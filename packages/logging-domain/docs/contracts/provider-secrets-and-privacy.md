# Provider Secret And Privacy Contracts

## Provider Secret Custody Status

`src/provider-secret-custody-status.ts` owns the logging-domain schema proof for production support provider-secret custody status rows. `src/provider-secret-custody-status-read-model.ts` owns the current fixture rows. They record custody-boundary recorded, provider-secret absent, backend secret store manual-required, rotation manual-required, revocation manual-required, and audit-export-ready states with legal/provider readiness, billing support, redaction, custody audit, rotation, revocation, manual proof, and audit export refs.

This contract is metadata-only. It does not claim provider secrets, payment provider tokens, raw child activity, raw support bundle payloads, account lookup results, billing provider contact records, remote support transcripts, real provider-secret custody, backend secret store execution, rotation execution, revocation execution, support backend upload execution, account lookup execution, billing provider contact execution, remote support session execution, production SLA, or default Ocentra-hosted family data.

## Provider Secret Execution Readiness

`src/provider-secret-execution-readiness.ts` owns the logging-domain schema proof for provider-secret execution readiness rows. `src/provider-secret-execution-readiness-read-model.ts` owns the current fixture rows. They record execution boundary, backend secret-store preflight, rotation preflight, revocation preflight, operator approval, manual execution, and support-safe audit export states with custody status, preflight, operator, manual proof, and audit refs.

## Provider Secret Rotation Revocation Status

`src/provider-secret-rotation-revocation-status.ts` owns the logging-domain schema proof for provider-secret rotation and revocation status rows. `src/provider-secret-rotation-revocation-status-read-model.ts` owns the current fixture rows. They record rotation requested, rotation preflight-ready, rotation manual-required, revocation requested, revocation preflight-ready, revocation manual-required, and audit-export-ready states with custody status, execution readiness, backend secret-store preflight, operator approval, manual proof, and audit refs.

These contracts are metadata-only. They do not claim provider secrets, payment provider tokens, raw child activity, raw support bundle payloads, account lookup results, billing provider contact records, remote support transcripts, backend secret store execution, rotation execution, revocation execution, provider-secret delivery, support backend upload execution, account lookup execution, billing provider contact execution, remote support session execution, production SLA, or default Ocentra-hosted family data.

## Privacy Legal Disclosure Status

`src/privacy-legal-disclosure-status.ts` owns the logging-domain schema proof for production support privacy/legal disclosure status rows. `src/privacy-legal-disclosure-status-guards.ts` owns the sensitive-data and overclaim rejection guards, and `src/privacy-legal-disclosure-status-read-model.ts` owns the current fixture rows. `scripts/test/production-support-privacy-legal-disclosure-status-proof.mjs` regenerates the deterministic proof artifacts. They record disclosure requested, parent-authorized, legal-review queued, legal-review running, parent-notification ready, publication-ready, failed, and manual-required states with parent consent, privacy policy, legal review, publication, support runbook, audit, failure, and manual proof refs.

This contract is metadata-only. It does not claim legal disclosure execution, public runtime execution, support backend upload execution, account lookup, billing provider contact, remote support sessions, production SLA, provider secrets, remote support transcripts, raw child activity custody, or raw support bundle payloads.
