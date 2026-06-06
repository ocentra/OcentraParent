# SOCIAL-21 Contract And Renderer Proof

The child approval/block UX snapshot requires child-facing state/action
contracts for:

- approval-request pending;
- blocked social route candidate;
- warning social route candidate;
- manual-review required;
- time-limit candidate;
- native-app unavailable.

Approval-pending surfaces must reference a parent approval request and stay in a
waiting-parent state. Blocked social route candidates must reference a gate plan
but remain contract-only. Warning and time-limit candidates are child-readable
without applying an action. Native app rows stay unavailable, and manual review
rows stay manual-required.

The parent-domain focused Vitest suite accepts an honest six-surface snapshot
and rejects missing surfaces, unproven runtime claims inside the source
snapshot, and unsupported state or action upgrades.

The text-domain suite accepts schema-backed child-facing title, body, and action
tokens. Exact-copy checks prove approval, block, and native-unavailable copy
stays calm and manual-required without implying notification delivery,
time-limit application, connector authorization, native app control, or
enforcement.

The portal-domain renderer bridge suite converts the honest snapshot into the
shared `BrowserChildInterventionPageModel` used by
`renderBrowserChildInterventionPage`. The proof script then writes rendered HTML
to the configured child-agent intervention HTML path, serves it through the real
Rust child-agent `/api/browser/intervention/page` endpoint with `no-store`, and
captures screenshots for each mapped social intervention state.
