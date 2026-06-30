<!-- agent-capsule -->

> Agent Capsule
> Doc: Module Plan Map
> Kind: module-to-plan routing map.
> Read when: You need to know which plan owns or references an app, package, or crate.
> Stop rule: Pick the mapped plan, then open only that plan's `AGENTS.md` and workpack route.
> Proves: routing intent only.
> Does not prove: implementation status, feature completion, or validation.

<!-- /agent-capsule -->

# Module Plan Map

Use this map to connect module README work to plan ownership. If a module README is updated later, add a `Plan Route` section pointing to the relevant rows here.

Current architecture authority is Rust-first. TypeScript package rows below are
routing/debt ownership, not permission for those packages to own product
contracts or business logic. Canonical product DTOs, route snapshots, actions,
read models, and cross-boundary schemas route through `crates/schema`,
`crates/parent-runtime-core`, or the owning Rust domain/runtime crate.

That same rule applies to tests. TS-side tests, when they exist, are limited to
pure UI/presentation, generated-edge, and thin-adapter surfaces. Product-path
logic, runtime truth, contract drift, and proof closure belong in Rust-owned
test folders/crates.

## Apps

| Module | Primary plan route | Secondary route |
| --- | --- | --- |
| `apps/portal` | `portal-ux-household-surfaces-plan` | `policy-control-plane-plan`, `tracking-plan`, `app-game-plan`, `network-plan`, `screen-plan` |
| `apps/parent-desktop` | `parent-desktop-runtime-package-plan` | `remote-access-plan`, `production-distribution-support` via release/support docs |
| `apps/local-api` | dev-only or retirement routing under `child-agent-runtime-distribution-plan` | no product UI dependency; Rust service/bridge routes own product contracts |

## Transitional TypeScript Packages

| Module | Primary plan route | Secondary route |
| --- | --- | --- |
| `packages/schema-domain` | migration/edge-decoder debt under `setup-install-provisioning-plan` | all contract-bearing plans until Rust/generated replacements land |
| `packages/endpoint-domain` | `child-agent-runtime-distribution-plan` | `lan-plan`, `payment-subscription-plan`, `remote-access-plan` |
| `packages/agent-protocol-domain` | transitional adapter debt under `child-agent-runtime-distribution-plan` | all service-backed feature plans until Rust-owned consumers replace it |
| `packages/text-domain` | `portal-ux-household-surfaces-plan` | `policy-control-plane-plan` |
| `packages/portal-domain` | presentation-helper shrink path under `portal-ux-household-surfaces-plan` | all portal-rendered feature plans; no product contracts or snapshots |
| `packages/activity-domain` | `data-custody-storage-plan` | `browser-plan`, `app-game-plan`, `network-plan`, `screen-plan`, `tracking-plan` |
| `packages/evidence-domain` | `data-custody-storage-plan` | all evidence-producing feature plans |
| `packages/event-domain` | UI subscription/glue debt under `eventing-plan` | all event-producing feature plans; Rust owns business events |
| `packages/logging-domain` | dev/proof/UI-edge helper debt under `logging-domain-parity` | all proof-bearing plans; Rust owns product proof logs |
| `packages/parent-domain` | `policy-control-plane-plan` | `portal-ux-household-surfaces-plan`, `app-game-plan`, `billing/support plans` |
| `packages/family-domain` | `account-identity-family-plan` | `setup-install-provisioning-plan` |
| `packages/capability-domain` | `device-trust-bootstrap-plan` | `lan-plan`, `remote-access-plan`, platform/runtime plans |
| `packages/policy-domain` | `policy-control-plane-plan` | `v0-8-enforcement-control-plan` |
| `packages/enforcement-domain` | `v0-8-enforcement-control-plan` | `app-game-plan`, `browser-plan`, `network-plan` |
| `packages/ai-domain` | `ai-plan` | `screen-ai-pipeline-plan`, `policy-control-plane-plan` |
| `packages/app-game-domain` | `app-game-plan` | `app-plan`, `v0-8-enforcement-control-plan` |
| `packages/browser-domain` | `browser-plan` | `screen-ai-pipeline-plan`, `v0-8-enforcement-control-plan` |
| `packages/lan-domain` | Rust-owned transport migration under `lan-plan` | `remote-access-plan`, `ai-plan`; architecture docs do not take LAN runtime workpacks |
| `packages/network-domain` | `network-plan` | `remote-access-plan`, `v0-8-enforcement-control-plan` |
| `packages/screen-domain` | `screen-plan` | `screen-ai-pipeline-plan`, `remote-access-plan` |
| `packages/tracking-domain` | `tracking-plan` | `policy-control-plane-plan`, `portal-ux-household-surfaces-plan` |
| `packages/remote-access-domain` | `remote-access-plan` | `lan-plan`, `parent-desktop-runtime-package-plan` |
| `packages/child-runtime-domain` | `child-agent-runtime-distribution-plan` | `app-plan`, `device-trust-bootstrap-plan` |
| `packages/setup-domain` | `setup-install-provisioning-plan` | `account-identity-family-plan` |
| `packages/data-custody-domain` | `data-custody-storage-plan` | `remote-access-plan`, `cloudflare-control-plane-plan` |
| `packages/billing-domain` | `payment-subscription-plan` | `cloudflare-control-plane-plan`, `production-distribution-support` |
| `packages/notification-domain` | `portal-ux-household-surfaces-plan` | `payment-subscription-plan`, `v0-8-enforcement-control-plan` |
| `packages/production-domain` | `parent-desktop-runtime-package-plan` | `cloudflare-control-plane-plan`, `payment-subscription-plan` |

