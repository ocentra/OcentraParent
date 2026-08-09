# Remote Access Workpack Index

Use this file to select exactly one workpack. Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear. Do not scan all workpacks by default.

| Workpack                                                                            | Purpose                                             | Status  |
| ----------------------------------------------------------------------------------- | --------------------------------------------------- | ------- |
| [01-remote-capability-fabric](workpacks/01-remote-capability-fabric.md)             | Capability, route, and standing-access model.       | Open — Rust capability-fabric contract and focused schema tests exist; pairing/relay/device-trust/session runtime and retained proof remain open |
| [02-live-screen-relay](workpacks/02-live-screen-relay.md)                           | Remote live screen/viewing path and privacy states. | Planned |
| [03-remote-input-control-authority](workpacks/03-remote-input-control-authority.md) | Remote input/control authority and limits.          | Deferred |
| [04-session-pairing-grants](workpacks/04-session-pairing-grants.md)                 | Pairing, disclosure, standing access, revocation.   | Planned |
| [05-relay-security-abuse-controls](workpacks/05-relay-security-abuse-controls.md)   | Relay security, abuse, rate limits, DoS.            | Planned |
| [06-rollout-proof-and-route-gate](workpacks/06-rollout-proof-and-route-gate.md)     | Proof and routing gate.                             | Planned |

Workpack 03 is retained for future control expansion and is not part of the current live-view pass.

## Default execution order

```text
WP01 -> WP04 -> WP02 -> WP05 -> WP06
WP03 stays deferred unless the assignment explicitly opens the future control slice.
```

## Selection rules

- Choose exactly one workpack.
- If the selected workpack owner/proof family is unclear, classify it through `WORKPACK_FAMILIES.md`; do not scan every family.
- Do not use local screen capture proof to close remote live-view proof.
- Do not use LAN pairing proof to close relay-backed remote access proof.
- Do not use live-view proof to claim remote input/control.
- Do not use relay route existence as remote readiness.
- Do not use UI-only proof as product proof.
- Do not claim readiness without pairing, standing-access visibility, revoke/remove-device, account/device-trust authority, relay degraded state, custody, and abuse-control proof or exact blockers.
