# Manual Platform Proof

Status: not applicable for this WP09 parser slice.

This workpack does not claim live Windows foreground polling, user32 active
window access, permission prompts, service capture loops, AppLocker/App Control,
or enforcement. It proves only contract/protocol/parser behavior with fixture
records.

Authority tier: observe-only parser proof.

Proof required to upgrade:

- live Windows foreground-window capture wired to this parser;
- OS/version/device notes;
- permission/error-state screenshots or logs;
- journal and SQLite replay;
- service event/read-model proof;
- portal UI screenshots;
- rollback and cleanup notes for any stronger platform setup.
