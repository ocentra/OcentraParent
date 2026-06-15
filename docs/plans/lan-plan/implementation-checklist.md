# LAN Plan Implementation Checklist

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan Implementation Checklist`
> Kind: full checklist; read exact rows only.
> Read when: Only for exact rows named by CHECKLIST_INDEX.md, workpack, or PR/DONE proof.
> Stop rule: Do not scan the whole checklist. Open exact row/section only.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This is the fill-in checklist for V0.9 LAN discovery, household inventory,
pairing, route custody, and LAN UI/UX work. Future AI workers must update this
file and the matching workpack checklist before reporting `DONE` or PR-ready.

This checklist tracks LAN-plan execution only. It does not replace
`docs/product-capability-checklist.md`, and workers must not edit that product
checklist unless a feature row status, proof, or gap actually changes and the
worker holds the correct hub lock.

## Fill Rules

- Keep unchecked items unchecked until code, docs, tests, UI proof, and proof
  artifacts are present.
- Use `[~]` for partial service-backed or contract-backed proof where the whole
  workpack is not complete.
- Record the lane, branch, PR, commit, or proof path in the notes column when an
  item moves.
- If an item is intentionally deferred, leave it unchecked and write the
  manual-required reason.
- Do not use this file to claim production household LAN readiness without real
  physical proof artifacts.
- Fill the matching `## AI Worker Checklist` inside the workpack file before
  reporting `DONE`.
- Report product-doc updates, or explicitly state why no product-doc update was
  needed.

## Required Proof Pack

Every implementation workpack needs a proof pack before the main workpack row
can be marked complete. Use this root unless the assignment names a stricter
location:

```text
output/lan-plan-proof/<workpack-id>/
```

The proof pack must contain or explicitly mark N/A for each applicable item:

- [ ] `00-source-snapshot.md`: git branch, commit, `git status --short`,
      existing source paths inspected, existing behavior, and the before-state
      gap.
- [ ] `01-contract-proof.log`: TypeScript contract tests, decode failures, and
      schema-boundary/source-shape checks for new or changed contracts.
- [ ] `02-rust-protocol-proof.log`: Rust protocol parity, serialization, and
      invalid-state tests when protocol/service shapes change.
- [ ] `03-runtime-service-evidence.json`: LAN adapter/read-model/service
      evidence, scan summary, source matrix rows, route state, registry state,
      and selected-device state.
- [ ] `04-registry-restart-proof.json`: durable household registry, parent
      decisions, route selection, stale/offline state, revocation, restart
      recovery, or explicit safe-unpaired state.
- [ ] `05-route-custody-security-proof.json`: pairing proof, origin route,
      wrong-device rejection, anonymous rejection, revoked route rejection, relay
      or cache custody label, and no Ocentra child-data custody claim.
- [ ] `06-ui-snapshots/`: Devices/LAN, Activity/Network, policy target, setup
      and degraded-state screenshots for every UI-visible state touched by the
      workpack.
- [ ] `07-playwright-ui-proof.log`: Playwright/browser proof for changed portal
      surfaces, including malicious/long text escaping and responsive state
      where applicable.
- [ ] `08-security-negative-proof.log`: negative tests proving weak sources do
      not confirm child identity, unsupported rows are visible-only, anonymous
      LAN control is rejected, wrong-origin/wrong-device proofs are rejected,
      and stale/revoked routes cannot receive commands.
- [ ] `09-manual-physical-lan-proof.md`: parent host, child host, router or
      firewall reachability, signed hello/heartbeat artifacts, service logs,
      screenshots, generated proof JSON, and manual-required labels for real
      household claims.
- [ ] `10-validation-commands.log`: focused validation plus any requested
      `npm run validate`/`ci:local`/manual command output.

## UI Snapshot Gates

When a workpack touches portal, policy target selection, route custody, source
matrix, setup flow, or diagnostic surfaces, workers must capture screenshots
before marking the workpack complete.

- [ ] Devices/LAN snapshot for the normal service-backed LAN view.
- [ ] Devices/LAN snapshot for stale, offline, ignored, revoked, unsupported, or
      manual-required state.
- [ ] Activity/Network snapshot for route, source, evidence, signed proof,
      parent decision, and audit diagnostics.
