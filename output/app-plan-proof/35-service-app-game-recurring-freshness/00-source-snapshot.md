# Source Snapshot

- Branch: `codex/app-plan-evidence-control-continuation`
- Base head: `5cf8244ceac6a78b3efbf10f92f52a5578a13f30`
- Workpack: WP35 service app/game recurring freshness

Native app cross-record:

This proof mirrors the shared app/game WP35 service capture cadence. It applies
to native app runtime evidence because the app-use read model consumes the same
app/game ActivityStore projection.

Before-state gap:

Native app runtime rows could be captured through the service path once, but the
service did not keep the path fresh on a recurring cadence.
