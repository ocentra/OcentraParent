<!-- agent-capsule -->

> Agent Capsule
> Doc: V1.0 Local MVP Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V1.0 Local MVP Expectations

This is the milestone-specific expectation file for V1.0 in `docs/product-roadmap.md`.

Supporting expectation files: [evidence storage](../expectations/evidence-storage.md),
[capture](../expectations/capture.md), [portal](../expectations/portal.md), [family setup](../expectations/family-setup.md),
[policy](../expectations/policy.md), [release installer](../expectations/release-installer.md),
[platforms](../expectations/platforms.md), and
[platform deliverables](../expectations/platform-deliverables.md).

## Outcome

- A Windows-first parent can install the child-device agent, open a local/LAN parent portal, and see real local activity evidence.
- A single-household setup path can represent at least one parent, one child
  profile, one child device, and the current device role/status.
- Local evidence, local AI dry-run policy preview, local-only reports, and update scaffolding work without Ocentra-hosted custody of family activity data.
- Main is green and ready for a visible human test checkpoint.

## Acceptance

- Windows install/uninstall, service autostart, restart survival, journal/query rebuild, process/window capture, network/domain observation, and portal visibility are proven.
- Setup state, child profile, device role, and source/custody labels are visible
  and backed by typed service state.
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