- [ ] Policy Network target snapshot when LAN device selection affects policy
      target binding.
- [ ] First-run setup or pairing snapshot when assignment, trust, rename, or
      direct-address entry is in scope.
- [ ] Responsive/narrow viewport snapshot when the touched UI is expected to be
      usable on small screens.
- [ ] Malicious/long text snapshot when hostnames, device names, vendor labels,
      IP strings, MAC strings, or proof labels are rendered.
- [ ] Explicit `ui-not-applicable.md` when the workpack has no UI surface.

## Evidence Quality Gates

- [ ] Raw fixture/evidence is stored with redacted sensitive values, not just a
      prose summary.
- [ ] Every visible device has source evidence, first seen, last seen,
      confidence/status, and no guessed owner or child profile.
- [ ] Weak LAN evidence never confirms a child-agent identity, assigns a child
      profile, or enables control.
- [ ] Signed child-agent hello/heartbeat cannot be marked implemented without
      signed artifacts and physical or equivalent manual proof.
- [ ] Routers, infrastructure, phones, printers, TVs, and unsupported devices
      remain visible but non-enrollable unless a supported child-agent route
      exists.
- [ ] Every stale, offline, ignored, revoked, unsupported, unavailable, and
      manual-required state is represented in contracts, service/read model, and
      UI where applicable.
- [ ] Every unsupported platform claim is represented as unsupported,
      manual-required, unavailable, or not-implemented until real platform proof
      exists.
- [ ] Every workpack and source row appears in the LAN source matrix with an
      implemented, partial, manual-required, or not-implemented status.
- [ ] Every failed, skipped, manual, or deferred test has a reason and follow-up
      owner recorded.

## Main Execution Gates

- [ ] Source docs read: folder README, source index, current snapshot, 20-step
      plan, test blueprint, UI/UX guide, coverage audit, implementation
      checklist, and the assigned workpack.
- [ ] Feature docs checked for overlap: family setup/device roles,
      remote/LAN/mobile platforms, network/domain control, policy schedules and
      approvals, activity/reporting, and platform support.
- [ ] Hub lock covers the workpack file and exact implementation/docs paths.
- [ ] Existing source layout inspected before editing; no parallel LAN truth
      created.
- [ ] TypeScript Effect Schema contracts land before Rust/service/portal
      consumers.
- [ ] Rust protocol parity exists for new protocol-facing contracts.
- [ ] Service/read-model/storage behavior exists before portal or policy claims
      depend on it.
- [ ] Portal UI renders capability, degraded, stale, unsupported, unavailable,
      revoked, and manual-required states honestly.
- [ ] Parent decisions are durable, auditable, and not overwritten by weak
      evidence.
- [ ] LAN scan discovers, child agent confirms, parent assigns.
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

