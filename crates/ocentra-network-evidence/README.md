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
replay-backed process/app correlation. Domain normalization lowercases and
validates metadata-derived domains, matches a deterministic public suffix model,
and derives registrable domains without upgrading to exact URL or content
claims. Flow aggregation merges reverse-direction packets into a single
five-tuple session, splits sessions by idle timeout, and records packet/byte
counters with explicit false exact-content/decrypted-payload claim flags.
Category lookup matches exact or registrable domains against source-custody
records, reports freshness/staleness, and rejects unsigned or older category
snapshot updates. The social/video/game classifier uses fresh domain categories
directly, keeps CDN/process hints confirmation-required, and can promote a
matching CDN/process hint only when separate browser confirmation is supplied.
The tunnel classifier labels VPN/proxy/Tor/tunnel indicators without claiming
hidden destinations, exact URLs, or decrypted content. The transfer classifier
labels remote desktop, torrent, and large-download candidates while leaving
unattributed high volume uncertain and keeping file names, exact URLs, and
content unavailable. The process/app correlation model links replayed flow PID
evidence to process snapshots and app inventory, while process-name-only traffic
stays candidate and adapter-unavailable or missing-process states stay explicit.

Live Npcap/libpcap capture, full vendor category feeds, analyzer comparison,
production CDN intelligence, managed/unmanaged browser URL correlation,
foreground session correlation, network adapter enforcement, file/content
inspection, policy handoff, and adapter execution remain separate proof-gated
workpacks.
