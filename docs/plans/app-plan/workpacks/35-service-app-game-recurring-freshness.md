# 35 Service App/Game Recurring Freshness

## Target State

The native app plan cross-records shared app/game WP35: agent-service startup
capture repeats bounded live native app/game runtime capture on a protocol-owned
cadence and keeps the existing journal/store/read-model path fresh without
claiming foreground, policy, portal, or adapter completion.

## Scope

- Reuse the shared app/game recurring capture cadence.
- Prove two service capture cycles append two live native app/game runtime rows.
- Prove foreground state remains not-claimed.
- Keep foreground source proof, portal freshness polish, policy consumption, and
  adapter execution as explicit gaps.

## Tests And Proof

- `cargo test -p ocentra-parent-agent-service activity_capture`
- `output/app-plan-proof/35-service-app-game-recurring-freshness`

## Done Signal

Native app runtime evidence now has recurring bounded service freshness through
the existing activity-capture path. Product status remains in progress until
foreground capture, portal source status, policy consumption, and platform
action proof are added.
