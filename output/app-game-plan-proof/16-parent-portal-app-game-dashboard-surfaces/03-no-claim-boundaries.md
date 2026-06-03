# No-Claim Boundaries

This workpack does not add portal-side scanning, classification, timers, SQLite
reads, OS crawling, AI classification, policy execution, or enforcement calls.

No-claim checks covered by tests and implementation:

- Inventory rows stay inventory counts and are not labeled as usage.
- Running counts stay running counts and are not labeled as foreground.
- Foreground counts stay foreground counts and are not labeled as content.
- Launcher rows remain launcher rows and are not promoted into known game
  sessions without service-backed child-game proof.
- Unknown and possible states become review/candidate dashboard states, not
  known app/game facts.
- Manual-required and permission-required capability states stay visible.
- Game budget support is displayed as a policy-proof gap.
- Evidence counts come from read-model `evidence` arrays only.
