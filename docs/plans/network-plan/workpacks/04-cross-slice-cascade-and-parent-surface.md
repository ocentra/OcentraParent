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
- WP04 remains open until shipped owner-backed cascade, AI queue, policy request, notification, custody, and parent-surface behavior exists; deterministic builders or test-created references do not close it.
