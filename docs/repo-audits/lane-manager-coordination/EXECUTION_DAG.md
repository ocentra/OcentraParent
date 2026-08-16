# Execution DAG

This is the lane-manager dependency order derived from the canonical per-thread self-assessments and structural audit baseline.

## Tier 0: global structural gates

Run before broad plan work:

```text
archive hygiene
-> test topology inventory
-> CI/package coverage matrix
-> architecture policy decision
-> ownership/orphan/legacy map
-> DRY/common-core candidate map
```

## Tier 1: foundation cluster

| Thread | First slice | Unblocks |
| --- | --- | --- |
| `logging-domain-parity` | WP03 portal/dev-log consumer closeout, then WP06 checker hardening. | Better proof/debug flow for all lanes. |
| `cloudflare-control-plane-plan` | CFCP-C queue/dead-letter + proof materialization. | `payment-subscription-plan`, account/cloud runtime handoffs. |
| `data-custody-storage-plan` | Substrate truth repair: proof roots, owner scripts, storage-custody-core test fix. | Device trust key/recovery, tracking custody, setup/export/sync. |
| `account-identity-family-plan` | WP02-WP05 proof reconciliation, WP07 real account/family UI proof prep. | Setup, policy, device trust, remote, payment/session gates. |
| `device-trust-bootstrap-plan` | Step-up/QR approval semantics next; defer key sealing until custody substrate exists. | Setup pair/trust, child package tamper, remote access, payment device subject. |

## Tier 2: infrastructure cluster

| Thread | First slice | Unblocks |
| --- | --- | --- |
| `lan-plan` | B1 proof regeneration, then B2 test truth repair. | Eventing WP10 authority wording, portal LAN consumer, remote route. |
| `eventing-plan` | WP10-A typed household-mesh bridge runtime + crate-level tests. | LAN/event bridge consumers, AI mesh, network/event consumers. |
| `policy-control-plane-plan` | WP06 route/proof truth repair, then WP03 and WP04 bundles. | Enforcement, app/game policy, portal authoring/preview, assistant actions. |

## Tier 3: runtime domain cluster

| Thread | First slice | Depends on |
| --- | --- | --- |
| `tracking-plan` | Fix closure precondition import/schema crash, migrate WP33 wrappers, clear architecture debt. | Data custody, eventing, notification/provider surfaces. |
| `network-plan` | Finish parent-domain network shim cleanup and create proof root. | Eventing/enforcement/browser/screen/AI/LAN for final rows. |
| `browser-plan` | Finish WP01 foundation cleanup, then WP03-WP05 inventory/platform proof. | Enforcement for final WP19/WP20. |
| `app-game-plan` | Truth ownership, proof-root normalization, source-policy-timer. | Enforcement app-game readiness/preflight. |
| `app-plan` | Truth repair only; delegate shared runtime to `app-game-plan` where appropriate. | App-game cleanup. |
| `ai-plan` | AI ownership cleanup and test-category rebase; local AI core proof. | Screen-AI and assistant surfaces. |

## Tier 4: overlay/product cluster

| Thread | First slice | Depends on |
| --- | --- | --- |
| `setup-install-provisioning-plan` | WP06 truth-sync, then WP03 export-surface repair. | Account, device trust, custody, parent/child distribution. |
| `portal-ux-household-surfaces-plan` | Start route + LAN consumer truth; then route ownership map. | Account/policy/LAN contracts for final closure. |
| `screen-plan` | Truth/proof-contract repair, then stale shim retargeting and Rust test relocation. | Tracking first; AI/screen-AI for derived claims. |
| `screen-ai-pipeline-plan` | Proof/test normalization and architecture cleanup. | Screen-plan and AI-plan artifacts. |
| `v0-8-enforcement-control-plan` | Proof-router truth, then Windows/browser/integrity boundaries and app-game service bridge. | App-game readiness, browser managed proof, policy dispatch. |

## Tier 5: distribution/remote/payment closure

| Thread | First slice | Depends on |
| --- | --- | --- |
| `child-agent-runtime-distribution-plan` | Proof-root materializer and test category normalization. | Setup/device trust for handoff; platform proof for final. |
| `parent-desktop-runtime-package-plan` | Proof-root + parent web distribution, then desktop package proof. | Setup handoff for final. |
| `remote-access-plan` | Contract parity and test repair only after account/device/LAN/screen foundations. | Account/session/device trust, LAN, screen live-view. |
| `payment-subscription-plan` | Payment worker/domain proof alpha after Cloudflare handoff progresses. | Cloudflare WP12, account/device subject for final entitlement semantics. |

## Hard sequencing notes

- Do not start `screen-ai-pipeline-plan` final proof before `screen-plan` and `ai-plan` produce current artifacts.
- Do not start `payment-subscription-plan` closure before Cloudflare publishes the handoff artifact.
- Do not start `remote-access-plan` beyond contract parity before account/device/LAN/screen dependencies are coherent.
- Do not let `v0-8-enforcement-control-plan` claim app/game readiness until app-game service/readiness/preflight proof is current.
- Do not let `portal-ux-household-surfaces-plan` claim runtime readiness; it consumes contracts and renders state.
