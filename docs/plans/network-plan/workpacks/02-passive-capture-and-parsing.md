# WP02 Passive Capture And Parsing

Scope: prove packet/flow capture and parser behavior at the correct proof tier without overstating live or privileged capture.

Source rows: `03-network-implementation-checklist-and-workpacks.md` rows 11-20.

Read next:

- `../02-network-tests-proof-and-validation-blueprint.md`
- `../TEST_PROOF_EXPECTATIONS.md`
- Platform docs only when live capture or privileged adapter proof is assigned

Expected outcome:

- Capture/tooling evaluation identifies pcap, DNS parser, public suffix, tcpdump/dumpcap/TShark/Wireshark, Zeek, Suricata, and Snort-compatible options.
- PCAP replay harness covers safe, suspicious, edge, corrupted, encrypted, DoH/DoT, QUIC/HTTP3, CDN, and IP-only cases.
- Ethernet/IP/TCP/UDP/ICMP, DNS, visible TLS SNI, plaintext HTTP Host, DoH/DoT, QUIC limitation, and flow aggregation behavior are bounded.
- Live adapter claims are separated from replay proof.

Expected tests/proof:

- `network.pcap-replay.safe-suspicious-edge`
- `network.parser.dns-schema-fuzz`
- `network.parser.packet-corruption-negative`
- `network.parser.sni-host-visible-only`
- `network.detector.doh-dot-quic-limitations`
- `network.flow.sessionization-ordering`
- Proof includes fixture corpus, parser output snapshots, overclaim negative cases, and live-capture proof tier.

Failure conditions:

- P1 replay proof is not live capture proof.
- Visible SNI/Host metadata is not exact page, video, message, or search content.
- Do not retain raw family traffic without custody, encryption, quota, retention, delete, and export proof.
