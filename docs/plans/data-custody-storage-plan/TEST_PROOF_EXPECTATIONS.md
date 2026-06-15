<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `Data Custody Storage Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Data Custody Storage Plan Test Proof Expectations

## General rule

Use focused commands first. Broader validation is allowed only after focused commands pass or a precise blocker is recorded.

If a required package/test path does not exist yet, write a blocker artifact and leave the checklist row open.

## Common command set

Use the subset relevant to the selected workpack:

```bash
cargo test -p ocentra-parent-storage-custody-core
cargo test -p ocentra-evidence
npm run build --workspace @ocentra-parent/production-domain
npm run test --workspace @ocentra-parent/production-domain -- custody
npm run test --workspace @ocentra-parent/portal -- storage
npm run lint:architecture -- --files crates/storage-custody-core crates/ocentra-evidence packages/production-domain apps/portal docs/plans/data-custody-storage-plan
```

Run through `npm run agent:run --` when collecting proof if available.

## WP01 Custody Source Of Truth

Expected coverage:

```text
data class matrix
owner/source-of-truth matrix
account control-plane separation
redaction boundary
no default hosted private activity store
```

## WP02 Encryption Key Custody

Expected coverage:

```text
key custody model
platform key wrapper matrix
wrong-key negative case
revoked device negative case
no universal Ocentra key
recovery/manual-required modes
```

## WP03 Parent Owned Cloud Sync

Expected coverage:

```text
provider capability matrix
encryption-before-upload proof
provider revoked state
quota/conflict/corruption states
offline retry and partial outage state
tombstone propagation
no automatic Ocentra fallback store
```

## WP04 Retention Delete Tombstone

Expected coverage:

```text
retention matrix
delete state machine
tombstone idempotency
offline retry behavior
report/export/assistant derived-data boundaries
wrong role denied
expiry boundary
restore cannot revive deleted state
```

## WP05 Export Import Backup Recovery

Expected coverage:

```text
export bundle contract
encrypted payload proof
import preview is non-mutating
wrong household/key/bundle state rejected
retention/tombstone preserved
restore/apply idempotent
partial restore state
support recovery limits
```

## WP06 Report Query Custody

Expected coverage:

```text
derived source matrix
deleted/expired data not returned
query cursor/pagination
query rate-limit/misuse boundary
notification payload allow/deny
portal cache custody
assistant citations restricted to allowed refs
stale/conflict state
```

## WP08 Parent Storage Settings Apply Flow

Expected coverage:

```text
storage choice state machine
export status
import preview
apply confirmation
provider disconnect/delete state
no automatic fallback store
portal cache status
visible manual-required state
```

## WP07 Rollout Proof And Route Gate

Expected aggregation:

```text
WP01 proof root exists or blocker recorded
WP02 proof root exists or blocker recorded
WP03 proof root exists or blocker recorded
WP04 proof root exists or blocker recorded
WP05 proof root exists or blocker recorded
WP06 proof root exists or blocker recorded
WP08 proof root exists or blocker recorded
route/index sync proof
privacy language review
manual-required gap register
```

Required negative states:

```text
storage claim without custody owner proof
sync claim without encryption/key proof
restore claim without preview/apply proof
delete claim without tombstone proof
report/query claim without source/citation proof
assistant/report output includes disallowed private payload
```
