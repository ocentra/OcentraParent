# Project Progress Matrix

This is the code-backed execution dashboard for Ocentra Parent. It supplements
`PLAN_INDEX.md`; it does not replace plan-local workpacks, proof roots, or
checklists.

Last live-code audit: 2026-07-17, on `main` at `37146806c`.

## Code-backed refresh — 2026-07-31

This refresh is a scheduling override for the older rows below. It was produced
from plan state/workpack indexes plus current source and test surfaces. A
checked plan row is **not** reproducible closure when its named `output/*proof*`
root is absent from the checkout. Use the four states below when assigning work:

- **Source + focused tests**: implementation and relevant test source exist;
  the test was not re-run by this refresh.
- **Locally proven**: a current focused validation/proof artifact was observed.
- **Proof absent**: implementation may exist, but the plan's required proof
  output is unavailable here and must be recreated before closure.
- **Blocked**: one exact missing runtime contract, provider, platform, or
  external prerequisite prevents the next validation.

| Plan | 2026-07-31 code-backed state | Next owning workpack / dependency |
| --- | --- | --- |
| account identity/family | Source + focused tests; named proof roots absent. | WP01 provider/session consumer, then WP03/06 request-safety proof. |
| AI | Precursor source only; 47 of 48 workpacks remain open. | WP03 Rust contract, then WP07 queue and WP08 routing. |
| app | Source + focused tests; all app workpacks remain open/unknown. | WP06–09 Windows capture/foreground path, then WP12–15 service/journal. |
| app/game | Source + focused tests; customer control path unproven. | WP19 policy compiler, then WP20 budget evaluator. |
| browser | Source + focused tests; plan still has all primary workpacks open. | WP01 Rust-owner contract reconciliation before launcher/custody work. |
| child runtime distribution | Source present; package/service proof roots absent. | WP02 Windows service lifecycle, then recreate package/install proof. |
| Cloudflare control plane | Validation-open: the worker and generated billing contracts are present; the old missing-boundary claim is stale. | Run the current worker lint/unit/contract/boot gates, then reconcile the proof text and payment dependency from those results. |
| data custody/storage | Source + focused tests; rollout proof absent. | WP07 regenerate current proof manifest and verify runtime gaps. |
| device trust bootstrap | Source + focused tests; platform sealing/product chain absent. | WP02 local key sealing, then step-up/recovery/tamper workpacks. |
| eventing | Strong source/test foundation; historical proof roots absent. | WP10 LAN household mesh consumer and route proof. |
| LAN | Strong source/test foundation; physical product proof absent. | WP23 two-device proof, then WP25 backend-to-Tauri-to-portal chain. |
| logging parity | Source + focused tests; proof restoration and routing lint remain. | WP03 Rust service-to-logging mapping, then routing-lint repair. |
| network | Source + focused tests; all workpacks open. | WP01 foundation contracts/eventing. |
| parent runtime distribution | Source present; signing/install proof absent. | WP10 setup handoff contracts, then signing/store gates. |
| payment/subscription | Source + focused tests; Cloudflare dependency blocks runtime. | Cloudflare billing boundary, then WP05 tax/refund/dispute policy. |
| policy control plane | Strong focused contracts; adapter execution remains an external enforcement dependency, not a WP04 contract task. | WP02 authoring/preview and WP05 ask-parent/override work; track trusted-adapter execution with the enforcement owner. |
| portal UX | Real shell/UI; most product flows open. | WP05 policy authoring after owning policy read model is ready. |
| remote access | Scaffold/adjacent route only. | WP01 capability fabric, then WP04 pairing/revocation. |
| screen AI pipeline | Source pieces only; no pipeline proof root. | WP01 prerequisite gate then WP02 trigger-to-capture proof. |
| screen | Capture/AI/live-view source exists; custody/platform closure open. | WP03 contract, then WP06–08 capture model/adapter. |
| setup/install/provisioning | Source + focused tests; depends on identity/device/install truth. | Parent setup-to-child trust/install handoff. |
| tracking | Strong source/test foundation; event spine and provider proof open. | WP34 tracking event contracts, then WP35–39 chains. |
| v0.8 enforcement | Source + focused tests; adapters and parent-visible receipts open. | WP05 app/game session handoff after policy/runtime dependencies. |

### Priority order

1. Cloudflare billing-boundary boot repair (unblocks Cloudflare and payment).
2. Network WP01 eventing foundation.
3. Tracking WP34 event contracts.
4. Screen-AI WP01 → WP02 trigger/capture chain.
5. AI WP03 → WP07 → WP08; Remote WP01 → WP04.

