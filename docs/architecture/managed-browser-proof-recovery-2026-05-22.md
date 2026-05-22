# Managed Browser Proof Recovery - 2026-05-22

## Scope

This record covers the V0.7 managed-browser URL/title proof recovery on branch
`codex/managed-browser-proof-recovery`.

The bug reproduced on Windows with Chrome installed at
`C:\Program Files\Google\Chrome\Application\chrome.exe`: external DevTools
requests to the controlled managed profile succeeded, but the Rust service path
returned `browser-bridge-io-error` and `agent.browser.evidence.recent.get`
returned zero rows.

## Fix

`crates/agent-core/src/browser_bridge_http.rs` now treats a DevTools HTTP
response with `Content-Length` as complete once the body bytes are present,
instead of waiting for the browser to close the TCP stream. Chromium can keep
the socket open long enough for the old read path to time out even when
`/json/version` and `/json/list` already returned valid JSON.

The fix keeps the existing managed-browser boundary:

- loopback-only bridge endpoint;
- Ocentra-managed profile path;
- Rust service command `agent.browser.managed.bridge.poll`;
- encrypted journal and SQLite ingest before read-model use;
- `tab-list-only` active-state honesty, not foreground active-tab certainty.

## Proof Commands

```powershell
cargo test -p ocentra-parent-agent-core browser_bridge
cargo build -p ocentra-parent-agent-service
cmd /c npm run test:managed-browser-service-proof
```

Observed service proof result:

```text
managed-browser-service-proof-ok=true
url=https://example.com/?ocentra_service_proof=1 title=example.com domain=example.com activeState=unknown capability=tab-list-only
```

The proof script writes detailed JSON under
`test-results/managed-browser-service-proof/`.

## Known Gaps

- The service proves exact URL/title/domain for a managed Chromium profile, but
  the active tab state remains `unknown` with capability `tab-list-only`.
- Hosted CI still cannot prove a real household managed-browser session.
- This does not add V0.8 enforcement, blocking, notification delivery, or model
  execution.
