# WP205 App/Game Platform Proof Status Runtime Detail Refs

## Scope

Carry the new Android, Windows, and Linux runtime detail proofs from WP202-WP204
into the shared app/game platform proof status read model.

This keeps one platform status spine for native apps and native games while
making the richer runtime evidence parent/service-visible:

- Android Accessibility runtime declaration proof.
- Windows local AppLocker/App Control policy evidence proof.
- Linux active-window tool/ref proof.

## Implementation

- Updated `packages/parent-domain/src/app-game-platform-proof-status.ts` to
  accept and expose:
  - `android-accessibility-runtime-proof-ref`;
  - `windows-local-policy-evidence-proof-ref`;
  - `linux-active-window-tool-proof-ref`.
- Updated `scripts/test/app-game-platform-proof-status-proof.mjs` so the
  platform status proof harness consumes the WP202-WP204 proof artifacts.

## Validation

Focused validation for this workpack:

```powershell
cmd /c node --check scripts/test/app-game-platform-proof-status-proof.mjs
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-platform-proof-status app-game-android-accessibility-runtime-proof app-game-windows-local-policy-evidence-proof app-game-linux-active-window-tool-proof
cmd /c npm run build --workspace @ocentra-parent/parent-domain
```

## Proof

The full platform proof harness remains:

- `test-results/app-game-platform-proof-status-proof/proof.json`
- `output/app-game-plan-proof/183-app-game-platform-proof-status-surface/proof.json`
- `output/app-game-plan-proof/184-app-game-platform-proof-status-service-surface/proof.json`
- `output/app-game-plan-proof/195-app-game-platform-proof-status-preflight-detail-refs/proof.json`

## Boundaries

Proved:

- The platform status read model can carry Android Accessibility runtime,
  Windows local policy evidence, and Linux active-window tool proof refs.
- These refs stay visibility-only in the shared status model.
- Adapter dispatch, broad installed-app blocking, platform enforcement,
  provider delivery, child-device delivery, raw policy XML, raw executable
  paths, raw service names, raw event rows, raw window titles, and private
  diagnostics remain unclaimed.

Not proved:

- Android overlay execution or Device/Profile Owner authority.
- Windows broad installed-app launch blocking execution.
- Linux foreground capture or policy enforcement.
- macOS/iOS runtime proof outside CI/manual artifact gates.
