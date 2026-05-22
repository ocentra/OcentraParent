# Ocentra Parent Desktop

This workspace is the production desktop shell for the parent portal. The Vite portal remains the fast HMR surface for development, while this Tauri app embeds the same built UI for desktop distribution.

The desktop shell does not execute child-device capture, policy enforcement, or AI model work. It connects to the local Ocentra Parent agent through the typed portal WebSocket path and displays only the read models the service exposes.

Useful commands:

```powershell
cmd /c npm run build --workspace @ocentra-parent/parent-desktop
cmd /c npm run tauri:dev --workspace @ocentra-parent/parent-desktop
cmd /c npm run tauri:build --workspace @ocentra-parent/parent-desktop
```

For parallel worker demos, keep running the lane-specific Rust agent and Vite portal ports from the hub assignment, then use the Vite URL for visual HMR validation.
