# Authority Tier Proof

- Workpack: WP09 Windows foreground app/game evidence adapter
- Authority tier: observe-only parser proof
- Capability state: `available`, `permissionLimited`, and `adapterError` are
  represented in contracts/protocol/parser rows.
- Stronger controls: not claimed.

## Current Proof

- Foreground-window fixture records can produce foreground rows.
- Permission-limited rows keep missing window/title metadata explicit.
- Title refs are optional and permission-gated.
- `contentKnowledgeState` stays `notClaimed`.

## Required To Move Up

- Wire live foreground capture to the parser.
- Persist rows through journal/SQLite.
- Expose service read models/events.
- Prove portal labels and redaction.
- Add platform setup/permission proof if a stronger authority tier is claimed.
