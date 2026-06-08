# Network Manual Platform Proof

Branch: codex/network-policy-preview-stored-flow-evidence-on-row10k
Source commit: 987da398c1c98c68bcaeb69862d899657e24de91
Source status:  M crates/agent-core/src/lib.rs
 M crates/agent-core/src/network_event_runtime.rs
 M crates/agent-core/src/network_event_runtime/remote_delivery_transport_dispatch_state.rs
 M crates/agent-protocol/README.md
 M crates/agent-protocol/src/network_flow.rs
 M crates/agent-protocol/src/network_flow_tests.rs
 M crates/agent-service/README.md
 M crates/agent-service/src/network_remote_delivery_status_payload.rs
 M crates/agent-service/src/network_remote_delivery_status_service_tests.rs
 M docs/features/network-domain-control.md
 M docs/plans/network-plan/implementation-checklist.md
 M docs/plans/network-plan/workpacks/README.md
 M output/eventing-plan-proof/14-24-runtime-lifecycle/eventing-clippy.log
 M output/eventing-plan-proof/14-24-runtime-lifecycle/eventing-tests.log
 M output/eventing-plan-proof/14-24-runtime-lifecycle/proof-summary.json
 M output/eventing-plan-proof/18-24-handler-policy/eventing-clippy.log
 M output/eventing-plan-proof/18-24-handler-policy/eventing-tests.log
 M output/eventing-plan-proof/18-24-handler-policy/proof-summary.json
 M output/eventing-plan-proof/18-24-handler-policy/source-shape.log
 M output/eventing-plan-proof/20-24-metrics-testkit/eventing-handler-policy-tests.log
 M output/eventing-plan-proof/20-24-metrics-testkit/eventing-metrics-tests.log
 M output/eventing-plan-proof/20-24-metrics-testkit/proof-summary.json
 M output/eventing-plan-proof/20-24-metrics-testkit/source-shape.log
 M output/eventing-plan-proof/25-30-queue-policy/eventing-clippy.log
 M output/eventing-plan-proof/25-30-queue-policy/eventing-tests.log
 M output/eventing-plan-proof/25-30-queue-policy/proof-summary.json
 M output/eventing-plan-proof/25-30-queue-policy/source-shape.log
 M output/eventing-plan-proof/31-35-request-response/eventing-clippy.log
 M output/eventing-plan-proof/31-35-request-response/eventing-tests.log
 M output/eventing-plan-proof/31-35-request-response/proof-summary.json
 M output/eventing-plan-proof/31-35-request-response/source-shape.log
 M output/eventing-plan-proof/36-41-journal-replay/eventing-tests.log
 M output/eventing-plan-proof/36-41-journal-replay/proof-summary.json
 M output/eventing-plan-proof/36-41-journal-replay/source-shape.log
 M output/eventing-plan-proof/42-44-parent-child-protocol-contracts/proof-summary.json
 M output/eventing-plan-proof/45-50-network-protocol-contracts/proof-summary.json
 M output/eventing-plan-proof/51-54-parent-child-runtime/proof-summary.json
 M output/eventing-plan-proof/52-ui-typed-intent-boundary/proof-summary.json
 M output/eventing-plan-proof/55-56-enforcement-journal-action/proof-summary.json
 M output/eventing-plan-proof/57-network-workpack-10-reusable-crate/proof-summary.json
 M output/eventing-plan-proof/58-network-event-chain-exact-refs/proof-summary.json
 M output/eventing-plan-proof/59-weak-network-evidence-command-routing/proof-summary.json
 M output/eventing-plan-proof/60-61-command-boundary/proof-summary.json
 M output/eventing-plan-proof/62-network-proof-links/proof-summary.json
 M output/eventing-plan-proof/63-delivery-decision-proof/proof-summary.json
 M output/eventing-plan-proof/63-type-safety-source-gate/eventing-clippy.log
 M output/eventing-plan-proof/63-type-safety-source-gate/eventing-tests.log
 M output/eventing-plan-proof/63-type-safety-source-gate/proof-summary.json
 M output/eventing-plan-proof/63-type-safety-source-gate/source-shape.log
 M output/eventing-plan-proof/66-76-source-safety/eventing-clippy.log
 M output/eventing-plan-proof/66-76-source-safety/eventing-tests.log
 M output/eventing-plan-proof/66-76-source-safety/proof-summary.json
 M output/eventing-plan-proof/66-76-source-safety/source-shape.log
 M output/eventing-plan-proof/67-lock-await/eventing-clippy.log
 M output/eventing-plan-proof/67-lock-await/eventing-tests.log
 M output/eventing-plan-proof/67-lock-await/proof-summary.json
 M output/eventing-plan-proof/67-lock-await/source-shape.log
 M output/eventing-plan-proof/68-fixture-parity/proof-summary.json
 M output/eventing-plan-proof/68-fixture-parity/rust-fixture-parity-tests.log
 M output/eventing-plan-proof/68-fixture-parity/source-shape.log
 M output/eventing-plan-proof/69-compatibility-matrix/compatibility-matrix-example.log
 M output/eventing-plan-proof/69-compatibility-matrix/compatibility-matrix-tests.log
 M output/eventing-plan-proof/69-compatibility-matrix/eventing-clippy.log
 M output/eventing-plan-proof/69-compatibility-matrix/proof-summary.json
 M output/eventing-plan-proof/69-compatibility-matrix/source-shape.log
 M output/eventing-plan-proof/70-topology-manifest/eventing-clippy.log
 M output/eventing-plan-proof/70-topology-manifest/proof-summary.json
 M output/eventing-plan-proof/70-topology-manifest/source-shape.log
 M output/eventing-plan-proof/70-topology-manifest/topology-manifest-example.log
 M output/eventing-plan-proof/70-topology-manifest/topology-manifest-tests.log
 M output/eventing-plan-proof/71-manual-clock/eventing-clippy.log
 M output/eventing-plan-proof/71-manual-clock/eventing-tests.log
 M output/eventing-plan-proof/71-manual-clock/proof-summary.json
 M output/eventing-plan-proof/71-manual-clock/source-shape.log
 M output/eventing-plan-proof/72-contract-registry/contract-registry-docs-example.log
 M output/eventing-plan-proof/72-contract-registry/contract-registry-tests.log
 M output/eventing-plan-proof/72-contract-registry/eventing-clippy.log
 M output/eventing-plan-proof/72-contract-registry/proof-summary.json
 M output/eventing-plan-proof/72-contract-registry/source-shape.log
 M output/eventing-plan-proof/73-duplicate-subscriber/duplicate-subscriber-test.log
 M output/eventing-plan-proof/73-duplicate-subscriber/eventing-clippy.log
 M output/eventing-plan-proof/73-duplicate-subscriber/proof-summary.json
 M output/eventing-plan-proof/73-duplicate-subscriber/source-shape.log
 M output/eventing-plan-proof/74-lifecycle-clear/eventing-clippy.log
 M output/eventing-plan-proof/74-lifecycle-clear/lifecycle-clear-tests.log
 M output/eventing-plan-proof/74-lifecycle-clear/proof-summary.json
 M output/eventing-plan-proof/74-lifecycle-clear/source-shape.log
 M output/eventing-plan-proof/74-production-shutdown/eventing-clippy.log
 M output/eventing-plan-proof/74-production-shutdown/production-shutdown-tests.log
 M output/eventing-plan-proof/74-production-shutdown/proof-summary.json
 M output/eventing-plan-proof/74-production-shutdown/source-shape.log
 M output/eventing-plan-proof/75-family-variants/eventing-clippy.log
 M output/eventing-plan-proof/75-family-variants/family-variant-tests.log
 M output/eventing-plan-proof/75-family-variants/proof-summary.json
 M output/eventing-plan-proof/75-family-variants/source-shape.log
 M output/eventing-plan-proof/delivery-semantics/eventing-clippy.log
 M output/eventing-plan-proof/delivery-semantics/eventing-delivery-tests.log
 M output/eventing-plan-proof/delivery-semantics/proof-summary.json
 M output/eventing-plan-proof/delivery-semantics/source-shape.log
 M output/eventing-plan-proof/full-eventing-plan/command-logs/eventing-command-boundary-proof.log
 M output/eventing-plan-proof/full-eventing-plan/command-logs/eventing-enforcement-journal-action-proof.log
 M output/eventing-plan-proof/full-eventing-plan/command-logs/eventing-network-protocol-contract-proof.log
 M output/eventing-plan-proof/full-eventing-plan/command-logs/eventing-network-runtime-proof.log
 M output/eventing-plan-proof/full-eventing-plan/command-logs/eventing-parent-child-protocol-contract-proof.log
 M output/eventing-plan-proof/full-eventing-plan/command-logs/eventing-parent-child-runtime-proof.log
 M output/eventing-plan-proof/full-eventing-plan/command-logs/eventing-ui-typed-intent-boundary-proof.log
 M output/eventing-plan-proof/full-eventing-plan/command-logs/source-shape.log
 M output/eventing-plan-proof/reusable-eventing-runtime/00-source-snapshot.md
 M output/eventing-plan-proof/reusable-eventing-runtime/command-logs/ocentra-eventing-clippy.log
 M output/eventing-plan-proof/reusable-eventing-runtime/command-logs/ocentra-eventing-tests.log
 M output/eventing-plan-proof/reusable-eventing-runtime/command-logs/source-shape.log
 M output/eventing-plan-proof/reusable-eventing-runtime/proof-summary.json
 M output/network-plan-proof/03a-live-capture-storage-proof/clippy.log
 M output/network-plan-proof/03a-live-capture-storage-proof/proof-summary.json
 M output/network-plan-proof/03a-live-capture-storage-proof/raw-capture-storage-tests.log
 M output/network-plan-proof/03a-live-capture-storage-proof/source-shape.log
 M output/network-plan-proof/10h-remote-delivery-outbox-status-bridge/00-source-snapshot.md
 M output/network-plan-proof/10h-remote-delivery-outbox-status-bridge/agent-protocol-domain-remote-delivery-status-test.log
 M output/network-plan-proof/10h-remote-delivery-outbox-status-bridge/agent-protocol-remote-delivery-status-test.log
 M output/network-plan-proof/10h-remote-delivery-outbox-status-bridge/agent-service-remote-delivery-status-test.log
 M output/network-plan-proof/10h-remote-delivery-outbox-status-bridge/expected-remote-delivery-outbox-status-bridge.json
 M output/network-plan-proof/10h-remote-delivery-outbox-status-bridge/proof-summary.json
 M output/network-plan-proof/10h-remote-delivery-outbox-status-bridge/source-shape.log
 M output/network-plan-proof/10k-remote-delivery-transport-dispatch-state/12-validation-commands.log
 M output/network-plan-proof/10k-remote-delivery-transport-dispatch-state/expected-remote-delivery-transport-dispatch-state.json
 M output/network-plan-proof/10k-remote-delivery-transport-dispatch-state/proof-summary.json
 M output/network-plan-proof/10k-remote-delivery-transport-dispatch-state/source-shape.log
 M output/network-plan-proof/37-dns-proxy-block-redirect-adapter/clippy.log
 M output/network-plan-proof/37-dns-proxy-block-redirect-adapter/dns-adapter-tests.log
 M output/network-plan-proof/37-dns-proxy-block-redirect-adapter/proof-summary.json
 M output/network-plan-proof/37-dns-proxy-block-redirect-adapter/source-shape.log
 M output/network-plan-proof/38-windows-firewall-adapter/clippy.log
 M output/network-plan-proof/38-windows-firewall-adapter/proof-summary.json
 M output/network-plan-proof/38-windows-firewall-adapter/source-shape.log
 M output/network-plan-proof/38-windows-firewall-adapter/windows-firewall-adapter-tests.log
 M output/network-plan-proof/43-zeek-structured-log-analyzer-comparison/clippy.log
 M output/network-plan-proof/43-zeek-structured-log-analyzer-comparison/proof-summary.json
 M output/network-plan-proof/43-zeek-structured-log-analyzer-comparison/source-shape.log
 M output/network-plan-proof/43-zeek-structured-log-analyzer-comparison/zeek-generator-tests.log
 M output/network-plan-proof/44-signature-alert-ingestion-proof/clippy.log
 M output/network-plan-proof/44-signature-alert-ingestion-proof/proof-summary.json
 M output/network-plan-proof/44-signature-alert-ingestion-proof/signature-alert-ingestion-tests.log
 M output/network-plan-proof/44-signature-alert-ingestion-proof/source-shape.log
 M output/network-plan-proof/46-ai-detection-fixture-proof/ai-detection-fixture-tests.log
 M output/network-plan-proof/46-ai-detection-fixture-proof/clippy.log
 M output/network-plan-proof/46-ai-detection-fixture-proof/proof-summary.json
 M output/network-plan-proof/46-ai-detection-fixture-proof/source-shape.log
 M output/network-plan-proof/47-ai-audit-narrative-proof/ai-audit-narrative-tests.log
 M output/network-plan-proof/47-ai-audit-narrative-proof/clippy.log
 M output/network-plan-proof/47-ai-audit-narrative-proof/proof-summary.json
 M output/network-plan-proof/47-ai-audit-narrative-proof/source-shape.log
 M output/network-plan-proof/48-risk-budget-threshold-proof/proof-summary.json
 M output/network-plan-proof/48-risk-budget-threshold-proof/risk-budget-threshold-tests.log
 M output/network-plan-proof/48-risk-budget-threshold-proof/source-shape.log
 M output/network-plan-proof/policy-preview-stored-flow-evidence/proof-summary.json
 M output/network-plan-proof/policy-preview-stored-flow-evidence/source-shape.log
 M packages/agent-protocol-domain/README.md
 M packages/agent-protocol-domain/src/defaults.ts
 M packages/agent-protocol-domain/src/network-remote-delivery-status.ts
 M packages/agent-protocol-domain/tests/network-remote-delivery-status.test.ts
 M scripts/test/network-policy-preview-stored-flow-evidence-proof.mjs
 M scripts/test/network-remote-delivery-outbox-status-bridge-proof.mjs
 M scripts/test/network-remote-delivery-transport-dispatch-state-proof.mjs
 M test-results/eventing-network-runtime-proof/proof.json
 M test-results/eventing-runtime-proof/proof.json
 M test-results/network-ai-audit-narrative-proof/proof.json
 M test-results/network-ai-detection-fixture-proof/proof.json
 M test-results/network-dns-adapter-proof/proof.json
 M test-results/network-live-capture-storage-proof/proof.json
 M test-results/network-policy-preview-stored-flow-evidence-proof/proof.json
 M test-results/network-remote-delivery-outbox-status-bridge-proof/proof.json
 M test-results/network-remote-delivery-transport-dispatch-state-proof/proof.json
 M test-results/network-risk-budget-threshold-proof/proof.json
 M test-results/network-signature-alert-ingestion-proof/proof.json
 M test-results/network-windows-firewall-adapter-proof/proof.json
 M test-results/network-zeek-analyzer-comparison-proof/proof.json


