<!-- agent-capsule -->

> Agent Capsule
> Doc: Plan Worker Flow
> Kind: agent flow documentation; read only when selected by root AGENTS or TASK_ROUTER.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Plan Worker Flow

Use this for product, feature, roadmap, policy, UI, AI, platform, enforcement,
remote, reporting, and docs work.

## Authoring rule

Plan docs must describe target outcomes, ownership, data/protocol/schema shapes,
test/proof expectations, and failure conditions. Do not turn plan docs into code
recipes. A short snippet is allowed only when it defines a public contract or
artifact shape more precisely than prose.

If a plan/workpack is not execution-ready, apply [PLAN_EXECUTION_STANDARD.md](PLAN_EXECUTION_STANDARD.md) before implementation. Missing proof roots, missing commands, missing negative cases, or missing no-claim boundaries are blockers, not things for an agent to infer.

## Minimum read path

1. `docs/PLAN_INDEX.md`.
2. Selected plan's `AGENTS.md`.
3. Selected plan's `PLAN_STATE.md`.
4. Selected plan's `NEXT_ACTIONS.md` when starting/resuming.
5. Selected plan's `WORKPACK_INDEX.md`.
6. Assigned workpack only.
7. `CHECKLIST_INDEX.md` only for the rows/section referenced by the workpack.
8. Feature/expectation docs named by the plan or workpack only.
9. Selected plan's `TEST_PROOF_EXPECTATIONS.md` for local required tests/proof.
10. Selected plan's `PROOF_INDEX.md` for proof root and artifact naming.
11. `docs/agent/PLAN_EXECUTION_STANDARD.md` only if the selected workpack lacks required execution fields.
12. `docs/agent/TEST_PROOF_DECISION_MATRIX.md` only after local expectations identify a global risk surface needing escalation.

## Plan selection tree

Use this tree before opening any detailed plan document.

| If the task is about...                                                                                                                           | Select                                    |
| ------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------- |
| Local AI, model runtime, provider routing, AI result validation, memory graph, parent assistant                                                   | `ai-plan`                                 |
| Native app/game evidence, launcher/runtime/foreground/session, app/game policy targets                                                            | `app-game-plan`                           |
| App-only legacy/reconciliation or app-plan workpack history                                                                                       | `app-plan`                                |
| Browser inventory/profile/tab/URL/settings/intervention/social-video/cloud-game browser work                                                      | `browser-plan`                            |
| Reusable Rust event bus, envelope, replay, retry, dead-letter, consumer contract spine                                                            | `eventing-plan`                           |
| Household LAN pairing, peer trust, local transport, provider mesh bridge                                                                          | `lan-plan`                                |
| DNS/connection/request metadata, network risk, domain policy signals                                                                              | `network-plan`                            |
| Parent client runtime package, installer, service lifecycle, launch/update artifacts                                                              | `parent-client-runtime-distribution-plan` |
| Portal household UX, parent/child surfaces, read-model presentation, screenshots                                                                  | `portal-ux-household-surfaces-plan`       |
| OCR/VLM screen AI pipeline, prompt/output safety, model result validation                                                                         | `screen-ai-pipeline-plan`                 |
| Screen capture, surface/window inventory, screenshot custody, retention                                                                           | `screen-plan`                             |
| Location/geofence/device status/tracking sessions, places, tracking alerts                                                                        | `tracking-plan`                           |
| Enforcement action handoff, adapter authority, rollback, audit, tamper/manual-required state                                                      | `v0-8-enforcement-control-plan`           |
| Household creation, public family site, install journey, parent/child role assignment, profile/device pairing readiness, first-run, recovery      | `setup-install-provisioning-plan`         |
| Login, users, household membership, roles, invites, account recovery, session/token lifecycle, device ownership authority                         | `account-identity-family-plan`            |
| Child agent process lifecycle, IPC/controller connection, service supervisor, tamper resilience, per-platform adapter coverage                    | `app-plan`                                |
| Evidence retention policy, delete-tombstone propagation, cursor-based query/pagination, export/migration, custody chain                           | `data-custody-storage-plan`               |
| Policy source of truth, schedule creation/evaluation, timezone/DST boundaries, ask-parent approval flow, nontechnical rule UI, override authority | `policy-control-plane-plan`               |
| App install or purchase request interception, store metadata, platform approval flow, deny/race safety                                            | `app-game-plan`                           |
| Remote screen/live-view relay, remote desktop/control, session grants, relay-unavailable fallback, remote privacy/abuse proof                     | `remote-access-plan`                      |
| Local screenshot capture custody chain, platform capture permissions, protected-surface handling                                                  | `screen-plan`                             |
| Social/video platform URL signal extraction, content-category boundary, account/feed detection, enforcement handoff                               | `browser-plan`                            |
| Parent assistant typed action boundary, chat/conversation flow, AI output safety, enforcement handoff (typed action required)                     | `ai-plan`                                 |
| Notification delivery idempotency, report generation/retention, sync convergence under partial outage, parent-facing report UI                    | `data-custody-storage-plan`               |
| Remote relay, remote access sessions, mobile child-agent parity, Android/iOS entitlements, physical household proof                               | `remote-access-plan`                      |
| Signed installer, distribution channels, update/rollback proof, store submission, privacy/legal, release gates                                    | `parent-client-runtime-distribution-plan` |
| Child package distribution, respawn, tamper/uninstall, signing/device-owner matrix, setup-device-trust handoff                                    | `child-agent-runtime-distribution-plan`   |
| Billing, subscriptions, Stripe Checkout/Portal, payment webhooks, entitlement gates, refunds/disputes, invoices/tax                               | `payment-subscription-plan`               |

If a task names several rows, choose the producer of the first changed contract or
proof claim. The selected workpack may then name a consumer plan; do not open
the consumer plan before that.

## Plan-local stop rules

- `AGENTS.md` decides the plan-local tree; `PLAN_HEALTH.md` is for broad status
  and route-audit claims, not default work context.
- `PLAN_STATE.md` says what is current; old snapshots and preserved full READMEs
  do not override it.
- `WORKPACK_INDEX.md` is the workpack chooser. Opening sibling workpacks is a
  scope expansion and must be recorded in the workpack/proof note.
- `TEST_PROOF_EXPECTATIONS.md` is mandatory before DONE/PR_READY; missing
  expected tests keep the row open.
- `PROOF_INDEX.md` is mandatory before DONE/PR_READY; missing proof roots keep
  the row open.
- Fenced examples in plan docs are contract/artifact/command examples unless the
  document says they define a public contract shape. They are not instructions
  to copy implementation code.

## Do not default-read

- Full `implementation-checklist.md`.
- All workpacks.
- Sibling plans.
- All `docs/features`.
- All `docs/expectations`.
- Historical checkpoint files.
- Full source inventories unless source ownership is unclear.

## During work

Keep implementation claims tied to proof. If you close a row, update the
assigned workpack, `PLAN_STATE.md` if current status changes, the relevant
feature doc/checklist if product status changes, and the proof index if new
artifacts are added.

## Validation choice

Each plan work item must choose a validation profile from the assigned workpack
plus the plan-local `TEST_PROOF_EXPECTATIONS.md`. Do not run a tiny happy-path
check for a multi-boundary claim. Do not run broad validation just to avoid
choosing. Pick the smallest set that covers every touched risk: contract,
runtime, UI, security, persistence, AI, platform, performance, observability,
release, or human workflow.
