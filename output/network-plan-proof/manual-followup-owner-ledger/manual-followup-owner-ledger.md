# Network Manual Follow-up Owner Ledger

## row10-broker-family-hub-delivery

- State: deferred
- Reason: Live broker and family-hub delivery require custody, auth, encryption, offset, replay, deletion, dedupe, broker config, family identity, and relay-policy artifacts before product delivery can be claimed.
- Follow-up owner: E-D network remote-delivery sequencing
- Follow-up path: row10b/row10f/row10g/row10h split branches after dependency integration
- Source refs: docs/features/network-domain-control.md, docs/plans/network-plan/implementation-checklist.md, docs/plans/network-plan/workpacks/README.md
- No-claim boundary: no live broker transport, no family-hub relay delivery, no remote adapter action

## raw-capture-retention-delete-export

- State: manual-required
- Reason: Raw PCAP/live-capture retention, remote deletion, and export propagation need live capture artifacts, encrypted storage evidence, quota rotation, custody chain, and private-traffic exclusion proof.
- Follow-up owner: E-D network custody and platform adapters
- Follow-up path: row13 live-capture custody plus remote delete/export propagation proof
- Source refs: docs/features/network-domain-control.md, docs/plans/network-plan/implementation-checklist.md, output/network-plan-proof/52-platform-claims-proof/11-manual-platform-proof.md
- No-claim boundary: no raw PCAP default storage, no production retention claim, no remote deletion propagation claim

## live-analyzer-model-policy-execution

- State: deferred
- Reason: Analyzer, local-AI model, and full policy engine execution remain fixture-backed or refs-only until live runtime execution and policy authority proof exists.
- Follow-up owner: E-D network AI/policy runtime
- Follow-up path: live analyzer/model/policy execution proof after local runtime integration
- Source refs: docs/features/network-domain-control.md, docs/plans/network-plan/implementation-checklist.md, output/network-plan-proof/51-end-to-end-pipeline-proof/proof-summary.json
- No-claim boundary: no live local model execution, no remote AI invocation, no policy engine execution claim

## host-adapter-execution

- State: manual-required
- Reason: DNS, Firewall, WFP, Android VpnService, Apple Network Extension, and Linux adapter rows require exact OS/device/permission/apply/rollback/audit artifacts before host mutation.
- Follow-up owner: E-D platform adapter proof owners
- Follow-up path: row37-row42 live adapter execution follow-up after platform proof artifacts
- Source refs: docs/expectations/enforcement.md, output/network-plan-proof/11-manual-platform-proof/11-manual-platform-proof.md, output/network-plan-proof/52-platform-claims-proof/proof-summary.json
- No-claim boundary: no host DNS mutation, no packet blocking, no enforcement command publication

## portal-risk-performance-platform-rendering

- State: deferred
- Reason: The current portal drawer renders service network read-model evidence, but broader risk-budget, performance, manual-required, degraded, and platform-state UI coverage is still unrendered.
- Follow-up owner: E-D network portal readiness
- Follow-up path: portal risk/performance/platform status rendering proof
- Source refs: docs/features/network-domain-control.md, docs/plans/network-plan/implementation-checklist.md
- No-claim boundary: no portal policy authority, no local evidence-grade computation in UI, no adapter command dispatch

## production-security-support-external-signoff

- State: manual-required
- Reason: Production-ready claims require external audit or penetration-test signoff plus full support, incident, staged-rollout, and training artifacts.
- Follow-up owner: Primary release/support coordination with E-D network evidence
- Follow-up path: external audit/support rollout proof after production scope is authorized
- Source refs: docs/features/network-domain-control.md, docs/plans/network-plan/implementation-checklist.md, output/network-plan-proof/11a-hardening-support-proof/proof-summary.json
- No-claim boundary: no production deployment claim, no external audit execution claim, no default remote upload

## ui-screenshot-na-for-non-ui-proof-rows

- State: skipped-non-ui
- Reason: Rust manifest and backend-only proof rows do not change a portal surface, so UI screenshots are explicitly not applicable and proof logs remain the evidence.
- Follow-up owner: Owning row implementer when UI changes
- Follow-up path: add portal screenshot proof only in branches that change UI rendering
- Source refs: docs/plans/network-plan/implementation-checklist.md, output/network-plan-proof/52-platform-claims-proof/11-manual-platform-proof.md
- No-claim boundary: no UI rendering claim without screenshot or e2e proof
