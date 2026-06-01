# Ocentra Parent Desktop

This workspace is the production desktop shell for the parent portal. The Vite portal remains the fast HMR surface for development, while this Tauri app embeds the same built UI for desktop distribution.

The desktop shell does not execute child-device capture, policy enforcement, or AI model work. It connects to the local Ocentra Parent agent through the typed portal WebSocket path and displays only the read models the service exposes.

Parent mobile runtime proof uses the same contract boundary as a shell wrapper: Android and iOS package mechanics may prove launch scaffolds, but parent mobile remains observer/request-first, routes assistant/report work to LAN service providers when available, and never runs local model execution by default.

Useful commands:

```powershell
cmd /c npm run build --workspace @ocentra-parent/parent-desktop
cmd /c npm run tauri:dev --workspace @ocentra-parent/parent-desktop
cmd /c npm run tauri:build --workspace @ocentra-parent/parent-desktop
```

For parallel worker demos, keep running the lane-specific Rust agent and Vite portal ports from the hub assignment, then use the Vite URL for visual HMR validation.

## Ownership

- Packages the parent portal as a desktop app for parent-owned devices.
- Connects to local, LAN, relay, cache, or parent-owned storage paths through
  typed contracts as those paths become available.
- Presents parent-controller and parent-observer status without taking
  child-agent authority.

## Must Not Own

- Child-device capture, policy evaluation, enforcement, timers, or local model
  execution.
- Silent local storage of child evidence outside the documented custody model.
- Platform claims that are only true for the Vite dev portal.

## Connected Docs

- [Platform expectations](../../docs/expectations/platforms.md)
- [Release installer expectations](../../docs/expectations/release-installer.md)
- [Remote access expectations](../../docs/expectations/roadmap-v2-parent-owned-remote-access-cloud-relay.md)

## Gaps To Fill

- Production packaging and signing.
- Route-status UX for local, LAN, relay, cache, and unavailable sources.
- Parent assistant/report workflows that call real service/provider paths.
