# SOCIAL-21 Source Snapshot

SOCIAL-21 now maps the existing child approval/block UX contracts into the
shared child browser intervention renderer and serves the rendered HTML through
the real Rust child-agent intervention endpoint.

The original contract and copy proof remains the state source:

- `packages/parent-domain/src/social-child-approval-block-ux-values.ts` defines
  child-facing surface ids, surface kinds, states, actions, reason codes, and
  the schema version.
- `packages/parent-domain/src/social-child-approval-block-ux.ts` defines child
  approval/block surface and snapshot schemas plus the decode helper.
- `packages/parent-domain/tests/social-child-approval-block-ux.test.ts`
  verifies honest child-facing state/action contracts and negative overclaim
  rejection.
- `packages/text-domain/src/social-child-approval-block-ux-text.ts` defines
  child-facing approval, warning, block, manual-required, time-limit, native
  unavailable, and action text tokens.
- `packages/text-domain/tests/social-child-approval-block-ux-text.test.ts`
  verifies calm schema-backed copy and rejects blame, surveillance, AI-blocking,
  credential, or message-content language.
- `packages/text-domain/package.json` exports the text-token subpath.

The rendered bridge proof is added under:

- `packages/portal-domain/src/social-child-intervention-page-model.ts`
  converts honest SOCIAL-21 snapshots into `BrowserChildInterventionPageModel`
  values for approval-hold, block, warn, parent-review, time-limit, and native
  unavailable states.
- `packages/portal-domain/tests/social-child-intervention-page-model.test.ts`
  verifies the renderer bridge and rejects dishonest rendered-UI claims when
  the input snapshot itself claims runtime behavior it did not prove.
- `scripts/test/social-child-intervention-page-proof.mjs` builds contracts,
  starts the real Rust child-agent service with
  `OCENTRA_PARENT_MANAGED_BROWSER_INTERVENTION_HTML_PATH`, fetches
  `/api/browser/intervention/page?target=...`, and captures screenshots of the
  served no-store pages.

The row still avoids notification delivery, browser navigation block execution,
time-limit application, connector authorization, native app control, final
policy execution, and enforcement.
