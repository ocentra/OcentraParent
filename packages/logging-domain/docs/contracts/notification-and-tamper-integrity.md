# Notification And Tamper Integrity Contracts

## Notification Audit History

`src/notification-audit-history.ts` owns the logging-domain proof for notification audit/history rows. It records provider status, retry lifecycle, receipt/manual-required refs, quiet-hours/escalation refs, redaction-safe payload fields, and child-data non-custody flags.

`src/notification-audit-history-handoff.ts` owns the metadata-only handoff from local source rows into those audit/history rows. The current app/game proof maps linked local outbox rows to queued audit entries and manual/unavailable rows to blocked audit entries while preserving source audit, evidence, and policy refs.

This contract is metadata-only. It does not claim provider adapters, send/retry execution, webhook receipt ingestion, credentials, notification history UI, raw child data, raw evidence payloads, or Ocentra-hosted child evidence custody.

## Tamper Integrity Audit

`src/tamper-integrity-audit.ts` owns the logging-domain proof for tamper/integrity audit rows. It records stale/offline heartbeat, permission loss, stopped service, removed agent, uninstall detection, tamper/manual-required, and admin-removal flow states with redaction-safe operational refs.

This contract is metadata-only. It does not claim stealth behavior, privilege escalation, hidden persistence, notification provider delivery, admin-removal blocking, raw child data, raw evidence payloads, raw URLs, screenshots, command lines, private paths, or message contents.
