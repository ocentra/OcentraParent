# Browser Runtime Social Provider Receipt Service Public Fields Proof

This proof adds Rust protocol field constants and service payload fields for the existing social provider receipt status path.

The service payload now exposes social provider receipt boundary rows, provider-dispatch-required rows, manual-receipt-required rows, provider attempt refs, receipt proof refs, durable result refs, durable store refs, read-model refs, and support-status refs. Manual-required rows publish zero durable rows and empty durable refs.

This does not update TypeScript defaults, the shared parser, or portal state while another lane owns the shared protocol defaults file. Provider delivery, receipt ingestion runtime, connector/native runtime, parent notification UI delivery, report delivery, final policy execution, and enforcement remain unclaimed.

Validation:
- `cargo test -p ocentra-parent-agent-service social_provider_receipt --quiet`
