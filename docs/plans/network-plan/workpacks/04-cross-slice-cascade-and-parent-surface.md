# WP04 Cross Slice Cascade And Parent Surface

Scope: route network evidence into cross-slice evidence bundles, local AI queue candidates, policy mapping, notifications, and parent UI without bypassing custody or authority.

Source rows: `03-network-implementation-checklist-and-workpacks.md` rows 31-36.

Read next:

- `../ui-ux-requirements-guide.md`
- `../../ai-plan/AGENTS.md` only for AI queue/evaluator handoff
- `../../policy-control-plane-plan/AGENTS.md` only for policy mapping
- `../../portal-ux-household-surfaces-plan/AGENTS.md` only for rendered parent surface

Expected outcome:

- Cascade router records why network evidence should or should not trigger browser/app/screen/AI follow-up.
- Cross-slice bundle preserves source, custody, grade, confidence, and stale/partial status.
- AI queue receives structured evidence refs, not raw PCAP or private content.
- Parent notification candidates and network evidence drawer show bounded claims and unavailable/manual-required states.

Expected tests/proof:

- `network.cascade.router-decision`
- `network.bundle.source-custody-grade`
- `network.ai-queue.raw-pcap-forbidden`
- `network.policy-mapping.evidence-grade`
- `network.parent-drawer.render-proof`
- `network.notification.candidate-boundary`
- Proof includes UI artifact path if rendered, redaction proof, and skipped-risk notes.

Failure conditions:

- Do not send raw packet payloads to AI.
- Do not let parent UI imply exact content from network metadata.
- Do not turn notification candidate rows into delivered notifications without notification-provider proof.

Current production truth (2026-08-16):

- The shipped read path retains real ActivityStore observations and runtime-delivery state.
- The former product-path caller, payload fields, service bridge, and disconnected evidence pipeline were deleted because they manufactured AI, policy, adapter, custody, export, and portal references from one observation rather than consuming authoritative owners.
- Tests/support that import or bless those deleted APIs are invalidated and must be deleted or rewritten during the test phase.
- WP04 remains blocked until shipped owner-backed cascade, AI queue, policy request, notification, custody, and parent-surface behavior exists; deterministic builders or test-created references do not close it.

## Direct composition prerequisites (2026-08-16 audit)

The shipped-call audit found no real cascade owner behind the former product
path. Real capture persists observations through `ActivityStore`; the apparent
cascade is only the `agent-service` `NetworkRuntimeDelivery` `OnceCell`, the
`agent-core` `NetworkRuntimeSpine`/`EventBus::new` path, read-time republish,
and `refs.rs` phase-reference manufacturing. There is no typed durable
`NetworkCascadeObligation`, durable cascade table, or shipped composition owner
that can legally fan one observation into the following consumer boundaries.

WP04 is therefore blocked behind these direct owner handoffs. They are
composition prerequisites, not completion claims for the owner workpacks:

| Owner boundary | Required handoff | Why WP04 cannot bypass it |
| --- | --- | --- |
| Eventing WP09 | Network consumer event chain | Ingestion-time publish, durable network journal, startup recovery, and the observation-to-read-model authority chain are missing from the shipped path. |
| AI WP07 | AI job queue contract | Network evidence must enqueue a bounded, leased, idempotent job through the AI owner. |
| AI WP19 | AI result journal SQLite ingest | AI results and policy decisions need a durable journal/read-model handoff rather than refs minted at read time. |
| Policy WP05 | Ask-parent overrides | A network-triggered request must use the policy-owned approval, expiry, replay, and notification handoff. |
| Policy WP08 | Policy event model | Policy causation, idempotency, replay, audit, rollback, and manual-required states must be authoritative. |
| Custody WP04 | Retention delete tombstone | Cascade outputs must honor delete/tombstone ordering and cannot resurrect derived evidence. |
| Custody WP06 | Report query custody | Reports and notification payloads need governed source refs, redaction, cursor, stale, and deleted-source states. |
| Portal WP09 | Browser/app/network surfaces | The parent network drawer must render metadata-only evidence and unavailable/degraded states. |
| Portal WP12 | Reports, notifications, and custody | Parent reports/alerts must expose custody and delivery state without implying provider delivery. |

No owner above is marked complete by this route. Until these handoffs are
shipped and independently proved, WP04 is not a legal READY workpack; the
invalidated product-path tests remain test-phase delete/rewrite debt.
