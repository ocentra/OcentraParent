# Lane Manager Autopilot Instructions

## Mission

Turn the per-thread self-assessments into controlled execution without letting lanes wander, duplicate work, or prove stale surfaces.

## Required first pass

Run these as coordination tasks before broad plan implementation. These are inventory/coordination tasks first, not broad source-edit permission.

| Step | Dispatch | Output |
| ---: | --- | --- |
| 0 | Lane manager | Archive hygiene: only canonical `*-selfaudit.md` files are active inputs. |
| 1 | Repo-audit WP01 | Test topology inventory: real tests, empty scaffolds, inline tests, move candidates. |
| 2 | Repo-audit WP02 | CI/package/crate coverage matrix. |
| 3 | Repo-audit WP07 | Orphan/legacy/pre-eventing/stale proof-wrapper inventory. |
| 4 | Repo-audit WP03 | Architecture policy decision: global cleanup, staged cleanup, or explicit exceptions. |
| 5 | Repo-audit WP04 | Ownership drift map with path owners. |
| 6 | Repo-audit WP05 | DRY/common-core candidates and required pre-extraction tests. |

## Dispatch protocol

Every thread assignment must use `DISPATCH_PACKET_TEMPLATE.md`.

The packet must name one thread, one slice, exact allowed paths, exact forbidden paths, read scope, validation level, owner boundary, event/log chain, and stop conditions.

For every thread assignment, send:

```text
Read the dispatch packet.
Read docs/repo-audits/lane-manager-coordination/thread-instructions/<thread>.md.
Read docs/repo-audits/event-driven-proof-architecture/thread-instructions/<thread>.md.
Follow only the slice named there.
Use docs/repo-audits/lane-manager-coordination/READ_SCOPE_BUDGET.md for read scope.
Use docs/repo-audits/lane-manager-coordination/VALIDATION_BUDGET_LADDER.md for validation level.
Do not widen scope.
Report exact files read, files changed, validation level, proof outputs, blockers, and next slice.
```

## Parallelization rule

Parallelize only when path ownership is disjoint.

Safe parallel examples:

- `logging-domain-parity` WP03/WP06 with `cloudflare-control-plane-plan` CFCP-C.
- `data-custody-storage-plan` substrate repair with `account-identity-family-plan` WP02-WP05 proof reconciliation.
- `browser-plan` WP01 with `lan-plan` B1 if no shared files are touched.

Unsafe parallel examples:

- Multiple lanes editing `packages/parent-domain` shims.
- Multiple lanes editing `packages/agent-protocol-domain` shared contracts.
- Tracking/screen/screen-AI lanes editing the same screen/tracking/AI proof scripts.
- App/app-game/enforcement lanes editing app-game readiness/preflight proof surfaces simultaneously.

## Dependency dispatch order

Use `DISPATCH_PHASES.md` as the authoritative queue. Summary:

1. Global structural work.
2. Foundation truth/proof: tracking S0/S1, data custody, LAN B1, Cloudflare, account, logging, policy, setup.
3. Infrastructure and owner cleanup: device trust, eventing, network, browser, app-game/app, AI, payment-local, distribution-local.
4. Overlay/product slices: portal, screen, screen-AI, enforcement, remote.
5. Platform proof and final handoff.

## Never accept these as closure

- `output/...` path named without generator command and local/CI run evidence.
- `test-results/...` path named without proof JSON/log and inspected result.
- `CI passed` without exact workflow job and command.
- `architecture passed` without exact scope.
- `tests exist` when only `.gitkeep` or inline source tests exist.
- `portal proved` when only route/component rendering was tested and runtime/service is missing.
- `manual-required` when a Windows/Android/Linux proof was feasible locally but skipped.
- `logger-ready` without logger pattern or explicit exemption.
- `event-driven` while directly importing another owner's runtime behavior.

## Manager closeout format

Every dispatch result must be summarized in `thread-instructions/INDEX.md` or a future coordinator status file with:

| Thread | Assigned slice | Files read | Files touched | Validation level | Proof outputs | Verdict | Next dependency |
| --- | --- | --- | --- | --- | --- | --- | --- |