Proof-root restoration is a verification/reproducibility packet. It must not be
used to claim a product path complete without the selected runtime proof.

## Live integration refresh — 2026-07-28

This is a merge-queue snapshot, separate from the 2026-07-17 whole-plan code
audit below. It records work that has current source, tests, proof, and a
pushed review branch; it is **not** a completion claim until the PR has green
CI, an acceptable review, is merged, and the named checklist row is reconciled.

| Plan / workpack | Current implementation evidence | Validation / integration state | Next action |
| --- | --- | --- | --- |
| Enforcement / WP05 app-game session handoff | Service validates persisted runtime/session evidence, rejects unknown identity, and asserts stored timer-binding fields. Protocol, core, and service focused tests are present. | PR [#584](https://github.com/ocentra/OcentraParent/pull/584), head `098b18acd`; all required jobs except the rerun service Clippy chain are green. The rerun is actively executing, not failed. | Let Clippy complete; then run service-test/aggregate checks, review, reconcile WP05 rows, and merge only if all are green. |
| App / WP01 runtime-decision contract boundary | Rust-owned app runtime contract change and focused branch are pushed. | PR [#577](https://github.com/ocentra/OcentraParent/pull/577) awaits CI/review. | Audit exact checklist/proof rows before treating it as closed. |
| Device trust / WP01 durable registry | Durable family/device trust registry branch is pushed. | PR [#576](https://github.com/ocentra/OcentraParent/pull/576) awaits CI/review. | Validate platform-sealing and parent-presence boundary before merge. |
| Network / WP01 foundation contract/eventing | Runtime contract/eventing repair is pushed. | PR [#573](https://github.com/ocentra/OcentraParent/pull/573) is behind `main`. | Rebase, rerun focused gates/CI, then review. |
| Cloudflare / WP12 handoff | Current generated billing-contract source, local seed/runtime proof, and focused worker tests are in the branch. | PR [#568](https://github.com/ocentra/OcentraParent/pull/568) is behind `main`. | Rebase, rerun worker/billing gates, reconcile stale plan wording. |
| Logging / WP04 parity, WP04/WP05/WP10 proof restoration | Three proof-restoration branches and one parity branch are pushed. | PRs [#572](https://github.com/ocentra/OcentraParent/pull/572), [#580](https://github.com/ocentra/OcentraParent/pull/580), [#581](https://github.com/ocentra/OcentraParent/pull/581), [#582](https://github.com/ocentra/OcentraParent/pull/582); #572 is behind `main`, the others await CI/review. | Regenerate each proof at its exact head; docs-only repair is not plan closure. |
| Portal / LAN result flow and E2E cleanup | Selected LAN-target persistence and E2E process cleanup are pushed. | PRs [#578](https://github.com/ocentra/OcentraParent/pull/578) and [#579](https://github.com/ocentra/OcentraParent/pull/579) await CI/review. | Run real portal/service click-through after CI; map accepted work to LAN/portal rows. |
| Shared tooling / schema build order | Test-build sequencing repair is pushed. | PR [#575](https://github.com/ocentra/OcentraParent/pull/575) is behind `main`. | Rebase and validate before wider test claims. |
| Local proof environment | Local proof isolation from LAN environment is pushed. | PR [#583](https://github.com/ocentra/OcentraParent/pull/583) awaits CI/review. | Treat as test infrastructure, not product-workpack closure. |

Only #584 is actively executing. The queue is parked for integration triage:
rebase-required first, then CI/review, then checklist and post-merge proof
reconciliation. No workpack here is done solely because a branch or PR exists.

## Status vocabulary

- **Foundation**: meaningful source and focused tests exist, but the product
  path is not yet proven end to end.
- **Integration**: multiple runtime layers exist; remaining work is joining,
  proving, or operating the path.
- **Blocked**: code or an external/runtime prerequisite prevents the next
  required proof.
- **Scaffold**: models/contracts or a thin implementation exist, but no
  credible product-runtime closure exists yet.

`Implemented` is not `done`. A workpack is only done after its selected tests,
proof artifact, checklist row, and merge state agree.

## Plan execution matrix

| Plan | Code state | Main runtime owners | Evidence observed | Current blocker / dependency | Next unblocker |
| --- | --- | --- | --- | --- | --- |
| `account-identity-family-plan` | Foundation | `family-identity-core`, `provisioning-core`, `entitlement-core`, `schema` | Family identity has 13 source / 7 test files; setup and signed-entitlement paths exist. | No production account adapter or D1/DO/KV migration proof. | Establish the Rust-owned account adapter and persistence migration boundary. |
| `ai-plan` | Foundation | `child-ai-core`, `screen-ai-core`, `agent-service`, `schema` | AI runtime and service seams exist. | Safety/output invariants and consumer proof remain fragmented. | Close one typed AI-result-to-policy handoff with safety and negative-path proof. |
| `app-plan` | Foundation | `app-core`, `agent-service`, `schema` | `app-core` has 3 source / 5 test files; service owns wider integration. | App-only authority and runtime evidence are incomplete. | Make app identity/evidence flow a single Rust-owned service path. |
| `app-game-plan` | Integration | `app-game-core`, `agent-service`, `schema` | 25 source / 20 test files; inventory, runtime, journal, and policy code exist. | Live platform metadata/crawling and portal product rows are incomplete. | Finish one live Windows app/game capture-to-read-model path. |
| `browser-plan` | Integration | `browser-core`, `agent-service`, `portal` | 43 source / 20 test files; managed-browser and intervention paths exist. | Managed/unmanaged execution and policy rollback are not closed. | Prove browser policy command through service, adapter, and visible portal state. |
| `child-agent-runtime-distribution-plan` | Integration | `child-runtime`, platform projects, release scripts | Child runtime, Android/iOS/Linux/macOS artifacts and proof surfaces exist. | Windows lifecycle/package proof is blocked; release proof is not whole-product readiness. | Resolve Windows service lifecycle and package smoke proof. |
| `cloudflare-control-plane-plan` | Integration / validation-open | `infra/cloudflare`, `billing-core`, account/billing contracts | Worker has 19 source / 28 test files, real route handlers, and generated billing contracts consumed by current source/tests. | Plan/workpack blocker text still names removed `packages/billing-domain` imports, while live source now uses `src/generated/billing-contracts`; current focused gates have not yet been refreshed. | Run the current lint, unit, contract, and worker-boot gates before changing ownership, then reconcile stale blocker/proof text with the result. |
| `data-custody-storage-plan` | Integration | `storage-custody-core`, `ocentra-evidence`, `ocentra-eventing` | Storage core has 63 source / 12 test files; custody/delete/export shapes exist. | Rollout/route-gate aggregation and cross-runtime custody proof remain open. | Prove one retention/delete/export flow through storage, eventing, and service. |
| `device-trust-bootstrap-plan` | Foundation / blocked | `schema`, `family-identity-core`, platform secure stores | Parent step-up validation, handoff schemas, trust helpers, and focused tests exist. | Concrete platform key-sealing adapters and the complete trusted-device product chain remain open. | Freeze the minimal parent-presence plus platform-sealed trust interface inside the owning core before shared-service integration. |
| `eventing-plan` | Integration | `ocentra-eventing`, `agent-protocol`, `agent-service` | 76 source / 34 test files; journal/replay and version-skew test surfaces exist. | Consumer product-path adoption and rollout proof remain. | Select one consumer path and prove replay/idempotency end to end. |
| `lan-plan` | Integration | `lan-core`, `agent-service`, `agent-core`, `schema` | 241 source / 91 test files; pairing, discovery, heartbeat, revocation, inventory, and read models exist. | Physical/consumer product proof and open follow-on workpacks remain. | Close a paired-device lifecycle through service and portal on a real platform. |
| `logging-domain-parity` | Foundation | `logging-core`, `logging-domain`, `agent-service`, portal | Logger, local evidence, MCP/query, and portal paths exist. | Broad adoption and several proof-root closeouts remain. | Make logging/proof correlation mandatory for one high-value product chain. |
| `network-plan` | Foundation | `network-core`, `ocentra-network-evidence`, `agent-service` | Network core has 49 source / 6 test files; service and evidence seams exist. | Broader parser, policy, and proof bundles are incomplete. | Close parser-to-policy evidence flow with malformed-input coverage. |
| `parent-client-runtime-distribution-plan` | Integration | Tauri parent desktop, Android/iOS parent projects, `parent-runtime-core` | Tauri shell and Android/iOS roots exist; focused package proof paths exist. | Whole release/signing/rollback readiness is unproven. | Produce one signed desktop package plus launch/rollback smoke. |
| `payment-subscription-plan` | Foundation / dependency-gated | `billing-core`, `entitlement-core`, Cloudflare worker | Billing core has 17 source / 4 test files; webhook and entitlement code exist. | Current Cloudflare/billing focused gates are not freshly green, and provider, account-authority, device-trust, and deployment proof remain open. | Refresh Cloudflare plus billing-core gates, then run one checkout/webhook-to-entitlement path without restoring obsolete TS contract ownership. |
| `policy-control-plane-plan` | Integration | `policy-control-core`, `agent-service`, `schema`, eventing | 126 source / 25 test files; compiler, preview, delivery, conflict, and authority code exist. | Policy-to-enforcement command/rollback product proof is incomplete. | Prove typed policy compile, delivery, execution receipt, and rollback. |
| `portal-ux-household-surfaces-plan` | Integration | `apps/portal`, `portal-domain`, HostBridge/service read models | Portal has 113 source / 87 test files and real route/panel code. | Several screens remain proof/presentation surfaces without completed backend actions. | Choose a service-backed household flow and prove the full click-through. |
| `remote-access-plan` | Scaffold | `remote-access-core`, `screen-live-view-core`, LAN, portal | Remote core has 2 source / 5 test files; adjacent live-view pieces exist. | Session grants, relay, revocation, and safety proof are not implemented as a product path. | Build view-only session grant/revoke state before any control feature. |
| `screen-ai-pipeline-plan` | Foundation | `screen-core`, `screen-ai-core`, capture adapter, `agent-service` | Capture/AI/service source and tests exist. | Trigger-to-capture-to-AI-to-policy operational proof remains open. | Close a redacted selected-window capture to typed AI-result proof. |
| `screen-plan` | Foundation | `screen-core`, `screen-capture-adapter`, `screen-live-view-core` | Capture adapters and platform paths exist; screen core has 3 source / 3 test files. | Cross-platform custody and live-view closure are incomplete. | Prove custody/delete behavior for one supported OS capture path. |
| `setup-install-provisioning-plan` | Integration | `provisioning-core`, setup/identity schemas, platform installers | Provisioning and setup readiness code exists. | Depends on identity, device trust, and child/parent installation truth. | Close a parent setup-to-child trust/install handoff after device trust exists. |
| `tracking-plan` | Integration | `tracking-core`, `agent-service`, `schema` | 70 source / 41 test files; location/geofence/device-status runtime exists. | Real device/provider/retention product proof remains incomplete. | Run a provider-to-read-model-to-portal tracking path with retention proof. |
| `v0-8-enforcement-control-plan` | Foundation | `child-enforcement-core`, `policy-control-core`, `agent-service`, schema | Contract and action-state surfaces exist. | Platform adapters, rollback, integrity, and parent-visible receipt proof remain open. | Make policy execution receipt + rollback a single adapter-backed vertical slice. |

## Workpack execution audit

This table is the current scheduling baseline derived from every routed
`WORKPACK_INDEX.md`, not a completion certificate. `Doc-claimed closed` means
the plan index currently marks the row checked/done. `Freshly reverified` stays
zero until the current branch regenerates the named proof and passes the
focused acceptance gate. Gitignored or absent historical `output/` and
`test-results/` paths cannot be used as retained current proof.

| Plan | Execution rows | Doc-claimed closed | Open / partial / blocked / unknown | Freshly reverified | Scheduling note |
| --- | ---: | ---: | ---: | ---: | --- |
| `account-identity-family-plan` | 7 | 6 | 1 | 0 | Checklist says 92/92, but WP01 is partial and cited output proof is absent locally. |
| `ai-plan` | 48 | 1 | 47 | 0 | Generic reset checklist does not reflect workpack state. |
| `app-game-plan` | 88 | 54 | 34 | 0 | The remaining 34 are only `possibly done`; audit before implementation. |
| `app-plan` | 95 | 0 | 95 | 0 | Reconciliation rows overlap app/game heavily; deduplicate before delegation. |
| `browser-plan` | 24 | 0 | 24 | 0 | Substantial runtime exists, but every execution row remains open. |
| `child-agent-runtime-distribution-plan` | 11 | 10 | 1 | 0 | Index claims ten complete while the generic checklist reports none. |
| `cloudflare-control-plane-plan` | 13 | 0 | 13 | 0 | All are blocked/proof-present; rerun against current generated-contract source before coding. |
| `data-custody-storage-plan` | 8 | 7 | 1 | 0 | Workpack index and checklist disagree in both directions on several rows. |
| `device-trust-bootstrap-plan` | 9 | 0 | 9 | 0 | Five partial, three blocked, one docs-only; adapter-backed runtime closure is missing. |
| `eventing-plan` | 4 | 3 | 1 | 0 | Nine historical rows are excluded and must not be rescheduled. |
| `lan-plan` | 25 | 13 | 12 | 0 | Remaining rows are mainly partial/manual physical proof, not twelve ordinary code packets. |
| `logging-domain-parity` | 10 | 0 | 10 | 0 | Five partial-proof, four source-present, one audit-open. |
| `network-plan` | 8 | 0 | 8 | 0 | Index correctly keeps the execution set open. |
| `parent-client-runtime-distribution-plan` | 11 | 7 | 4 | 0 | Routed through `parent-desktop-runtime-package-plan`; state/index and checklist disagree on WP03/WP04. |
| `payment-subscription-plan` | 13 | 3 | 10 | 0 | Engineering specification is not runtime closure; Cloudflare/trust dependencies remain. |
| `policy-control-plane-plan` | 8 | 6 | 2 | 0 | Six checked workpacks are not reflected by the generic checklist status. |
| `portal-ux-household-surfaces-plan` | 20 | 5 | 15 | 0 | Checklist is stale relative to the workpack index. |
| `remote-access-plan` | 6 | 0 | 6 | 0 | Five planned rows and one deferred control row. |
| `screen-ai-pipeline-plan` | 10 | 0 | 10 | 0 | Proof manifest/root is absent; rows correctly remain open. |
| `screen-plan` | 40 | 18 | 22 | 0 | Eighteen checked workpacks are not reflected by the generic checklist. |
| `setup-install-provisioning-plan` | 7 | 6 | 1 | 0 | WP06 is done as a blocker/aggregation packet but remains open for whole-plan scheduling; 93/93 checklist is not product completion. |
| `tracking-plan` | 39 | 0 | 39 | 0 | Internally checked rows were intentionally reopened for audit/proof reruns. |
| `v0-8-enforcement-control-plan` | 20 | 6 | 14 | 0 | Six checked workpacks are not reflected by the generic checklist. |
| **Total** | **524** | **144** | **380** | **0** | Plus 145 reference/source-only rows and 9 historical rows excluded from execution scheduling. |

### Acceptance state for each workpack

A workpack advances independently through `implemented -> focused gate green ->
retained proof regenerated -> checklist reconciled -> accepted commit -> merged ->
post-merge reverified`. Only the final state counts toward `Freshly reverified`.

## Crate to plan matrix

| Runtime owner | Plans unblocked or directly served | Priority reason |
| --- | --- | --- |
| `schema` + `agent-protocol` | every plan | Stable shared contracts prevent duplicate ownership and unblock service integration. |
| `ocentra-eventing` | custody, policy, tracking, LAN, enforcement, portal | Replay/idempotency/journal semantics are the common durability spine. |
| `family-identity-core` + device trust | account, setup, payment, remote, enforcement | Parent authority and device presence are required before sensitive actions. |
| `policy-control-core` | app/game, browser, LAN, network, screen, enforcement, portal | Policy is the shared decision producer for product controls. |
| `child-enforcement-core` + adapters | policy, browser, app/game, network, screen, enforcement | Converts approved policy into a reversible platform action. |
| `agent-service` | nearly every product plan | It joins feature crates into commands, read models, proof logs, and HostBridge state. |
| `storage-custody-core` | custody, tracking, screen, AI, account, payment | Retention/delete/export safety is a cross-cutting closure requirement. |
| `lan-core` | LAN, setup, remote, portal, child runtime | Device pairing and household transport unblock multi-device product proof. |
| `infra/cloudflare` + generated billing contracts | payment, account, setup, portal | Current source exists; focused worker/billing gates and stale proof reconciliation decide the next cloud packet. |
| `apps/portal` | portal, setup, policy, tracking, LAN, remote, reports | Presentation closes only after service read models and typed actions are available. |

## Dependency and unblock order

1. **Refresh Cloudflare and billing proof**: run the current generated-contract lint, unit, contract, worker-boot, and billing-core gates; then replace stale missing-`billing-domain` blocker text with the observed result. Do not restore removed TypeScript contract ownership.
2. **Build device trust runtime**: parent presence and sealed device trust unblock safe account, setup, payment, remote, and enforcement decisions.
3. **Close policy to enforcement vertical slice**: compiler -> service -> adapter -> receipt -> rollback. This becomes the reusable control path for browser, app/game, network, and screen.
4. **Use LAN/service as the first physical household proof**: pairing -> device state -> portal read model establishes the multi-device integration baseline.
5. **Close custody and observability on that vertical slice**: correlated logs, retention/delete, and replay make later feature work trustworthy.
6. **Scale feature producers**: browser, app/game, network, screen, tracking, and AI can then feed the same decision and evidence spine.
7. **Finish portal and distribution proof**: run real click-through and package/smoke proof only after the underlying product paths exist.

## Checklist synchronization rule

Every plan checklist should retain unchecked rows unless the named workpack has
all of the following: implementation, focused tests, retained proof artifact,
and an accepted merge state. A checklist audit may record current code evidence
and a blocker, but must never turn a code inventory into a completion claim.
