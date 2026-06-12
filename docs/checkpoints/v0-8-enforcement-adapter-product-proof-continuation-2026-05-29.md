<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.8 Enforcement Adapter Product Proof Continuation - 2026-05-29
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.8 Enforcement Adapter Product Proof Continuation - 2026-05-29

Branch: `codex/v0-8-enforcement-adapter-product-proof-continuation`

## Scope

This checkpoint continues worker B's V0.8 enforcement-adapter proof chain. It
does not add UI work, C-owned paths, or new blocking behavior. It verifies the
real service paths that exist, keeps broad app/domain/browser behavior
manual-required or unavailable, and records a rejected product claim-upgrade
decision when required host/platform artifacts are missing.

## Proof Command

```powershell
node scripts/test/v0-8-enforcement-adapter-product-proof-continuation.mjs
```

The command runs:

- `cmd /c npm run build:contracts`
- `cargo test -p ocentra-parent-agent-core enforcement_app_time_limit`
- `cargo test -p ocentra-parent-agent-core manual_required_network_and_browser_targets_return_unavailable_audit_without_adapter_execution`
- `cargo test -p ocentra-parent-agent-service enforcement_execute_reports_manual_required_service_states_for_unwired_adapters`
- `cmd /c node scripts/test/v0-8-os-adapter-proof-hardening.mjs`

The wrapper validates generated artifacts and writes:

- `test-results/v0-8-enforcement-adapter-product-proof-continuation/proof.json`
- `test-results/v0-8-os-adapter-proof-hardening/proof.json`
- `test-results/v0-8-windows-app-time-limit-adapter-mvp/*.json`
- `test-results/v0-8-production-enforcement-hardening/*.json`
- `test-results/windows-managed-unmanaged-browser-enforcement-proof/*.json`

## Proof Labels

Expected wrapper labels:

- `v0.8.continuation.os-adapter-hardening-artifact-accepted`
- `v0.8.continuation.app-time-limit-audit-recovery-truth`
- `v0.8.continuation.broad-adapter-manual-required-truth`
- `v0.8.continuation.browser-process-only-nonclaim-truth`
- `proof-matrix.v0-8-enforcement-adapter-product-proof-continuation`
- `v0.8.continuation.claim-upgrade-refusal-proof`

## Honest Boundaries

- Owned-process terminate and app time-limit lifecycle have real service proof
  where the host supports them.
- Broad app blocking stays manual-required or unavailable outside
  owned-process and app time-limit proof.
- Network/domain blocking stays manual-required or unavailable until an
  OS-approved adapter proves real host behavior.
- Managed-browser service commands returning manual-required do not prove exact
  URL enforcement by themselves.
- Unmanaged-browser process evidence does not prove exact URL, active tab, page
  title, download source, page text, HTTPS content, or user intent.
- Product-ready broad app/domain/browser enforcement remains rejected until the
  missing host/platform artifacts are present.

## Manual Proof Still Required

Before upgrading V0.8 product claims:

1. Run the continuation proof command on a real Windows child host and archive
   every generated proof JSON with the commit SHA.
2. Record Rust service logs for app time-limit execute, restart recovery,
   parent cancel, unavailable recovery, expiry, audit, encrypted journal, and
   SQLite storage.
3. Record owned-process terminate, process-id-required, and process-name
   mismatch service logs without presenting them as global app blocking.
4. Record broad app, network/domain, and managed-browser service commands
   returning manual-required or unavailable until real host adapters exist.
5. Use managed-browser intervention proof, not service-command target strings,
   before claiming exact URL enforcement.
6. Record Android device-owner and iOS Family Controls entitlement/device
   artifacts before upgrading mobile child enforcement claims.
