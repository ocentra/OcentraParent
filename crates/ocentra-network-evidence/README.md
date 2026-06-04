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
visibility, QUIC limited-visibility candidates, DoH/DoT resolver candidates, and
flow/session summaries. Domain normalization lowercases and validates
metadata-derived domains, matches a deterministic public suffix model, and
derives registrable domains without upgrading to exact URL or content claims.
Flow aggregation merges reverse-direction packets into a single five-tuple
session, splits sessions by idle timeout, and records packet/byte counters with
explicit false exact-content/decrypted-payload claim flags.

Live Npcap/libpcap capture, analyzer comparison, policy handoff, and adapter
execution remain separate proof-gated workpacks.
