# SOCIAL-16 Source Snapshot

SOCIAL-16 adds contract-only Android native social app capability matrix support
under `packages/parent-domain`.

- `packages/parent-domain/src/social-android-native-app-capability-matrix-values.ts`
  defines Android native social capability surfaces, target kinds, capability
  states, proof states, policy scopes, reasons, proof refs, and boundaries.
- `packages/parent-domain/src/social-android-native-app-capability-matrix.ts`
  defines capability row, claim-boundary, and matrix schemas plus decode helper.
- `packages/parent-domain/tests/social-android-native-app-capability-matrix.test.ts`
  verifies honest app-level/manual Android social capability rows and negative
  overclaim rejection.

The matrix reuses parent-domain product capability vocabulary and does not add
public package/barrel exports in this row.
