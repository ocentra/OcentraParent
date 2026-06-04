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

The first slice parses classic PCAP files with Ethernet/IPv4/UDP DNS query
metadata. It records source/destination IPs and ports, DNS question name/type,
and explicit false exact-content/decrypted-payload claim flags.

Live Npcap/libpcap capture, TCP/TLS/QUIC parsing, analyzer comparison, policy
handoff, and adapter execution remain separate proof-gated workpacks.
