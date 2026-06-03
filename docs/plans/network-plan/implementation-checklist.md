# Network Plan Implementation Checklist

This is the fill-in checklist for network evidence, classification, cascade,
AI audit, policy handoff, and proof-gated intervention work. Future AI workers
must update this file and the matching workpack checklist before reporting
`DONE` or PR-ready.

This checklist tracks network-plan execution only. It does not replace
`docs/product-capability-checklist.md`, and workers must not edit the product
checklist unless a feature row status, proof, or gap actually changes and the
worker holds the correct hub lock.

## Fill Rules

- Keep unchecked items unchecked until code, docs, tests, UI proof, and proof
  artifacts are present.
- Use `[~]` for partial contract/service/proof where the whole workpack is not
  complete.
- Record lane, branch, PR, commit, or proof path when an item moves.
- Leave intentionally deferred items unchecked and write the manual-required
  reason.
- Do not use this file to claim network/domain blocking readiness without real
  adapter proof.
- Do not start network event bus implementation before reusable Rust eventing
  proof exists.
- Fill the matching workpack checklist before reporting `DONE`.
- Report product-doc updates, or explicitly state why no product-doc update was
  needed.

## Required Proof Pack

Every implementation workpack needs a proof pack before the main workpack row
can be marked complete. Use this root unless the assignment names a stricter
location:

```text
output/network-plan-proof/<workpack-id>/
```

The proof pack must contain or explicitly mark N/A for each applicable item:

- [ ] `00-source-snapshot.md`: git branch, commit, `git status --short`,
      existing source paths inspected, existing behavior, and before-state gap.
- [ ] `01-contract-proof.log`: TypeScript/Rust contract tests, invalid states,
      schema-boundary/source-shape checks, and protocol parity where applicable.
- [ ] `02-eventing-proof.log`: reusable Rust eventing dependency, event
      constants, routing, queue/retry/TTL, no-adapter-call guards, and no UI
      business-event publish path when eventing changes.
- [ ] `03-parser-fixture-proof.json`: PCAP fixture input, parser output,
      expected JSON, TShark/Wireshark comparison, and must-not-claim records.
- [ ] `04-analyzer-alert-proof.json`: Zeek-style summaries,
      Suricata/Snort-compatible alert fixtures, false-positive guards, and
      no-signature-only-enforcement proof.
- [ ] `05-ai-policy-proof.json`: AI detection/audit input refs, output refs,
      policy decision refs, risk budget state, and unsupported-claim rejection.
- [ ] `06-adapter-action-proof.json`: adapter capability, dry-run/manual-required
      state, apply/result/rollback/unavailable state, and audit refs.
- [ ] `07-journal-sqlite-proof.json`: journal entry refs, replay result,
      SQLite/read-model rows, and custody labels.
- [ ] `08-ui-snapshots/`: parent portal screenshots for normal, degraded,
      manual-required, audit, risk budget, and limitation states when UI changes.
- [ ] `09-security-negative-proof.log`: no exact URL/video/message/search claim,
      no raw PCAP to AI, no decrypted payload, no weak-evidence block, no UI/AI
      direct enforcement.
- [ ] `10-performance-proof.log`: throughput, latency, CPU, memory, disk, queue,
      and dropped-event metrics when real-time claims are touched.
- [ ] `11-manual-platform-proof.md`: OS/device/permission details, exact manual
      steps, screenshots/logs, and manual-required labels for platform claims.
- [ ] `12-validation-commands.log`: focused validation plus any requested
      `npm run validate`/`ci:local`/manual command output.

## Evidence Quality Gates

- [ ] Network-only evidence never claims exact URL, exact video, private
      messages, search query, page content, screen activity, or decrypted
      payload.
- [ ] Every network claim has evidence grade A/B/C/D.
- [ ] Every network event that crosses Rust runtime uses reusable eventing
      contracts, not a private network bus.
