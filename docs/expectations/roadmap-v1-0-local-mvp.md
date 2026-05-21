# V1.0 Local MVP Expectations

This is the milestone-specific expectation file for V1.0 in `docs/product-roadmap.md`.

Supporting expectation files: [evidence storage](evidence-storage.md), [capture](capture.md), [portal](portal.md), [policy](policy.md), [release installer](release-installer.md), [platforms](platforms.md), and [platform deliverables](platform-deliverables.md).

## Outcome

- A Windows-first parent can install the child-device agent, open a local/LAN parent portal, and see real local activity evidence.
- Local evidence, local AI dry-run policy preview, local-only reports, and update scaffolding work without Ocentra-hosted custody of family activity data.
- Main is green and ready for a visible human test checkpoint.

## Acceptance

- Windows install/uninstall, service autostart, restart survival, journal/query rebuild, process/window capture, network/domain observation, and portal visibility are proven.
- The local evaluator can dry-run a narrow page, video-link, app, or domain observation against parent rules.
- The product clearly labels what is captured, inferred, degraded, unavailable, or out of scope.
- Linux, macOS, Android, and iOS are documented as CI/package/scaffold,
  future-platform, unavailable, or blocked claims instead of being implied by
  the Windows MVP.

## Validation

- Run `npm run validate`.
- Include package smoke, local service smoke, portal Playwright, manual Windows install/run notes, and CI green on `main`.
- Include the platform deliverables matrix in the V1.0 handoff so users can see
  which platforms are product-ready and which are only scaffolded.
