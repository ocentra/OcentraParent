# @ocentra-parent/logging-domain

Parent-local logging runtime helpers and focused proof coverage built on centralized schemas.

## Public Surface

- `./core/logConfig`
- `./core/logDecisionProvider`
- `./core/logger`
- `./core/stackTrace`
- `./test-log/bridgeConvert`
- `./test-log/ingestManifest`
- `./test-log/logsTree`
- `./test-log/ndjsonLogFileWriter`
- `./test-log/ndjsonPaths`
- `./test-log/ndjsonWriter`
- `./test-log/testLogDuckDb`
- `./test-log/testLogRetention`
- `./test-log/wipeNdjsonScope`
- `./transport/bridgeServer`
- `./transport/bridgeTransport`
- `./app-log/appNdjsonWriter`
- `./app-log/createAppLogStorage`
- `./package-info`

## What Stays Here

- parent-local logging runtime helpers
- NDJSON and DuckDB developer observability helpers
- bridge/server transport utilities for local logging flows
- app-log storage helpers
- focused unit and integration proofs for canonical logging-related schemas

## What Does Not Stay Here

- shared logging, app-log, test-log, support-workflow, provider-secret, privacy/legal, tamper, or delete-executor schema ownership
- cross-package schema/value/id/read-model contract definitions that now live in `@ocentra-parent/schema-domain/*`
- policy authority, feature-specific product behavior, child evidence custody, or support-backend execution claims

## Current Shape

This package no longer publishes local schema mirrors such as:

- `./contracts`
- `./test-log/types`
- `./test-log/ndjsonBrands`
- `./transport/bridgeLogPayload`
- `./app-log/types`
- support/status/privacy/provider/tamper/read-model schema leaves that were centralized into `schema-domain`

The live tests in this package should import those canonical schemas directly from `@ocentra-parent/schema-domain/*` while keeping runtime helpers local here.

## Validation Intent

Keep validation focused on:

- building `@ocentra-parent/logging-domain`
- proving the local runtime exports still work
- proving in-package tests parse the canonical centralized schemas
- architecture lint for this package boundary