This proof aggregates the existing platform-specific Rust proof gates into the required network-plan row 11 manual/platform proof pack.
It names the OS/device/permission evidence needed before platform claims can move beyond manual-required, unavailable, research-only, or proof-gated state.

## Windows - Npcap live capture observation
Proof rows: 13
Manual-required label: manual-required until host driver, permission, bounded capture, stop, quota, retention, custody, and private-traffic refs are supplied
Required permission: Administrator or equivalent Npcap capture permission on a named child device interface
Exact manual steps:
1. Identify child Windows host, Ocentra agent build, network interface, and Npcap installation version.
2. Attach driver, interface, permission, bounded-capture, clean-stop, quota, retention/delete/export, custody, and private-traffic-exclusion refs.
3. Run the live-capture proof gate and retain command log plus host/device proof evidence before claiming capture readiness.
Log evidence: command log captured by network-manual-platform-proof harness
Screenshot evidence: not applicable until a live UI/host/device proof claim is made

## Windows - DNS proxy/block/redirect and Windows Firewall adapter proof boundaries
Proof rows: 37, 38
Manual-required label: manual-required/unavailable unless supported capability, policy, apply/result, rollback, and audit refs are present
Required permission: Host DNS configuration or Windows Firewall administrative permission, depending on adapter kind
Exact manual steps:
1. Name child Windows host, adapter kind, parent rule ref, evidence ref, and policy decision ref.
2. Attach target/rule, supported capability, adapter authorization, apply/result, rollback, and audit refs.
3. Keep dry-run/manual/unavailable states non-executable until artifact refs are present and validated.
Log evidence: command log captured by network-manual-platform-proof harness
Screenshot evidence: not applicable until a live UI/host/device proof claim is made