| Step | Workpack                                                                                     | Status | Owner/Lane | Branch/PR/Commit                               | Evidence Or Proof                                                                                                                                                                                       | Doc/Checklist Decision                                                                                                                     |
| ---- | -------------------------------------------------------------------------------------------- | ------ | ---------- | ---------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| 01   | [Contract boundary and Effect schemas](workpacks/01-contract-boundary-and-effect-schemas.md) | [~]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | `packages/parent-domain/src/lan-discovery-source-matrix.ts`, `packages/agent-protocol-domain/src/lan-discovery-source-matrix.ts`, focused source-matrix proof JSON                                      | Partial: source matrix contracts exist; broader discovery/pairing contracts remain governed by later workpack proof.                       |
| 02   | [Evidence model and device record](workpacks/02-evidence-model-and-device-record.md)         | [~]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Service-backed add-device read model, source matrix rows, and Devices/LAN screenshots                                                                                                                   | Partial: evidence is visible; canonical all-source device evidence store is not fully complete.                                            |
| 03   | [Interface detection](workpacks/03-interface-detection.md)                                   | [~]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Source matrix marks selected-interface requirements and Windows neighbor-table path                                                                                                                     | Partial: interface requirements are represented; full safe adapter/interface selector proof remains open.                                  |
| 04   | [Neighbor table ingestion](workpacks/04-neighbor-table-ingestion.md)                         | [~]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Windows neighbor-table row implemented in Rust service source matrix; Linux/macOS rows remain manual-required                                                                                           | Partial: Windows proof exists; Linux/macOS normalization remains manual/platform-gated.                                                    |
| 05   | [Targeted ARP checks](workpacks/05-targeted-arp-checks.md)                                   | [ ]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Source matrix marks `targeted-arp-refresh` not implemented                                                                                                                                              | Open: no targeted ARP refresh implementation.                                                                                              |
| 06   | [Bounded ARP sweep](workpacks/06-bounded-arp-sweep.md)                                       | [ ]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Source matrix marks `bounded-arp-sweep` not implemented                                                                                                                                                 | Open: no bounded sweep implementation.                                                                                                     |
| 07   | [Passive discovery listeners](workpacks/07-passive-discovery-listeners.md)                   | [ ]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Source matrix marks passive ARP, mDNS, SSDP, LLMNR, and NetBIOS listeners not implemented                                                                                                               | Open/manual-required: packet listener adapters and packet fixtures are not present.                                                        |
| 08   | [mDNS and DNS-SD discovery](workpacks/08-mdns-dns-sd-discovery.md)                           | [ ]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Source matrix marks mDNS/DNS-SD query manual-required and child/parent mDNS advertisements not implemented                                                                                              | Open/manual-required: mDNS query and advertisement artifacts are missing.                                                                  |
| 09   | [SSDP and UPnP discovery](workpacks/09-ssdp-upnp-discovery.md)                               | [ ]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Source matrix marks SSDP/UPnP query manual-required and passive SSDP listener not implemented                                                                                                           | Open/manual-required: SSDP/UPnP adapter and fixtures are missing.                                                                          |
| 10   | [NetBIOS, LLMNR, and reverse DNS](workpacks/10-netbios-llmnr-reverse-dns.md)                 | [~]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Source matrix rows fence NetBIOS, LLMNR, and reverse DNS as weak name-only sources                                                                                                                      | Partial: name evidence is modeled honestly; real query adapters remain manual-required.                                                    |
| 11   | [Light service probing](workpacks/11-light-service-probing.md)                               | [ ]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Source matrix marks service identity probe manual-required                                                                                                                                              | Open: no bounded service probing adapter.                                                                                                  |
| 12   | [OUI and vendor lookup](workpacks/12-oui-vendor-lookup.md)                                   | [~]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Source matrix rows fence OUI/vendor lookup as weak/manual-required source                                                                                                                               | Partial: vendor signal is represented; real vendor DB/proof and randomized-MAC handling remain open.                                       |
| 13   | [Merge and de-duplication engine](workpacks/13-merge-deduplication-engine.md)                | [~]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Household device spine/read model merges local child agent and passive LAN evidence; weak source fence proof exists                                                                                     | Partial: route/read-model merge proof exists; full all-source dedupe engine proof remains open.                                            |
| 14   | [Explainable classification](workpacks/14-explainable-classification.md)                     | [~]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Source matrix and UI diagnostics expose evidence labels, authority, proof state, and non-claims                                                                                                         | Partial: classification/status explanation exists; complete device-type classifier proof remains open.                                     |
| 15   | [Household device store](workpacks/15-household-device-store.md)                             | [~]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Trusted registry, parent decisions, route custody, stale/offline selected-device rows, command-backed controls, and `output/lan-plan-proof/15-household-device-store/devices-identity-routing-proof.md` | Partial: service-backed rename/type persistence proof exists; full durable store/restart proof across all decisions remains open.          |
| 16   | [Read models and LAN events](workpacks/16-read-models-and-lan-events.md)                     | [~]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | `test-results/v0-9-lan-source-matrix-plan-completion/proof.json`, Devices/LAN and Activity/Network screenshots                                                                                          | Partial: source matrix/read model and diagnostics exist; full event stream/replay coverage remains open.                                   |
| 17   | [Parent and child mDNS advertisements](workpacks/17-parent-child-mdns-advertisements.md)     | [ ]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Source matrix marks parent and child mDNS advertisements not implemented                                                                                                                                | Open/manual-required: no advertisement implementation or packet proof.                                                                     |
| 18   | [Signed child hello and heartbeat](workpacks/18-signed-child-hello-heartbeat.md)             | [~]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Signed hello/heartbeat rows are strong-identity but manual-required; rejection rows and route-custody labels exist                                                                                      | Partial/manual-required: artifact-gated signed child proof exists as contract/read-model rows; real signed artifacts missing.              |
| 19   | [Assignment, revocation, and audit](workpacks/19-assignment-revocation-audit.md)             | [~]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Devices/LAN command controls for add, route select, rename, trust, ignore, restore, revoke; canonical household decisions route over local-network; Activity/Network audit diagnostics                  | Partial: command surfaces, identity persistence, and audit/read-model rows exist; full recovery/restart/negative route proof remains open. |
| 20   | [Proof gates, fixtures, and rollout](workpacks/20-proof-gates-fixtures-rollout.md)           | [~]    | codex-b    | `codex/v0-9-lan-source-matrix-plan-completion` | Focused proof script, proof JSON, Playwright browser proof, screenshot artifacts, and full `npm run validate` proof from the LAN source-matrix branch                                                   | Partial: current proof pack exists; physical second-device, packet adapter, performance, and manual rollout proof remain open.             |

