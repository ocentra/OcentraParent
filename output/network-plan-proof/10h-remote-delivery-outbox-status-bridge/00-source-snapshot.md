# Row10h Remote Delivery Outbox Status Bridge Source Snapshot

proofRevision=network-remote-delivery-outbox-status-bridge-proof/v1
branchMarker=codex/network-row10h-remote-outbox-status-bridge-on-row10g
sourceBase=60304716a09386a4f56796eb44d62dcf19ca5f92
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
- crates/agent-protocol/readme.md
- crates/agent-service/readme.md
- packages/agent-protocol-domain/readme.md

Before-state gap:
- Row10g produced prepared outbox candidates, but the service/protocol status event still exposed only durable-envelope refs.

Current bridge boundary:
- The row10h bridge exposes row10g outbox refs, handoff refs, replay/support refs, prepared candidate counts, duplicate rejection, and zero dispatch/ack counters.
- The bridge remains read-only and keeps live transport, remote acknowledgement, product-ready delivery, policy authority, adapter execution, exact content, and host filtering unclaimed.
