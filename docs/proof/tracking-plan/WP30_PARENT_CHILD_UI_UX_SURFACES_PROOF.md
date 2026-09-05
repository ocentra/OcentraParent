# WP30 Parent And Child UI/UX Surfaces Proof Manifest

Recorded: 2026-09-05

## Bounded result

`WP-tracking-plan-30-parent-and-child-ui-ux-surfaces` is complete for the
reviewed product-consumer truthfulness slice. The real `PolicyTracking` route
consumes the typed Rust tracking snapshot, fixed proof fixtures remain confined
to `ProofPanels`, malformed and missing snapshots fail closed, and the hosted
desktop and mobile product routes render an honest unavailable state without
fabricated tracking rows.

This manifest retains the ignored generated bundle at
`output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/`.

## Reviewed implementation and tests

- Rust route-state owner: `crates/parent-runtime-core/src/parent_ui_bridge/live_activity/tracking_panel.rs`, its `tracking_panel/` modules, and `crates/parent-runtime-core/src/parent_ui_bridge/live_activity/snapshot/tracking.rs`.
- Portal-domain projection: `packages/portal-domain/src/tracking-status-panel.ts` and `packages/portal-domain/src/tracking-status-panel-helpers.ts`.
- Product renderer: `apps/portal/src/ParentPortalRoute.tsx`, `apps/portal/src/TrackingStatusRoutePanel.tsx`, and `apps/portal/src/tracking-status-route-panel-body.tsx`.
- Vendor consumer: `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface.tsx`.
- Behavioral TypeScript tests: `packages/portal-domain/tests/unit/tracking-status-panel.test.ts`, `apps/portal/tests/unit/tracking-status-panel.test.ts`, and `apps/portal/tests/e2e/tracking-hosted-ui-proof.spec.ts`.
- Behavioral Rust tests: `crates/parent-runtime-core/tests/integration/parent_ui_bridge/runtime_and_activity_tests/routes.rs` and `crates/parent-runtime-core/tests/integration/parent_ui_bridge/runtime_and_activity_tests/actions.rs`.

## Validation evidence

| Boundary | Result | Evidence |
| --- | --- | --- |
| Hosted UI proof | passed; Playwright 2/2 | `ocentra-parent.tracking-plan-hosted-ui-proof-20260905004419-ad74d514` |
| Portal tests | 54 files, 235 passed | focused workspace run |
| Portal-domain tracking tests | 1 file, 16 passed | focused workspace run |
| Rust Policy Tracking route | 2 passed | focused parent-runtime integration run |
| Rust retention action projection | 1 passed | focused parent-runtime integration run |
| Scoped architecture and generated artifacts | passed | focused repository gates |

The hosted proof reached the service health endpoint and then rendered the
explicit unavailable state because the parent-local bridge Account owner is
unavailable. That is the current intended fail-closed runtime boundary, not a
successful live tracking/provider claim. Positive typed snapshot rendering is
covered by the behavioral Portal and Rust tests.

Visual inspection of the generated desktop and mobile screenshots found no
overlap, clipped controls, fixture cards, or fabricated data rows. The product
route presents one coherent unavailable-state group and leaves the remaining
canvas empty instead of filling it with false data.

## Generated proof bundle hashes

| File | SHA-256 | Bytes |
| --- | --- | ---: |
| `11-ui-snapshots/hosted-parent-devices-shell.png` | `62e7b2c1f8a19a3c73579e197c4a78d14f36eb94dff6f3bac451208bbbe9344f` | 708729 |
| `11-ui-snapshots/hosted-parent-overview-shell.png` | `7e3d146b0fa2276437e70f096a83193b6527be297d17a70590050664aadd2e14` | 763796 |
| `11-ui-snapshots/hosted-policy-tracking-unavailable-mobile.png` | `7e634c6f150956b8e58c482d8ccf4a93fc073eaf6013d93c27e692b468babb2e` | 228054 |
| `11-ui-snapshots/hosted-policy-tracking-unavailable.png` | `26f5c020acc798840b68801851790b9d50919d732333950a70c041d87b990823` | 807662 |
| `11-ui-snapshots/hosted-proof-panels-tracking-unavailable.png` | `fa235a13265962d16cb535585e1485e4ab10cd307a5d0bf9fdbfe4096a0f07d3` | 733870 |
| `12-playwright-proof.log` | `df9c282df6d4eff53dc7f2db61b07d904bdac687a821118506de903b5ba6ba2c` | 557 |
| `13-security-negative-proof.log` | `4ae5eaf83c5cd5b66b41783e538743edb0871b6d89130a38a9f1d034c1bd5d80` | 774 |
| `16-validation-commands.log` | `a46b842c0fb349da60940f1d6dc5eec99ed811422d1a302bb82a2a1add8f88b3` | 441 |
| `17-hosted-ui-proof.json` | `798a85a6135b3b5ad019af7b0de78d84cdc8da7741609382b85063ce2036e533` | 7771 |

## No-claim boundary

This proof does not establish live coordinates or accuracy, permission or
provider ownership, authenticated child delivery, retention mutation,
notification delivery, policy action authority, remote control, durable
tracking ingress or replay, physical-device behavior, plan-wide completion, PR
readiness, CI, `develop`, or `main` completion.