## Current LAN Source Matrix Reconciliation - 2026-06-02

Checked items below are concrete proof in current `main` or the B-lane LAN
source-matrix branch. They do not mark a whole workpack complete unless every
requirement in that workpack is complete.

- [ ] The LAN read model carries all 20 plan workpacks as typed status rows.
- [ ] The source matrix carries implemented, partial, manual-required, and
      not-implemented rows instead of hiding gaps in prose.
- [ ] Weak LAN evidence sources are visible but cannot confirm child-agent
      identity or assign a child profile.
- [ ] Signed child-agent hello and heartbeat remain artifact-gated instead of
      being silently marked implemented.
- [ ] Devices/LAN and Activity/Network can render the matrix through the
      service-backed add-device read model.
- [ ] Devices/LAN household rename/type decisions for LAN-discovered neighbors
      route through the Rust LAN service and survive refresh without a
      portal-only second truth; proof is recorded in
      `output/lan-plan-proof/15-household-device-store/devices-identity-routing-proof.md`.
- [ ] Live B-lane browser screenshots exist for Devices/LAN,
      Activity/Network, and Network policy target binding under
      `output/playwright/lan-source-matrix-plan-completion/`.
- [ ] Focused proof JSON exists at
      `test-results/v0-9-lan-source-matrix-plan-completion/proof.json`.
- [ ] Packet-mode adapters remain manual-required or not implemented.
- [ ] Physical household proof still needs a second child-agent device.
- [ ] Real signed child hello/heartbeat artifacts are missing.
- [ ] Parent/child mDNS advertisements are not implemented.
- [ ] Optional relay/cache and parent-owned storage routes remain unavailable
      or manual-required until separately implemented and proved.
- [ ] Android/iOS child-agent parity, store signing, and mobile entitlement
      claims remain manual-required until separate platform proof exists.

## Physical/Manual Proof Checklist

These items must be filled before any report can claim production household LAN
readiness:

- [ ] Parent host identity, OS version, network interface, and firewall state.
- [ ] Second child-agent host identity, OS version, agent version, and route
      address.
- [ ] Router/firewall reachability proof between parent and child hosts.
- [ ] Signed child hello artifact with timestamp, device id, parent route id,
      signature state, and rejection proof for malformed/replayed/wrong-device
      variants.
- [ ] Signed child heartbeat artifact with freshness, stale/offline transition,
      and restart recovery proof.
- [ ] Router/infrastructure rows visible but not controllable.
- [ ] Unsupported LAN devices visible but not assignable as child-agent targets.
- [ ] Parent assignment, rename, trust, ignore, restore, route select, revoke,
      and recovery proof after service restart.
- [ ] Devices/LAN, Activity/Network, and Policy Network screenshots from the
      physical run.
- [ ] Generated proof JSON and validation command log stored under the workpack
      proof pack.

## Worker Report Template

Use this shape in the hub report or PR-ready note:

```text
DONE LAN workpack <number/name>
Owner/lane:
Branch/commit/PR:
Touched paths:
Checklist updates:
Source snapshot:
Validation commands and logs:
Proof pack root:
Raw evidence artifacts:
UI snapshots:
Security negative proof:
Manual/physical LAN proof:
Feature docs updated:
Expectation docs updated:
Product capability checklist:
Known gaps/manual-required:
No-claim boundaries preserved:
```
