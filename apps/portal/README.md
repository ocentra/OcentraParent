# Portal App

Minimal Vite dev portal for local and LAN agent visibility.

This is not the product portal. It exists so the Rust agent service can be seen and validated while the repo is still in scaffold mode.

```powershell
cmd /c npm run dev:agent
cmd /c npm run dev:portal
```

For cross-device LAN testing, run from the repo root:

```powershell
cmd /c npm run dev:lan
```

Run the real browser UI check from the repo root:

```powershell
cmd /c npm run playwright:install
cmd /c npm run test:e2e --workspace @ocentra-parent/portal
```

The Playwright check starts the Rust agent and Vite portal on the scaffold smoke ports, verifies WebSocket connection state, clicks command buttons, checks rendered event output, and fails on browser console or page errors.
