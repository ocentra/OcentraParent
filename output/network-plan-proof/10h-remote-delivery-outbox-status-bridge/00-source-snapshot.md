# Row10h Remote Delivery Outbox Status Bridge Source Snapshot

proofRevision=network-remote-delivery-outbox-status-bridge-proof/v1
scopeMarker=row10h-remote-delivery-outbox-status-bridge
sourceBase=c99e70b85e33090dfa85d6dfe9df41da9d875fb1
worktreeStatus=expected to contain generated row10h proof artifacts until committed

Inspected paths:
- scripts/test/network-remote-delivery-outbox-status-bridge-proof.mjs
- crates/agent-protocol/src/constants/network_flow.rs
- crates/agent-protocol/src/network_flow.rs
- crates/agent-protocol/src/network_flow_tests.rs
- crates/agent-service/src/network_remote_delivery_status_payload.rs
- crates/agent-service/src/network_remote_delivery_status_service_tests.rs
- crates/agent-service/src/websocket.rs
- packages/agent-protocol-domain/src/defaults.ts
- packages/agent-protocol-domain/src/network-remote-delivery-status.ts
- packages/agent-protocol-domain/tests/network-remote-delivery-status.test.ts
- docs/features/network-domain-control.md
- docs/plans/network-plan/implementation-checklist.md
- docs/plans/network-plan/workpacks/README.md
- crates/agent-protocol/README.md
- crates/agent-service/README.md
- packages/agent-protocol-domain/README.md

Before-state gap:
- Row10g produced prepared outbox candidates, and row10k now carries those refs through the current typed status payload.

Current bridge boundary:
- The row10h bridge evidence remains preserved in the row10k status payload through row10g outbox refs, handoff refs, replay/support refs, prepared candidate counts, duplicate rejection, and zero dispatch/ack counters.
- The row10k payload adds manual-required blocked dispatch refs while remaining read-only and keeping live transport, remote acknowledgement, product-ready delivery, policy authority, adapter execution, exact content, and host filtering unclaimed.
