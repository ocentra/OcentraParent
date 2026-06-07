# AI-19 Source Snapshot

Branch: `codex/browser-child-intervention-endpoint-flow`

Base inspected: `8e16b284`

Before-state gap:

- AI-19 had activity-domain child UX state contracts and text-domain calm copy
  tokens.
- The row remained partial because it did not prove rendered child page delivery
  through the managed-browser child-agent endpoint.
- PR399 added the shared child intervention renderer and Rust
  `/api/browser/intervention/page` endpoint, which can now carry the AI-19 child
  UX states without creating a duplicate page path.

Source paths inspected:

- `packages/activity-domain/src/browser-ai-child-ux-schemas.ts`
- `packages/activity-domain/src/browser-ai-child-ux-values.ts`
- `packages/activity-domain/tests/browser-ai-child-ux.test.ts`
- `packages/text-domain/src/browser-child-ux.ts`
- `packages/text-domain/tests/browser-child-ux.test.ts`
- `packages/portal-domain/src/browser-child-intervention-page.ts`
- `crates/agent-service/src/browser_intervention_page.rs`
- `scripts/test/managed-browser-composited-block-proof.mjs`
