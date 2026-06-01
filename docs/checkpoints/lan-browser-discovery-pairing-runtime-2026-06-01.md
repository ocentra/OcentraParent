# LAN Browser Discovery Pairing Runtime Checkpoint

Date: 2026-06-01

Branch: `codex/lan-browser-discovery-pairing-runtime`

## Scope

- Adds explicit browser-first LAN runtime command/event contracts for discovery scan and add-device request.
- Routes those commands through the Rust service LAN pairing runtime rather than portal fixtures.
- Keeps discovery honest: the service emits the current local-service command target or trusted registry state, while physical household LAN remains `manual-required`.
- Keeps cloud relay and remote desktop/control out of scope.

## Runtime Events

- `agent.lan-pairing.browser-discovery.scan` reports `agent.lan-pairing.browser-discovery.reported`.
- `agent.lan-pairing.add-device.request` reports `agent.lan-pairing.add-device.reported` after a valid challenge request.
- Wrong-origin add-device requests reject without trusting a device.
- Paired selected routes expose trusted registry entries and selected-device readiness.

## Non-Claims

- No fake household devices.
- No physical two-device/router/firewall proof.
- No cloud relay.
- No remote desktop/control.
