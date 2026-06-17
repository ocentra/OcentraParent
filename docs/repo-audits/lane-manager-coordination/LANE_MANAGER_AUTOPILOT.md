# Lane Manager Autopilot Instructions

## Mission

Turn the per-thread self-assessments into controlled execution without letting lanes wander, duplicate work, or prove stale surfaces.

## Required first pass

Run these as coordination tasks before broad plan implementation:

| Step | Dispatch | Output |
| ---: | --- | --- |
| 0 | Lane manager | Remove or quarantine legacy numbered self-assessment duplicates so only canonical `*-selfaudit.md` files are used. |
| 1 | Repo-audit WP01 | Test topology inventory: real tests, empty scaffolds, inline tests, move candidates. |
| 2 | Repo-audit WP02 | CI/package/crate coverage matrix. |
| 3 | Repo-audit WP03 | Architecture policy decision: global cleanup, staged cleanup, or explicit exceptions. |
| 4 | Repo-audit WP04 | Ownership drift plus orphan/legacy/pre-eventing code map. |
| 5 | Repo-audit WP05 | DRY/common-core candidates, especially repeated child-domain event-chain assembly. |

## Dispatch protocol

For every thread assignment, send:

```text
Read docs/repo-audits/lane-manager-coordination/thread-instructions/<thread>.md.
Follow only the slice named there.
Do not widen scope.
Report exact files, commands, proof outputs, blockers, and next slice.
```

## Parallelization rule

Parallelize only when path ownership is disjoint.

Safe parallel examples:

- `logging-domain-parity` WP03/WP06 with `cloudflare-control-plane-plan` CFCP-C.
- `data-custody-storage-plan` substrate repair with `device-trust-bootstrap-plan` step-up/QR semantics, if neither edits shared files.
- `eventing-plan` WP10 typed bridge with `lan-plan` proof regeneration, if LAN source files are not edited by eventing.

Unsafe parallel examples:

- Multiple lanes editing `packages/parent-domain` shims.
- Multiple lanes editing `packages/agent-protocol-domain` shared contracts.
- Tracking/screen/screen-AI lanes editing the same screen/tracking/AI proof scripts.
- App/app-game/enforcement lanes editing app-game readiness/preflight proof surfaces simultaneously.

## Dependency dispatch order

1. Global structural work.
2. Foundation cluster: account, data custody, device trust, Cloudflare, logging.
3. Infrastructure cluster: LAN, eventing, policy core.
4. Runtime domain cluster: tracking, network, browser, app-game/app, AI.
5. Overlay/product cluster: setup, portal UX, screen, screen-AI, enforcement.
6. Distribution/remote/payment closure: child runtime package, parent runtime package, remote access, payment.

## Never accept these as closure

- `output/...` path named without generator command and local/CI run evidence.
- `test-results/...` path named without proof JSON/log and inspected result.
- `CI passed` without exact workflow job and command.
- `architecture passed` without exact scope.
- `tests exist` when only `.gitkeep` or inline source tests exist.
- `portal proved` when only route/component rendering was tested and runtime/service is missing.
- `manual-required` when a Windows/Android/Linux proof was feasible locally but skipped.

## Manager closeout format

Every dispatch result must be summarized in `thread-instructions/INDEX.md` or a future coordinator status file with:

| Thread | Assigned slice | Files touched | Commands run | Proof outputs | Verdict | Next dependency |
| --- | --- | --- | --- | --- | --- | --- |
