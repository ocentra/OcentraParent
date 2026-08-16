<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.8 OS-Adapter Product Proof
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.8 OS-Adapter Product Proof

Date: 2026-05-30

Scope: typed product proof read model for the V0.8 OS-adapter enforcement boundary. This checkpoint does not claim broad OS blocking, exact browser URL control, anti-tamper, signing, store release, Android device-owner enforcement, or iOS Family Controls.

## Captured Proof Rows

- Owned-process terminate: implemented where the host process adapter supports pid/name guardrails.
- App time-limit lifecycle: implemented where the host timer/process adapter supports local timer state, expiry, parent cancel, restart recovery, and audit.
- Broad app blocking: manual-required until OS-approved app/package identity, apply, rollback, and custody artifacts exist.
- Network/domain blocking: manual-required until a host network filter adapter has apply, rollback, and custody artifacts.
- Managed-browser service command: manual-required and not exact URL control.
- Managed-browser exact URL: manual-required until managed browser active-tab/exact URL apply and audit proof exists.
- Unmanaged-browser process-only: implemented only as pid/name process evidence where the process adapter supports it.
- Unmanaged-browser exact evidence: not claimed.
- Restart recovery: local timer custody proof, not anti-tamper or bypass-resistance proof.
- Parent cancel/override: timer-scoped cancel proof, not broad unblock rollback.
- Audit custody: local journal/store proof, not production anti-tamper hardening.
- Rollback artifact gate: manual-required until same-identity apply, rollback, and custody artifacts are present.

## Validation Command

```powershell
cmd /c node scripts/test/v0-8-os-adapter-product-proof.mjs
```

The harness writes `test-results/v0-8-os-adapter-product-proof/proof.json` with entry counts, proof labels, validation commands, proved claims, non-claims, and runtime wiring gaps.
