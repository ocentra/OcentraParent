# Status Payload Export And Deletion Contracts

## Status Backend Payload Custody

`src/status-backend-payload-custody.ts` owns the logging-domain schema proof for production support status backend payload custody rows. `src/status-backend-payload-custody-read-model.ts` owns the current fixture rows. They record custody boundary, retention manual-required, delete request, deletion manual-required, audit-export-ready, and backend-unavailable states with status target refs, queue refs, audit refs, redaction refs, custody refs, retention refs, delete refs, and manual proof refs.

This contract is metadata-only. It does not claim status backend execution, durable status backend payload storage, status backend payload deletion, retry worker execution, audit persistence, public runtime execution, provider execution, support backend upload execution, account lookup, billing provider contact, remote support sessions, production SLA, provider secrets, raw support bundles, or default Ocentra-hosted family data.

## Data Export/Delete Lifecycle

`src/data-export-delete-lifecycle.ts` owns the logging-domain schema proof for `production-support-data-export-delete-lifecycle-proof`. `src/data-export-delete-lifecycle-read-model.ts` owns the current fixture rows. They record parent-authorized export and delete requested, authorized, queued, running, succeeded, failed, and manual-required lifecycle states with local queue/runtime/output/delete refs, redaction/audit refs, custody refs, and manual proof requirements.

This contract is metadata-only. It does not claim real backend upload execution, public runtime execution, provider execution, production SLA, remote support sessions, raw child activity custody, provider secrets, remote support transcripts, or default Ocentra-hosted family data.

## Delete Executor Proof

`src/delete-executor-proof.ts` owns the logging-domain schema proof for `production-support-delete-executor-proof`. `src/delete-executor-read-model.ts` owns the current fixture rows. They record delete executor readiness/status rows for local export output, support backend payload, status backend payload, public runtime payload, and legal disclosure payload targets with delete-request, authorization, redaction/audit, custody, source-proof, and manual-proof refs.

This contract is metadata-only. It does not claim real delete execution, durable queue execution, payload deletion execution, provider execution, public runtime execution, legal execution, support backend upload execution, production SLA, raw child activity custody, raw support bundle payloads, provider secrets, remote support transcripts, or default Ocentra-hosted family data.
