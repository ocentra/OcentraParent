<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `Data Custody Storage Plan Checklist Index`
> Kind: exact checklist router.
> Read when: a selected workpack references checklist rows.
> Stop rule: do not scan unrelated checklist rows.
> Proves: checklist routing only.
> Does not prove: implementation completion.
> Proof rule: a checkbox can be checked only after proof artifacts and focused command results exist.

<!-- /agent-capsule -->

# Data Custody Storage Plan Checklist Index

## Fill rules

- Leave a checkbox unchecked until proof exists.
- Every checked row must cite one or more proof artifacts from `PROOF_INDEX.md`.
- Every proof item must list exact commands run, pass/fail/blocker, and no-claim boundaries.
- Do not mark storage/sync/export/delete/report rows complete from docs-only work unless the selected workpack allows docs-only proof.
- Do not mark PR_READY until WP07 aggregates proof from all required earlier workpacks.

## WP01 Custody Source Of Truth

- [ ] Data classification matrix defined.
- [ ] Source-of-truth matrix defined.
- [ ] Account/control-plane separation documented.
- [ ] Redaction boundary documented.
- [ ] No default hosted private activity store proof exists.
- [ ] Current repo owners mapped.
- [ ] Adjacent owner handoffs recorded.
- [ ] Required proof artifacts written.
- [ ] Focused commands pass or blocker recorded.
- [ ] Workpack completion section filled.
- [ ] PLAN_STATE updated if state changed.
- [ ] No broad custody claim made.

## WP02 Encryption Key Custody

- [ ] Key custody model defined.
- [ ] Platform key wrapper matrix defined.
- [ ] Wrong-key negative proof exists.
- [ ] Revoked-device negative proof exists.
- [ ] No universal Ocentra key proof exists.
- [ ] Recovery/manual-required modes defined.
- [ ] Platform limitations recorded.
- [ ] Required proof artifacts written.
- [ ] Focused commands pass or blocker recorded.
- [ ] Workpack completion section filled.
- [ ] Adjacent device-trust handoff recorded.
- [ ] No key-readiness overclaim made.

## WP03 Parent Owned Cloud Sync

- [ ] Provider capability matrix defined.
- [ ] Encryption-before-upload proof exists.
- [ ] Provider revoked state exists.
- [ ] Quota/conflict/corruption states defined.
- [ ] Offline retry and partial outage states defined.
- [ ] Tombstone propagation defined.
- [ ] No automatic Ocentra fallback store proof exists.
- [ ] Required proof artifacts written.
- [ ] Focused commands pass or blocker recorded.
- [ ] Workpack completion section filled.
- [ ] Provider disconnect behavior recorded.
- [ ] No sync-ready overclaim made.
- [ ] Parent-visible state requirements recorded.

## WP04 Retention Delete Tombstone

- [ ] Retention matrix defined.
- [ ] Delete state machine defined.
- [ ] Tombstone idempotency proof exists.
- [ ] Offline retry behavior defined.
- [ ] Derived-output boundary proof exists.
- [ ] Wrong-role denial proof exists.
- [ ] Expiry boundary proof exists.
- [ ] Restore cannot revive deleted state proof exists.
- [ ] Required proof artifacts written.
- [ ] Focused commands pass or blocker recorded.
- [ ] Workpack completion section filled.
- [ ] No delete-ready overclaim made.
- [ ] Adjacent eventing/data owner handoffs recorded.

## WP05 Export Import Backup Recovery

- [ ] Export bundle contract defined.
- [ ] Encrypted payload proof exists.
- [ ] Import preview is non-mutating proof exists.
- [ ] Wrong household/key/bundle negative proof exists.
- [ ] Tombstone preserved proof exists.
- [ ] Restore/apply idempotency proof exists.
- [ ] Partial restore state defined.
- [ ] Support recovery limits recorded.
- [ ] Required proof artifacts written.
- [ ] Focused commands pass or blocker recorded.
- [ ] Workpack completion section filled.
- [ ] No restore-ready overclaim made.

## WP06 Report Query Custody

- [ ] Derived source matrix defined.
- [ ] Deleted/expired data not returned proof exists.
- [ ] Query cursor/pagination proof exists.
- [ ] Query rate-limit/misuse boundary defined.
- [ ] Notification payload allow/deny matrix defined.
- [ ] Portal cache custody proof exists.
- [ ] Assistant allowed-citation proof exists.
- [ ] Stale/conflict state proof exists.
- [ ] Required proof artifacts written.
- [ ] Focused commands pass or blocker recorded.
- [ ] Workpack completion section filled.
- [ ] No report/query custody overclaim made.
- [ ] Adjacent AI/notification owner handoffs recorded.

## WP08 Parent Storage Settings Apply Flow

- [ ] Parent storage choice state machine defined.
- [ ] Export status proof exists.
- [ ] Import preview proof exists.
- [ ] Apply confirmation proof exists.
- [ ] Provider disconnect proof exists.
- [ ] Provider delete proof exists.
- [ ] No automatic fallback proof exists.
- [ ] Portal cache status proof exists.
- [ ] Required proof artifacts written.
- [ ] Focused commands pass or blocker recorded.
- [ ] Workpack completion section filled.
- [ ] No settings-ready overclaim made.

## WP07 Rollout Proof And Route Gate

- [ ] WP01 proof root consumed or blocker recorded.
- [ ] WP02 proof root consumed or blocker recorded.
- [ ] WP03 proof root consumed or blocker recorded.
- [ ] WP04 proof root consumed or blocker recorded.
- [ ] WP05 proof root consumed or blocker recorded.
- [ ] WP06 proof root consumed or blocker recorded.
- [ ] WP08 proof root consumed or blocker recorded.
- [ ] Route/index sync proof written.
- [ ] Privacy language review proof written.
- [ ] Manual-required gap register written.
- [ ] Adjacent handoff proof written.
- [ ] Focused validation commands pass or blockers recorded.
- [ ] PLAN_STATE and WORKPACK_INDEX reflect actual state.
- [ ] No PR_READY claim without required proof roots.
