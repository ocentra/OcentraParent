# logging-domain-parity Event Architecture Instruction

## Owns

- shared log contracts;
- test-log and app-log schemas;
- proof-trace bridge and query surfaces;
- redaction and support-log boundaries.

## Must not own

- domain-specific product decisions;
- plan-specific proof meaning;
- runtime authority for tracking, screen, policy, LAN, AI, or payment.

## Required chain

```text
proof run starts with run id and correlation id
-> each owner logs boundary milestones
-> log bridge/storage captures structured lines
-> query/proof tool extracts relevant chain
-> plan proof cites logs plus events/read models
```

## Logging/proof

This plan must make logging deterministic for tests and proof. Fresh-root proof must not depend on ambient old logs.

## Tests

Logging tests belong in `logging-domain`, dev scripts, and portal/dev-log consumer tests. Other plans should consume logging APIs, not reimplement logging formats.

## First architecture slice

Close WP03 portal/dev-log consumer truth, then WP06 checker/enforcement hardening. Also define a small reusable proof-chain log recipe for other plans to follow.
