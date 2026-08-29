<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-plan`
> Doc: `NEXT_ACTIONS.md`
> Kind: short resume/action list.
> Read when: After `PLAN_STATE.md` when resuming this plan.
> Stop rule: Select only the first legal incomplete workpack from the graph.
> Proves: execution order only; not completion.

<!-- /agent-capsule -->

# Native Apps Plan Next Actions

The code-first audit is complete for all 95 workpacks. Use
[CODE_AUDIT.md](CODE_AUDIT.md) and the engineering graph; do not route from
legacy package/script paths embedded in old workpack prose.

## Phase 1 implementation order

1. **WP18 + WP49 — compiler/routing foundation**
   - Implement one Rust-owned native app policy compiler and category/risk/AI
     candidate routing path.
   - Require evidence freshness, device/local-user binding, schedule/authority
     refs, dry-run semantics, and fail-closed manual-required hard actions.
   - Add focused positive and stale/wrong-device/missing-proof/no-adapter tests.
2. **WP16 + WP17 — durable review/risk production**
   - Produce new/unknown/risk candidates from authoritative inventory/runtime
     evidence.
   - Add one-shot/persistent approval, expiry/replay, restart, and parent/child
     lifecycle tests.
3. **WP19 + WP20 — runtime and child UX**
   - Compose sessions, schedules, bonus/allow-once state, timer lifecycle, and
     child warning/request delivery with restart tests.
4. **WP62-WP65 — notification preference and delivery status**
   - WP59's scheduler bridge, WP60's metadata-only audit-history bridge, WP61's
     persisted provider-preflight bridge, and WP62's preference-preflight
     bridge plus real behavioral test source are reviewed. Implement the real
     WP64/WP65 status producers and preference owner next. Durable history/query,
     quiet-hours, retry/dead-letter, provider delivery, focused execution, and
     retained proof remain separate boundaries.
5. **WP15 + WP48 + WP63 — parent product surface**
   - Render inventory/running/foreground/session and source freshness states,
     evidence drill-in, empty/stale/degraded/manual states, and malicious/large
     metadata tests.
6. **WP26 — performance harnesses**
   - Add the specified inventory/process/replay/policy/portal scale tests.
7. **WP102 — shared-route validation**
   - Keep the retired parent-domain packet absent and validate the existing
     WP101-to-WP103 Rust route during the owning test phase.

## Execution rules

- Claim exact files through Enforcer before editing.
- One coherent workpack slice per branch; use E: only for any worktree.
- Update the workpack, `CODE_AUDIT.md`, `PLAN_STATE.md`, `WORKPACK_INDEX.md`,
  and graph mapping with every verified finding.
- Finish source and expected-test writing before broad validation.
- Phase 2 runs focused tests/Enforcer for the touched slice.
- Phase 3 regenerates proof from a clean checkout only after Phase 2 is green.
- Do not open the plan PR until the selected whole-plan audit/implementation
  batch is internally coherent under the user's promotion policy.
