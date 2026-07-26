# WP05 Negative-Case Proof

`app_game_timer_session_evidence_requires_matching_persisted_runtime_and_session`
proves both sides of the persisted boundary:

- matching session/runtime evidence binds and revalidates successfully;
- a changed process id is rejected as `Mismatch`.

The dispatch harness additionally proves that a command with no session
evidence is rejected as `app-game-session-evidence-required`, and one with an
unresolved runtime evidence id is rejected as `app-game-runtime-evidence-mismatch`.
An unknown runtime classification is also rejected, so weak identity cannot
silently become app/game execution authority.

The dispatch validator also requires known-app/known-game runtime state and a
session summary with matching process identity and freshness. It does not
promote a portal value or AI classification into execution authority.

Still open: a separate end-to-end parent-visible expiry workflow and mobile
adapter proof are not covered by this focused packet.