## Rust Crates

| Module | Primary plan route | Secondary route |
| --- | --- | --- |
| `crates/schema` | Rust-first parent architecture | all contract-bearing plans |
| `crates/agent-protocol` | `child-agent-runtime-distribution-plan` | all service-backed feature plans |
| `crates/agent-service` | `child-agent-runtime-distribution-plan` | all runtime feature plans |
| `crates/agent-core` | `child-agent-runtime-distribution-plan` | `app-game-plan`, `network-plan`, `screen-plan`, `tracking-plan` |
| `crates/agent-updater` | `parent-desktop-runtime-package-plan` | `setup-install-provisioning-plan` |
| `crates/app-core` | `app-plan` | `parent-desktop-runtime-package-plan` |
| `crates/app-game-core` | `app-game-plan` | `v0-8-enforcement-control-plan` |
| `crates/browser-core` | `browser-plan` | `v0-8-enforcement-control-plan` |
| `crates/network-core` | `network-plan` | `remote-access-plan`, `v0-8-enforcement-control-plan` |
| `crates/ocentra-network-evidence` | `network-plan` | `data-custody-storage-plan` |
| `crates/screen-core` | `screen-plan` | `screen-ai-pipeline-plan` |
| `crates/screen-ai-core` | `screen-ai-pipeline-plan` | `ai-plan`, `screen-plan` |
| `crates/screen-live-view-core` | `remote-access-plan` | `screen-plan` |
| `crates/screen-capture-adapter` | `screen-plan` | `screen-ai-pipeline-plan` |
| `crates/tracking-core` | `tracking-plan` | `policy-control-plane-plan` |
| `crates/lan-core` | `lan-plan` | `remote-access-plan`, `ai-plan` |
| `crates/remote-access-core` | `remote-access-plan` | `lan-plan` |
| `crates/child-runtime` | `child-agent-runtime-distribution-plan` | `app-plan`, `device-trust-bootstrap-plan` |
| `crates/child-ai-core` | `ai-plan` | `screen-ai-pipeline-plan` |
| `crates/child-policy-core` | `policy-control-plane-plan` | `v0-8-enforcement-control-plan` |
| `crates/child-enforcement-core` | `v0-8-enforcement-control-plan` | `app-game-plan`, `browser-plan`, `network-plan` |
| `crates/child-notification-core` | `portal-ux-household-surfaces-plan` | `v0-8-enforcement-control-plan` |
| `crates/policy-control-core` | `policy-control-plane-plan` | `v0-8-enforcement-control-plan` |
| `crates/parent-runtime-core` | `parent-desktop-runtime-package-plan` | `portal-ux-household-surfaces-plan` |
| `crates/family-identity-core` | `account-identity-family-plan` | `setup-install-provisioning-plan` |
| `crates/entitlement-core` | `payment-subscription-plan` | `device-trust-bootstrap-plan` |
| `crates/billing-core` | `payment-subscription-plan` | `cloudflare-control-plane-plan` |
| `crates/provisioning-core` | `setup-install-provisioning-plan` | `device-trust-bootstrap-plan` |
| `crates/storage-custody-core` | `data-custody-storage-plan` | `remote-access-plan` |
| `crates/logging-core` | `logging-domain-parity` | all proof-bearing plans |
| `crates/ocentra-eventing` | `eventing-plan` | all event-consuming runtime plans |
| `crates/ocentra-evidence` | `data-custody-storage-plan` | all evidence-producing runtime plans |

## README Plan Route Section

When a module README is safely updated later, add this section without deleting existing detail:

```markdown
## Plan Route

Primary plan: `docs/plans/<plan>/AGENTS.md`
Secondary routes: `<other relevant plans>`
```
