# portal-ux-household-surfaces-plan Event Architecture Instruction

## Owns

- portal route map, route panels, visible state labels, dev/test command panels, accessibility and responsive UI proof.

## Must not own

- runtime truth for account, policy, LAN, tracking, screen, remote, device trust, custody, or enforcement.

## Required chain

```text
portal dev/test command or route action
-> typed command/read-model request
-> owner service/domain handles it
-> portal consumes read model
-> Playwright verifies UI plus log/event/read-model artifact
```

## Logging/proof

Log command click, route id, read-model request, rendered state, degraded/manual-required state, and source owner for every displayed claim.

## Tests

Portal-domain tests cover route contracts and view models. Apps/portal tests cover rendering and dev commands. Playwright proof must be tied to logs/events/read models.

## First architecture slice

Finish start-route plus LAN consumer truth. Broad household closure waits account and policy owner proof.