- [ ] Analyzer alerts are evidence inputs, not policy authority.
- [ ] AI audit is advisory and cites evidence refs.
- [ ] Parent policy is the action authority.
- [ ] Adapter action requires policy decision refs and adapter proof.
- [ ] Dry-run and manual-required states cannot call adapters.
- [ ] Vite/TypeScript UI cannot own network business logic or publish adapter
      commands.
- [ ] Platform claims name exact OS/device/permission proof.
- [ ] Every failed, skipped, manual, or deferred test has a reason and follow-up
      owner recorded.

## Main Execution Gates

- [ ] Source docs read: folder README, source index, current snapshot, full
      scope plan, tests/proof blueprint, UI/UX guide, coverage audit,
      implementation checklist, eventing plan, and assigned workpack.
- [ ] Feature docs checked for overlap: child-agent local service,
      network/domain control, AI, policy, enforcement, browser, app/game,
      screen, LAN, reports/notifications.
- [ ] Hub lock covers the workpack file and exact implementation/docs paths.
- [ ] Existing source layout inspected before editing; no parallel network truth
      created.
- [ ] Reusable Rust eventing is implemented before network event routing.
- [ ] TypeScript Effect Schema contracts land before Rust/service/portal
      consumers where TypeScript domain boundaries are touched.
- [ ] Rust protocol parity exists for new protocol-facing contracts.
- [ ] Journal/read-model/storage behavior exists before portal or policy claims
      depend on it.
- [ ] Parent UI renders capability, degraded, stale, unsupported, unavailable,
      manual-required, limitation, audit, and risk-budget states honestly.
- [ ] Required proof pack exists with logs, JSON, screenshots, or explicit N/A
      reasons for every applicable gate.
- [ ] Feature docs, expectation docs, module READMEs, and product capability
      checklist decisions are recorded.
- [ ] `DONE` report includes workpack, touched paths, validation, proof, known
      gaps, screenshots, and documentation changes.

## Base Workpack Checklist

Use `[ ]` for not started, `[~]` for in progress, and `[x]` only after the
required proof pack exists. The `Evidence Or Proof` cell must name concrete
artifact paths, command logs, screenshots, PR checks, or an explicit
manual-required/N/A file.

