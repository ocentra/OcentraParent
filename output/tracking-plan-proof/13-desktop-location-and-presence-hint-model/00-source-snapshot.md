# WP13 Desktop Location And Presence Hint Source Snapshot

- Branch: codex/tracking-plan-full-continuation-a
- Commit: 6c2ccb6802870db9d0e81abf941661fd47c073f7
- Git status at proof generation:

```text
M docs/features/location-geofence-device-status.md
 M docs/plans/tracking-plan/implementation-checklist.md
 M docs/plans/tracking-plan/workpacks/13-desktop-location-and-presence-hint-model.md
 M docs/plans/tracking-plan/workpacks/33-proof-gates-fixtures-rollout-and-pr-gate.md
 M output/tracking-plan-proof/13-desktop-location-and-presence-hint-model/00-source-snapshot.md
 M output/tracking-plan-proof/13-desktop-location-and-presence-hint-model/01-contract-proof.log
 M output/tracking-plan-proof/13-desktop-location-and-presence-hint-model/13-security-negative-proof.log
 M output/tracking-plan-proof/13-desktop-location-and-presence-hint-model/16-validation-commands.log
 M output/tracking-plan-proof/13-desktop-location-and-presence-hint-model/proof-summary.json
?? packages/parent-domain/src/tracking-desktop-presence-hint-proof.ts
?? packages/parent-domain/tests/tracking-desktop-presence-hint-proof.test.ts
?? scripts/test/tracking-desktop-presence-hint-proof.mjs
```

- Scope: parent-domain desktop presence hint rows for OS-location manual-required, LAN/Wi-Fi/IP hint-only, manual check-in, stale, offline, and missing-device states.
- No desktop OS location runtime, precise GPS, physical presence, physical-device proof, or production behavior is claimed.
