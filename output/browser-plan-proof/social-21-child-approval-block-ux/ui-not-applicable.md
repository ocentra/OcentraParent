# UI Proof Now Applicable

SOCIAL-21 started as a parent-domain child approval/block UX contract plus
text-domain copy-token proof. The row now includes a rendered child UI proof via
the shared child browser intervention renderer merged from PR399.

The current screenshots under `06-ui-snapshots` are captured from Playwright
after loading the real Rust child-agent endpoint
`/api/browser/intervention/page?target=...`. They prove rendered approval-hold,
block, warn, parent-review, time-limit candidate, and native-unavailable pages
are served with the shared bridge payload.

This proof still does not claim notification delivery, browser navigation block
execution, applied time limits, connector authorization, native app control,
final policy execution, or enforcement.
