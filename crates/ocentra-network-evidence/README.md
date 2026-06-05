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
It also includes a deterministic evidence cascade router. Domain normalization
lowercases and validates metadata-derived domains, matches a deterministic
public suffix model, and derives registrable domains without upgrading to exact
URL or content claims. Flow aggregation merges reverse-direction packets into a
single five-tuple session, splits sessions by idle timeout, and records
packet/byte counters with explicit false exact-content/decrypted-payload claim
flags. Category lookup matches exact or registrable domains against
source-custody records, reports freshness/staleness, and rejects unsigned or
older category snapshot updates. The social/video/game classifier uses fresh
domain categories directly, keeps CDN/process hints confirmation-required, and
can promote a matching CDN/process hint only when separate browser confirmation
is supplied. The tunnel classifier labels VPN/proxy/Tor/tunnel indicators
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
rejected. The Windows WFP proof gate models lab-proof readiness only when
grade-A block policy, parent rule, evidence, target/provider/layer, lab-ready
capability, administrator permission, driver signing/package,
provider-registration, layer-capability, rollback, lab-result, and audit refs
are all present. Research-only, weak evidence, non-block policy actions,
manual-required, and unavailable states are non-executable, and exact URL,
decrypted payload, page content, live driver install, callout registration,
packet block, kernel payload inspection, command invocation, adapter action,
and enforcement-command claims are rejected.

Live Npcap/libpcap capture, full vendor category feeds, analyzer comparison,
production CDN intelligence, unmanaged browser URL correlation, foreground
session correlation, live network adapter enforcement, file/content inspection,
local-AI model execution/worker runtime, full policy engine execution,
notification provider delivery, live WFP driver/callout proof, live adapter
execution, and portal rendering remain separate proof-gated workpacks.