| Step | Workpack | Status | Owner/Lane | Branch/PR/Commit | Evidence Or Proof | Doc/Checklist Decision |
| --- | --- | --- | --- | --- | --- | --- |
| 01 | Source index and repo reconciliation | [x] | primary | main docs pass | `docs/plans/network-plan/source-index.md` | Planning only; no runtime claim. |
| 02 | Current network snapshot and gap map | [x] | primary | main docs pass | `docs/plans/network-plan/current-network-snapshot.md` | Planning only; no runtime claim. |
| 03 | Contract boundary and Effect schemas | [ ] | - | - | - | Open. |
| 04 | Rust protocol parity for network contracts | [ ] | - | - | - | Open. |
| 05 | NetworkFlowEvidence contract | [ ] | - | - | - | Open. |
| 06 | NetworkDomainEvidence contract | [ ] | - | - | - | Open. |
| 07 | NetworkActivityClassification contract | [ ] | - | - | - | Open. |
| 08 | NetworkEvidenceGrade model | [ ] | - | - | - | Open. |
| 09 | NetworkPolicyAction and capability contract | [ ] | - | - | - | Open. |
| 10 | NetworkActivityEvent contracts and reusable Rust eventing consumption | [ ] | - | - | - | Blocked on eventing plan implementation. |
| 11 | Rust crate and tooling evaluation | [ ] | - | - | - | Open. |
| 12 | PCAP file replay harness | [ ] | - | - | - | Open. |
| 13 | Live pcap/Npcap/libpcap capture adapter | [ ] | - | - | - | Open. |
| 14 | Packet parser | [ ] | - | - | - | Open. |
| 15 | DNS query/response parser | [ ] | - | - | - | Open. |
| 16 | TLS ClientHello/SNI parser | [ ] | - | - | - | Open. |
| 17 | HTTP Host parser | [ ] | - | - | - | Open. |
| 18 | QUIC/HTTP3 limitation detector | [ ] | - | - | - | Open. |
| 19 | DoH/DoT detector | [ ] | - | - | - | Open. |
| 20 | Flow aggregation/sessionization | [ ] | - | - | - | Open. |
| 21 | Domain normalization and public suffix model | [ ] | - | - | - | Open. |
| 22 | Domain/category intelligence database | [ ] | - | - | - | Open. |
| 23 | Social/video/game/cloud-gaming classifier | [ ] | - | - | - | Open. |
| 24 | VPN/proxy/Tor/tunnel classifier | [ ] | - | - | - | Open. |
| 25 | Remote desktop/torrent/download classifier | [ ] | - | - | - | Open. |
| 26 | Process/app correlation model | [ ] | - | - | - | Open. |
| 27 | Managed browser correlation bridge | [ ] | - | - | - | Open. |
| 28 | Unmanaged browser correlation | [ ] | - | - | - | Open. |
| 29 | App/game foreground/session correlation | [ ] | - | - | - | Open. |
| 30 | Screen summary trigger integration | [ ] | - | - | - | Open. |
| 31 | Evidence cascade router | [ ] | - | - | - | Open. |
| 32 | Cross-slice evidence bundle builder | [ ] | - | - | - | Open. |
| 33 | Network-triggered local AI queue | [ ] | - | - | - | Open. |
| 34 | Evidence-grade policy mapping | [ ] | - | - | - | Open. |
| 35 | Parent notification candidate mapping | [ ] | - | - | - | Open. |
| 36 | Parent UI network evidence drawer | [ ] | - | - | - | Open. |
| 37 | DNS proxy/block/redirect adapter | [ ] | - | - | - | Open/manual-required until adapter proof. |
| 38 | Windows Firewall adapter | [ ] | - | - | - | Open/manual-required until adapter proof. |
| 39 | Windows WFP research/proof gate | [ ] | - | - | - | Open/manual-required until authority proof. |
| 40 | Android VpnService adapter/proof gate | [ ] | - | - | - | Open/manual-required until physical-device proof. |
| 41 | Apple Network Extension adapter/proof gate | [ ] | - | - | - | Open/manual-required until entitlement/device proof. |
| 42 | Linux nftables/eBPF/TUN adapter/proof gate | [ ] | - | - | - | Open/manual-required until distro proof. |
| 43 | Zeek-style structured log generator and analyzer comparison proof | [ ] | - | - | - | Open. |
| 44 | Suricata/Snort-compatible signature alert ingestion proof | [ ] | - | - | - | Open. |
| 45 | Event topic namespace, publisher SDK, subscriber filtering, backpressure, retention, and broker/family-hub decision proof | [ ] | - | - | - | Open; local-first reusable eventing first. |
| 46 | AI detection model fixture evaluation and drift/precision proof | [ ] | - | - | - | Open. |
| 47 | AI audit narrative and recommendation proof | [ ] | - | - | - | Open. |
| 48 | Household risk budget and cascade threshold model | [ ] | - | - | - | Open. |
| 49 | Performance, latency, resource, and high-concurrency benchmark proof | [ ] | - | - | - | Open. |
| 50 | Security, privacy, compliance, deployment, support, and staged rollout proof | [ ] | - | - | - | Open. |

## Worker Report Template

Use this shape in the hub report or PR-ready note:

```text
DONE network workpack <number/name>
Owner/lane:
Branch/commit/PR:
Touched paths:
Checklist updates:
Source snapshot:
Validation commands and logs:
Proof pack root:
Eventing proof:
Parser/analyzer proof:
AI/policy proof:
Adapter/action proof:
Journal/read-model proof:
UI snapshots:
Security negative proof:
Performance proof:
Manual/platform proof:
Feature docs updated:
Expectation docs updated:
Product capability checklist:
Known gaps/manual-required:
No-claim boundaries preserved:
```
