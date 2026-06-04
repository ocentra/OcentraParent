# Manual Platform Proof

WP33 uses the real local `sysinfo` process table, real encrypted `ActivityJournal`
append, and real `ActivityStore::ingest_journal` SQLite replay in the Rust test.

Limitations:

- No long-running process poller is claimed.
- No service event or WebSocket exposure is claimed.
- No foreground/window source is claimed.
- No platform authority tier is promoted.
