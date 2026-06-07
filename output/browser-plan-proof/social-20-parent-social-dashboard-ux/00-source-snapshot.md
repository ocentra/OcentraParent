# SOCIAL-20 Source Snapshot

SOCIAL-20 adds contract-only parent social dashboard UX support under
`packages/parent-domain` and schema-backed dashboard copy tokens under
`packages/text-domain`.

- `packages/parent-domain/src/social-dashboard-ux-values.ts` defines dashboard
  panel ids, panel kinds, statuses, actions, severities, and reason codes.
- `packages/parent-domain/src/social-dashboard-ux.ts` defines panel and
  dashboard snapshot schemas plus decode helper.
- `packages/parent-domain/tests/social-dashboard-ux.test.ts` verifies honest
  dashboard section/action/status contracts and negative overclaim rejection.
- `packages/text-domain/src/social-dashboard-ux-text.ts` defines parent-facing
  dashboard title, section, status, and action text tokens.
- `packages/text-domain/tests/social-dashboard-ux-text.test.ts` verifies the
  copy is schema-backed and avoids surveillance, hidden connector, or
  enforcement claims.
- `packages/text-domain/package.json` exports the text-token subpath.

The row intentionally avoids apps/portal and portal-domain rendered UI while
those areas are active in other lanes. It does not render a dashboard, fetch
runtime data, notify parents or children, or claim policy/enforcement behavior.
