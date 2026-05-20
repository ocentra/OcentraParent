# LAN Pairing Expectations

LAN features expose the child-device agent beyond loopback and must be treated as trust-boundary work.

## Expected Deliverables

- Explicit LAN enablement.
- Origin allowlist.
- Pairing proof contract.
- Trusted device registry.
- Device identity display.
- Multi-device command routing.

## Acceptance

- Anonymous LAN control is rejected.
- Loopback remains the default.
- Pairing state is auditable.
- Portal can distinguish devices.
- Tests cover rejected and accepted routes.
- Parent-visible UI makes the selected device clear.

## Non-Goals

- Do not treat LAN as production auth.
- Do not expose broad unauthenticated control APIs.
- Do not skip origin checks because a workflow is dev-only.

## Done Signal

A parent portal can discover or pair with a local agent only through explicit trusted flow, and the agent rejects anonymous or incorrectly routed LAN commands.
