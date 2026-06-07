# Row10f Remote Delivery Status Bridge Source Snapshot

proofRevision=network-remote-delivery-status-bridge-proof/v1
branchMarker=codex/network-row10f-remote-delivery-status-bridge-on-row10e
sourceBase=1afe73504408348c1e51fcd886da3e504bff8026
worktreeStatus=expected to contain generated row10f proof artifacts until committed

Inspected paths:
- scripts/test/network-remote-delivery-status-bridge-proof.mjs
- crates/agent-protocol/src/constants.rs
- crates/agent-protocol/src/constants/field.rs
- crates/agent-protocol/src/constants/network_flow.rs
- crates/agent-protocol/src/network_flow.rs
- crates/agent-protocol/src/network_flow_tests.rs
- crates/agent-protocol/src/tests.rs
- crates/agent-protocol/src/transport.rs
- crates/agent-service/src/network_remote_delivery_status_payload.rs
- crates/agent-service/src/network_remote_delivery_status_service_tests.rs
- crates/agent-service/src/main.rs
- crates/agent-service/src/websocket.rs
- packages/agent-protocol-domain/src/contracts.ts
- packages/agent-protocol-domain/src/defaults.ts
- packages/agent-protocol-domain/src/network-remote-delivery-status.ts
- packages/agent-protocol-domain/tests/network-remote-delivery-status.test.ts
- packages/agent-protocol-domain/package.json
- docs/features/network-domain-control.md
- docs/plans/network-plan/implementation-checklist.md
- docs/plans/network-plan/workpacks/README.md
- crates/agent-protocol/readme.md
- crates/agent-service/readme.md
- packages/agent-protocol-domain/readme.md

Before-state gap:
- Row10b through row10e were local proof boundaries, but the service/protocol layer did not expose a typed row10f status bridge that consumers can parse without making live remote-delivery claims.

Current bridge boundary:
- The row10f bridge reads row10e durable envelope proof state and serializes a typed protocol status event for portal/service consumers.
- The bridge keeps live broker delivery, family-hub delivery, provider or child-device delivery, policy authority, adapter execution, exact content, and host filtering unclaimed.
