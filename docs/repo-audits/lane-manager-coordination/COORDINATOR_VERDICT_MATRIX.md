# Coordinator Verdict Matrix

This matrix summarizes the coordinator assessment of each canonical `*-selfaudit.md` input. It is a dispatch guide, not a completion claim.

| Thread | Verdict | First dispatch slice | Primary blocker | Dependency tier |
| --- | --- | --- | --- | --- |
| `codex-a-lane-manager` | coordinator-ready | archive hygiene + repo-audit WP01-WP05 | no final plan verdict matrix before this file | global |
| `logging-domain-parity` | partial/useful early | WP03 portal/dev-log consumer closeout | remaining WP proof roots and WP06 checker hardening | foundation |
| `cloudflare-control-plane-plan` | partial | CFCP-C queue/dead-letter + negative hardening | no canonical proof artifacts; payment handoff missing | foundation |
| `data-custody-storage-plan` | partial/blocking | substrate truth repair | proof roots/scripts drift; storage-custody-core test failure; parent-domain holdout | foundation |
| `account-identity-family-plan` | partial | WP02-WP05 proof reconciliation | WP06/WP07 open; sibling runtime/custody/trust blockers | foundation |
| `device-trust-bootstrap-plan` | partial | step-up/QR approval | key sealing waits custody substrate; later proof missing | foundation |
| `lan-plan` | partial | B1 proof regeneration | open implementation rows and physical/runtime proof | infrastructure |
| `eventing-plan` | partial | WP10 typed household-mesh runtime + crate tests | runtime still legacy shape; crate-level tests/proof missing | infrastructure |
| `policy-control-plane-plan` | partial | WP06 route/proof truth repair | WP03/WP04 proof bundles missing; WP02/WP05 dependency-open | infrastructure |
| `tracking-plan` | false-green/partial | WP33 closure precondition repair | schema/import crash; parent-domain wrappers; architecture debt | runtime |
| `network-plan` | partial/false-green-risk | parent-domain shim cleanup + proof root | proof roots missing; inline Rust tests; platform proof gaps | runtime |
| `browser-plan` | partial | WP01 foundation cleanup | closure audit red; proof roots missing; re-export debt | runtime |
| `app-game-plan` | partial | truth ownership cleanup | parent-domain façade; proof root missing; platform/product closure | runtime |
| `app-plan` | routing repair | app-plan truth repair | overlaps app-game; stale proof/source docs | runtime |
| `ai-plan` | partial/false-green-risk | AI ownership and architecture cleanup | parent-domain wrappers; placeholder categories; missing proof roots | runtime |
| `setup-install-provisioning-plan` | partial | WP06 truth-sync | stale aggregate proof; WP03 export mismatch; sibling blockers | overlay |
| `portal-ux-household-surfaces-plan` | partial/downstream | start-route + LAN consumer truth | route placeholders; LAN consumer red; proof roots missing | overlay |
| `screen-plan` | partial/false-green | truth/proof-contract repair | 100/100 mismatch; proof roots missing; inline Rust tests | overlay |
| `screen-ai-pipeline-plan` | blocked/partial | proof/test normalization + architecture cleanup | upstream screen/AI artifacts missing; architecture debt | overlay |
| `v0-8-enforcement-control-plan` | partial/false-green-risk | proof-router truth | corrupted proof index; app-game/browser/policy dependencies | overlay |
| `child-agent-runtime-distribution-plan` | partial | proof-root materializer + test category normalization | package/runtime proof and device proof missing | late closure |
| `parent-desktop-runtime-package-plan` | partial | proof-root + parent web distribution | package/update/setup handoff proof missing | late closure |
| `remote-access-plan` | missing/partial contract | contract parity and test repair after prerequisites | no remote protocol/service/portal/proof fabric | late closure |
| `payment-subscription-plan` | partial/blocked | payment worker/domain proof after Cloudflare handoff progresses | Cloudflare handoff proof and payment proof bundles missing | late closure |

## Manager action

Use `EXECUTION_DAG.md` for ordering and `thread-instructions/<thread>.md` for dispatch details.

## Critical path

```text
global structural gates
-> logging/cloudflare/data-custody/account/device-trust
-> LAN/eventing/policy
-> tracking/network/browser/app-game/app/AI
-> setup/portal/screen/screen-AI/enforcement
-> child package/parent package/remote/payment closure
```

## Main anti-patterns to block

- broad frontage package edits without path locks;
- scoped validation presented as repo-wide clean;
- proof path named without generator and run evidence;
- empty test category folders counted as coverage;
- inline Rust tests counted as final public-boundary coverage;
- downstream UI/product plans claiming upstream runtime truth.
