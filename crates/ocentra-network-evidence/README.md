# ocentra-network-evidence

Reusable network metadata parsing and replay helpers for Ocentra Parent proof
fixtures.

## Owns

- Deterministic PCAP fixture replay.
- Metadata-only packet, DNS, flow, and analyzer input helpers as they land.
- Evidence-grade and must-not-claim proof helpers for network-derived facts.

## Must Not Own

- Live packet capture drivers or OS permissions.
- Parent policy, enforcement, adapter calls, or portal rendering.
- Decrypted payload, exact URL, page content, message content, or search query
  claims from network-only evidence.

## Current Slice

The current slices parse classic PCAP files with Ethernet/IPv4 metadata, DNS
query/response metadata, TLS ClientHello SNI visibility, plain HTTP Host
visibility, QUIC limited-visibility candidates, DoH/DoT resolver candidates,
flow/session summaries, deterministic domain/category metadata,
social/video/game/cloud-gaming classifier candidates, VPN/proxy/Tor/tunnel
indicator classification, remote/torrent/download candidate classification, and
replay-backed process/app correlation plus a managed-browser correlation bridge.
It also includes a deterministic evidence cascade router and Zeek-style
connection, DNS, HTTP, TLS, and SSL analyzer-summary proof with approved
comparison artifacts. Suricata/Snort-compatible signature alert ingestion now
turns fixture rows into typed analyzer alert records with signature, rule-source,
severity, timestamp, flow, evidence, and custody refs. Domain normalization
lowercases and validates metadata-derived domains, matches a deterministic
public suffix model, and derives registrable domains without upgrading to exact
URL or content claims. Flow aggregation merges reverse-direction packets into a
single five-tuple session, splits sessions by idle timeout, and records
packet/byte counters with explicit false exact-content/decrypted-payload claim
flags. The live-capture proof gate models Windows Npcap, Linux libpcap, and
macOS BPF/libpcap readiness from driver, interface, permission, bounded-capture,
clean-stop, quota-rotation, retention/delete/export, custody, and
private-traffic-exclusion refs. Missing artifacts stay manual-required,
platform gaps stay unavailable/degraded, and the proof rejects live driver
invocation, unbounded capture, raw PCAP without custody, exact content,
policy/adapter authority, and enforcement-command claims.
The raw capture storage custody proof builds on that gate: raw artifact storage
is authorized only when the live-capture proof is proof-ready and raw artifact
manifest, local encrypted storage, encryption-at-rest, quota, retention,
delete/export, custody-chain, and private-traffic-exclusion refs are all
present. Missing refs stay manual-required, unavailable/degraded live-capture
states stay visible, and live capture execution, remote upload, raw PCAP without
custody, content, policy, adapter, and enforcement claims are rejected.
The bounded live-capture execution proof accepts driver-backed execution only
when a proof-ready row13 gate is paired with driver invocation, interface,
permission, bounded-window, clean-stop, custody, retention/delete/export,
metadata-only sanitization, and private-traffic-exclusion refs. Windows metadata
snapshots can be recorded as observations, but cannot substitute for
Npcap/libpcap packet capture. Raw artifact creation, raw PCAP without custody,
content visibility, policy authority, adapter authority, host filtering, and
enforcement commands stay rejected.
The Android physical target proof observes a named physical device through
read-only ADB identity probes and accepts readiness only when serial, product,
model, device, Android release, ABI, command refs, and evidence refs match.
Missing ADB, disconnected targets, missing observations, and mismatches stay
explicit, while emulator-only product support, live VpnService execution,
packet capture/blocking, app package correlation, adapter authority, production
Android support, content visibility, and enforcement commands remain rejected.
Unmanaged-browser
correlation records known or portable browser processes as process-only bypass
evidence, keeps browser-like process names candidate-only, preserves managed
browser boundary and adapter-unavailable states, and rejects exact URL,
active-tab, page-title, page-content, decrypted-payload, policy, adapter, and
enforcement-command claims. App/game foreground/session correlation consumes
stored app/game evidence, session-summary, foreground, process-correlation, and
launcher refs to confirm foreground or running sessions, keeps launcher-only
rows guarded, keeps candidates review-only, preserves missing/unavailable
states, and rejects exact URL, screen-content, AI-device-scanner, policy,
adapter, and enforcement-command claims. Screen-summary trigger planning queues
a screen-summary job only when the evidence cascade recommends screen
confirmation and parent settings, local encrypted queue, delete-after-analysis,
local runtime, debounce, and protected-surface guards allow it. Disabled,
queue-unavailable, custody-manual-required, protected-surface, debounced, and
not-recommended states remain explicit, and raw-image retention, remote upload,
screen content, policy, adapter, and enforcement-command claims are rejected.
Category lookup
matches exact or registrable domains against source-custody records, reports
freshness/staleness, and rejects unsigned or older category snapshot updates. The
social/video/game classifier uses fresh domain categories directly, keeps
CDN/process hints confirmation-required, and can promote a matching CDN/process
hint only when separate browser confirmation is supplied. The tunnel classifier
labels VPN/proxy/Tor/tunnel indicators
without claiming hidden destinations, exact URLs, or decrypted content. The
transfer classifier labels remote desktop, torrent, and large-download
candidates while leaving unattributed high volume uncertain and keeping file
names, exact URLs, and content unavailable. The process/app correlation model
links replayed flow PID evidence to process snapshots and app inventory, while
process-name-only traffic stays candidate and adapter-unavailable or
missing-process states stay explicit. The managed-browser bridge can attach
exact URL evidence only when a managed browser page record matches the
network-observed domain; network-only domain evidence remains domain-only. The
cascade router ranks available evidence, orders next checks for weak hints, and
never authorizes policy or adapter action. The cross-slice evidence bundle
builder assembles trigger refs plus domain/category, managed-browser,
process/app, screen, and local-AI suggestion refs into one downstream bundle
after cascade routing while rejecting decrypted payload, network-only exact URL,
policy-authority, or adapter-authority claims. The network-triggered local-AI
queue planner can turn a local-AI-recommended bundle into a queued job that
carries only trigger, evidence, summary, queue, and model-runtime refs. Disabled,
model-unavailable, queue-unavailable, and not-recommended states remain explicit
and do not carry jobs. The evidence-grade policy mapper turns A/B/C/D evidence
plus parent rule/policy refs into dry-run, parent-review, or observe-only
handoff states while keeping adapter and enforcement command authorization false.
The parent notification candidate mapper converts those handoff states into
candidate-only parent notification records with policy, rule, evidence, and
local-AI refs while rejecting provider delivery, sensitive payload, adapter, and
enforcement-command claims. The DNS adapter proof boundary models block and
redirect apply-readiness only when grade-A policy, parent rule, evidence,
supported capability, adapter authorization, apply/result, rollback, and audit
refs are all present. Dry-run, weak evidence, manual-required, and unavailable
states are non-executable, and exact URL, decrypted payload, page content, host
DNS mutation, and enforcement-command claims are rejected. The Windows Firewall
adapter proof boundary models block apply-readiness only when grade-A policy,
parent rule, evidence, target/rule, supported capability, adapter authorization,
apply/result, rollback, and audit refs are all present. Dry-run, weak evidence,
non-block policy actions, manual-required, and unavailable states are
non-executable, and exact URL, decrypted payload, page content, live firewall
mutation, netsh or PowerShell invocation, and enforcement-command claims are
rejected. The bounded Windows Firewall lab execution proof is separate from that
adapter boundary: it accepts executed-and-rolled-back only when an apply-ready
row38 adapter proof is paired with an Ocentra lab rule name, an RFC 5737
TEST-NET target, administrator permission, and apply/verify-present/rollback/
verify-removed command evidence. Missing host/admin/command evidence stays
manual-required or unavailable, and production enforcement, persistent firewall
rules, policy execution, enforcement commands, and exact-content claims remain
rejected. The Windows WFP proof gate models lab-proof readiness only when
grade-A block policy, parent rule, evidence, target/provider/layer, lab-ready
capability, administrator permission, driver signing/package,
provider-registration, layer-capability, rollback, lab-result, and audit refs
are all present. Research-only, weak evidence, non-block policy actions,
manual-required, and unavailable states are non-executable, and exact URL,
decrypted payload, page content, live driver install, callout registration,
packet block, kernel payload inspection, command invocation, adapter action,
and enforcement-command claims are rejected. The Android VpnService proof gate
models physical-device proof readiness only when grade-A block policy, parent
rule, evidence, package/service, physical-device-ready capability, VpnService
declaration, user consent, physical-device proof, package identity,
virtual-interface, traffic-observation, rollback, and audit refs are all
present. Device Owner proof is required only when Device Owner authority is
claimed. Research-only, weak evidence, non-block policy actions,
manual-required, and unavailable states are non-executable, and emulator-only
product support, exact URL, decrypted payload, page content, live VPN tunnel,
packet block, app/package correlation, adapter action, and enforcement-command
claims are rejected. The Apple Network Extension proof gate models
entitlement/device proof readiness only when grade-A block policy, parent rule,
evidence, bundle/extension, Apple-device-ready capability, developer team,
entitlement approval, provisioning profile, signing, device/TestFlight, Network
Extension declaration, extension configuration, rollback, and audit refs are all
present. Supervision/MDM proof is required only when supervision or managed
device authority is claimed. Research-only, weak evidence, non-block policy
actions, manual-required, and unavailable states are non-executable, and
simulator-only product support, exact URL, decrypted payload, page content, live
Network Extension behavior, packet block, app-level control, adapter action, and
enforcement-command claims are rejected. The Linux nftables/eBPF/TUN proof gate
models distro proof readiness only when grade-A block policy, parent rule,
evidence, selected adapter kind, distro/kernel, distro-kernel proof, permission,
adapter API capability, adapter plan, service-manager scope, rollback,
lab-result, and audit refs are all present. Research-only, weak evidence,
non-block policy actions, manual-required, and unavailable states are
non-executable, and generic Linux support, exact URL, decrypted payload, page
content, live adapter install, packet filtering, kernel hook load, TUN interface
mutation, service-manager install, adapter action, and enforcement-command
claims are rejected. The Zeek-style analyzer proof generates deterministic
connection, DNS, HTTP, TLS, and SSL rows from fixture-backed metadata, requires
approved comparison artifacts for each generated log kind, preserves unknown,
missing, ambiguous, and encrypted states without guessing, and rejects exact URL,
page content, decrypted payload, live analyzer invocation, signature-alert
ingestion, policy authority, adapter authority, and enforcement-command claims.
The signature alert ingestion proof records Suricata and Snort-compatible alerts
as analyzer evidence, marks high/critical non-false-positive signatures as review
candidates only, keeps known false positives non-enforcing, and rejects exact
URL, page content, decrypted payload, live IDS/IPS invocation, IPS prevention,
policy authority, adapter authority, and enforcement-command claims. The network
AI detection fixture proof evaluates labeled structured-summary, evidence-ref,
and analyzer-alert-ref fixtures against model predictions, records precision,
recall, accuracy, and average confidence drift, preserves unknown and mismatch
uncertainty codes, and rejects model-execution, remote-AI, raw PCAP, exact URL,
page-content, decrypted-payload, policy-authority, adapter-authority, and
enforcement-command claims. The network AI audit narrative proof consumes those
detection results and emits parent-readable advisory narratives with cited
detection, evidence, analyzer-alert, and parent-rule refs. High-risk true
positives recommend parent/policy review, uncertain detections recommend
managed-browser or screen-summary confirmation, benign cited detections stay
monitor-only, and remote-AI, raw PCAP, exact URL, page content, private message,
search query, decrypted payload, policy authority, adapter authority, and
enforcement-command claims are rejected. The household risk-budget threshold
proof consumes AI audit reports, child/profile refs, household policy refs,
prior-event refs, and adapter proof state to map risk pressure into ignore,
monitor, ask-parent, warn-child, limit, block, or manual-required
recommendations. Safe-behavior credit requires a parent rule cap, expiry, audit
reason, and UI explanation, while signature-only and missing-adapter cases stay
manual-required for control actions and no enforcement command is published.
The performance benchmark proof aggregates deterministic fixture rows for
packet-to-summary, packet-to-detection, detection-to-cascade latency, event
throughput, CPU, memory, disk, queue depth, dropped events, and high-concurrency
flow counts. It preserves dry-run/manual-required/unsupported/unavailable/
degraded path states and rejects real-time response, production SLO, raw PCAP,
exact URL, page content, decrypted payload, adapter action, host filtering, and
enforcement-command claims. The security readiness proof records gate refs for
threat model, privacy/compliance review, retention/delete/export custody, key
rotation, secret handling, rule/model provenance and rollback, support
materials, staff training, staged rollout, and known-gap signoff. Production
rollout remains blocked without an external audit or penetration-test signoff
ref, and default remote upload, raw PCAP without custody, exact content,
policy/adapter authority, and enforcement-command claims are rejected.
The end-to-end pipeline proof composes the existing deterministic trigger,
typed-event-ref, evidence bundle, local-AI refs-only queue, AI detection/audit,
risk-budget, policy mapping, adapter proof-state, audit, portal read-model, and
retention/delete/export refs into one product-path artifact while proving weak
or unavailable evidence cannot authorize adapter apply, AI remains advisory, and
UI/network surfaces cannot bypass policy. The platform-claim manifest proof
composes Windows Firewall/WFP, Android VpnService, Apple Network Extension, and
Linux nftables/eBPF/TUN gates into exact platform rows, including Apple
macOS/iOS and separate Linux nftables, eBPF, and TUN targets, with OS/device
refs, permission or entitlement refs, adapter capability refs, audit refs,
unavailable-state accounting, and manual follow-ups for missing required
artifacts while rejecting generic platform support, live adapter execution, UI
policy authority, exact URL, page content, decrypted payload, and
enforcement-command claims. The adapter capability status proof derives
target-specific supported, lab-ready, physical-device-ready, Apple-device-ready,
distro-ready, dry-run, research-only, manual-required, and unavailable status
rows from that platform manifest instead of creating a second platform truth
table, preserves manual follow-ups, verifies the current Activity drawer
capability/platform status source, and keeps live adapter execution, broader
platform capability UX, UI policy authority, and enforcement-command
publication unclaimed. The action-result state proof records blocked,
terminated, dry-run, manual-required, and unavailable result states from policy
refs plus adapter proof refs while rejecting live host mutation, process
termination execution, exact URL/content claims, and enforcement-command
publication.

Production live Npcap/libpcap driver support, raw artifact creation, full vendor
category feeds, production CDN intelligence, unmanaged browser exact URL or
active-tab correlation, unmanaged browser adapter action or process termination,
foreground session live adapter implementation, app/game policy evaluation,
process termination, time-limit execution, screen capture adapter execution,
OCR/VLM execution, screen analysis result creation, raw image retention, remote
upload, live network adapter enforcement, file/content inspection, local-AI
model execution/worker runtime, production model-quality monitoring, production
SLO validation, full policy engine execution, notification provider delivery,
portal AI audit UI, risk-budget, or performance rendering, live WFP
driver/callout proof, production Windows Firewall enforcement, live adapter
execution, Android physical-device behavior,
Apple entitlement/device behavior, Linux distro/kernel adapter behavior, live
Zeek/TShark/Wireshark/Suricata/Snort execution, full support-material authoring,
external audit execution, production deployment, and portal rendering remain
separate proof-gated workpacks.
