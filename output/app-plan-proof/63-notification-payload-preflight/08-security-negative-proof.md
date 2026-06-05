# Security Negative Proof

- Scheduled rows require minimal alert id, family/device scope, severity, reason code, evidence ref, policy ref, and parent action link refs only.
- Payload preflight rows require raw child evidence, raw URL/title, raw message text, screenshot/report, and sensitive provider metadata exclusions.
- Schema tests reject provider payload template runtime claims and raw URL/title inclusion.
- No provider credentials, provider receipts, cloud routing, durable outbox storage, child delivery, parent UI, or adapter dispatch are claimed.
