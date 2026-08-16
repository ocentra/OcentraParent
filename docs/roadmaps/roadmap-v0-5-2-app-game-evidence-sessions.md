<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.5.2 App And Game Evidence Sessions Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V0.5.2 App And Game Evidence Sessions Expectations

This is the milestone-specific expectation file for V0.5.2 in `docs/product-roadmap.md`.

Supporting expectation files: [app and game evidence](../expectations/app-game-evidence.md), [capture](../expectations/capture.md), [evidence storage](../expectations/evidence-storage.md), [policy](../expectations/policy.md), [enforcement](../expectations/enforcement.md), [portal](../expectations/portal.md), [platforms](../expectations/platforms.md), and [platform deliverables](../expectations/platform-deliverables.md).

## Outcome

- Native app/game activity becomes queryable as typed sessions before policy or enforcement depends on it.
- Running time, foreground time, first/last seen, run count, evidence refs, and unknown states come from stored evidence.
- AI may consume stored app/game evidence or digests, but it must not scan processes, windows, files, or launchers itself.
- Platform-specific process/window/app inventory limits are explicit capability
  states instead of hidden assumptions.

## Acceptance

- The system distinguishes process, launcher, known game/app, foreground session, and unknown attribution where evidence allows.
- Policy targets can reference app/game sessions, categories, titles, launchers, and time budgets.
- Child-facing and parent-facing states are ready for later block, terminate, time-limit, and ask-parent handoff.
- Windows, macOS, Linux, Android, and iOS app/game session support is claimed
  only where platform adapters can prove it.

## Validation

- Run `npm run validate`.
- Include domain parser tests, Rust read-model tests, journal replay tests, and portal summary checks.
