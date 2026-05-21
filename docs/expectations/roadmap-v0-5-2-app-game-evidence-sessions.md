# V0.5.2 App And Game Evidence Sessions Expectations

This is the milestone-specific expectation file for V0.5.2 in `docs/product-roadmap.md`.

Supporting expectation files: [app and game evidence](app-game-evidence.md), [capture](capture.md), [evidence storage](evidence-storage.md), [policy](policy.md), [enforcement](enforcement.md), [portal](portal.md), [platforms](platforms.md), and [platform deliverables](platform-deliverables.md).

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
