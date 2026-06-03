# Authority Tier Proof

WP08 authority tier: `observe-only`.

The new runtime evidence contract and staged parser can represent:

- process observed;
- process started;
- process exited;
- unknown process;
- launcher runtime-only process;
- permission-limited metadata.

The slice does not claim:

- foreground authority;
- content knowledge;
- policy authority;
- adapter execution authority;
- broad installed-app blocking;
- launch prevention;
- process termination;
- rollback authority.

Proof needed to move beyond `observe-only`:

- live platform adapter proof;
- journal/SQLite replay;
- policy target compiler proof;
- dry-run policy decision proof;
- authority-tier and capability status proof;
- adapter setup, execution, rollback, and audit proof.