## Windows - WFP signed/permissioned lab proof gate
Proof rows: 39
Manual-required label: manual-required unless signed driver/package, admin permission, provider registration, layer matrix, rollback, lab result, and audit refs are present
Required permission: Administrator permission plus signed Windows Filtering Platform driver/package proof
Exact manual steps:
1. Name child Windows host, WFP target/provider/layer refs, and signed package version.
2. Attach administrator permission, driver signing/package, provider-registration, layer-capability, rollback, lab-result, and audit refs.
3. Keep research-only/manual/unavailable states non-executable and do not claim packet blocking without the lab proof pack.
Log evidence: command log captured by network-manual-platform-proof harness
Screenshot evidence: not applicable until a live UI/host/device proof claim is made

## Android - VpnService physical-device proof gate
Proof rows: 40
Manual-required label: manual-required until physical device, VpnService declaration, consent, package identity, interface, traffic observation, rollback, and audit refs are present
Required permission: Android VpnService user consent, with Device Owner proof only when that authority is claimed
Exact manual steps:
1. Name physical child Android device, OS version, package identity, service declaration, and VpnService consent artifact.
2. Attach virtual-interface, traffic-observation, rollback, audit, and physical-device proof refs.
3. Attach Device Owner proof only if the product claim uses Device Owner authority.
Log evidence: command log captured by network-manual-platform-proof harness
Screenshot evidence: not applicable until a live UI/host/device proof claim is made

