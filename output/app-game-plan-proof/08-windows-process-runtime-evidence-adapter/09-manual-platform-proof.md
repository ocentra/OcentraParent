# Manual Platform Proof

Status: not applicable for WP08.

WP08 does not run a live Windows process poller, subscribe to start/exit events,
read executable metadata from disk, collect signatures/hashes from the OS, or
exercise a physical child device. It stages the contract/protocol/parser proof
that a later live adapter can call.

Manual platform proof remains required before claiming:

- live Windows process polling;
- process start/exit subscription;
- executable metadata collection;
- publisher signature or file hash collection from disk;
- journal/SQLite runtime replay;
- service-backed portal runtime rows;
- foreground evidence;
- enforcement.
