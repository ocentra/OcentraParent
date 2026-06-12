<!-- agent-capsule -->

> Agent Capsule
> Doc: Activity Surface Main-Backed Adapter Proof
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Activity Surface Main-Backed Adapter Proof

Date: 2026-05-29
Owner lane: codex-b
Branch: codex/activity-surface-main-backed-adapter

## Scope

This checkpoint proves the non-visual Activity surface adapter foundation from the current main branch. The proof covers typed report and tab read-model requests for:

- Reports
- Screen
- App Use
- Browser
- Games
- Network

The slice is service/read-model proof only. It does not change C-owned Activity visual layout files, vendor catalog UI paths, A parent-assistant/API-AI files, or D mobile bridge files.

## Product Truth

Activity product data must come from typed Rust service/read-model paths. Vite is only the development shell and must not own product Activity data.

The Activity surface keeps unavailable or local read-model conditions explicit through typed states: ready, empty, unavailable, offline, stale, permission-required, and scaffold-only. It does not convert missing real household data into UI-check fixture data.

## Focused Proof

Required command:

```powershell
node scripts/test/activity-surface-main-backed-adapter-proof.mjs
```

The proof command runs:

- TypeScript contract build.
- Activity domain contract tests.
- Agent protocol-domain Activity adapter tests.
- Rust protocol Activity tests.
- Rust service Activity dispatcher/read-model tests.
- Portal live Activity state and Activity UI intent tests.
- Real Rust service runtime proof for Activity report and read-model commands.
- Pre-AI proof matrix validation.

It writes:

```text
test-results/activity-surface-main-backed-adapter/proof.json
```

## Evidence

- Activity domain contracts: `packages/activity-domain/src/activity-surface.ts`
- Activity domain tests: `packages/activity-domain/tests/activity-surface.test.ts`
- Portal/agent command and event adapter: `packages/agent-protocol-domain/src/activity-surface-adapter.ts`
- Adapter tests: `packages/agent-protocol-domain/tests/activity-surface-adapter.test.ts`
- Rust protocol shapes: `crates/agent-protocol/src/activity_surface.rs`
- Rust service adapter: `crates/agent-service/src/activity_surface_adapter.rs`
- Rust service read models: `crates/agent-service/src/activity_surface_read_models.rs`
- Dispatcher proof: `crates/agent-service/src/activity_surface_main_backed_adapter_tests.rs`
- Portal state proof: `apps/portal/tests/live-activity-surface-adapter.test.ts`
- UI intent proof: `apps/portal/tests/activity-ui-intent.test.ts`
- Runtime proof harness: `scripts/test/activity-surface-main-backed-adapter-proof.mjs`
- Matrix registration: `docs/expectations/pre-ai-proof-matrix.json`

## Known Gaps

- C-owned visual polish and product UX remain incomplete, but the Activity UI
  intent seam consumes the merged adapter surface.
- Family fan-out beyond local service state and data storage destination selection remain typed local or unavailable behavior.
- This checkpoint does not claim mobile parent parity, Android device-owner enforcement, iOS entitlement proof, visible browser proof, Playwright proof, package validation, or full root validation.