## Apple - Network Extension entitlement/device proof gate
Proof rows: 41
Manual-required label: manual-required until entitlement, provisioning, signing, device/TestFlight, extension configuration, rollback, and audit refs are present
Required permission: Approved Apple Network Extension entitlement and device/TestFlight proof; supervision/MDM proof only when claimed
Exact manual steps:
1. Name Apple device, OS version, developer team, entitlement approval, provisioning profile, signing, and bundle/extension refs.
2. Attach device/TestFlight, extension declaration/configuration, rollback, and audit refs.
3. Attach supervision or MDM proof only if the product claim relies on managed-device authority.
Log evidence: command log captured by network-manual-platform-proof harness
Screenshot evidence: not applicable until a live UI/host/device proof claim is made

## Linux - nftables/eBPF/TUN distro proof gate
Proof rows: 42
Manual-required label: manual-required until distro/kernel, permission, adapter API, service-manager, rollback, lab result, and audit refs are present
Required permission: Distro-specific privileged network adapter permission for nftables, eBPF, or TUN
Exact manual steps:
1. Name distro, kernel, service manager, selected adapter kind, and child host proof.
2. Attach permission, adapter API capability, adapter plan, service-manager scope, rollback, lab-result, and audit refs.
3. Keep generic Linux support unavailable until the selected distro/kernel proof pack exists.
Log evidence: command log captured by network-manual-platform-proof harness
Screenshot evidence: not applicable until a live UI/host/device proof claim is made

## Not Claimed
- live packet capture driver invocation
- host DNS mutation, proxy installation, Windows Firewall mutation, WFP driver install, or packet blocking
- Android VPN tunnel/filtering or Device Owner behavior without physical-device proof
- Apple Network Extension behavior, supervision, MDM, or app-level control without entitlement/device proof
- Linux adapter install, packet filtering, kernel hook load, TUN mutation, or service-manager install
- exact URL, page content, private message, search query, or decrypted payload availability
- policy authority, adapter action authority, or enforcement command publication

Screenshot policy: Screenshots are not attached because this slice is a non-UI contract/proof harness; live host or device proof must attach screenshots/logs before platform capability claims are upgraded.
