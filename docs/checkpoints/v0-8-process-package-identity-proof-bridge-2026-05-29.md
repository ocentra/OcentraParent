<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.8 Process Package Identity Proof Bridge
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.8 Process Package Identity Proof Bridge

Date: 2026-05-29

## Scope

This checkpoint records the typed proof bridge for Windows process and package identity prerequisites before any broad app-blocking claim can upgrade.

The bridge captures installed app inventory, process lineage, executable identity, package identity, publisher/signature state, inventory/process matching, unsupported identity fallback, rollback readiness, and audit custody requirements.

## Proof Command

```powershell
node scripts/test/v0-8-process-package-identity-proof-bridge.mjs
```

The command builds contracts, runs the focused parent-domain bridge tests, verifies proof-matrix entries, and writes:

```text
test-results/v0-8-process-package-identity-proof-bridge/proof.json
```

## Product Truth

- The bridge is contract and CI-mechanical proof only.
- Windows process/package identity remains manual-required until real host inventory, lineage, package, publisher/signature, apply, rollback, and audit artifacts exist.
- Unknown apps remain unknown. Unknown, unsupported, or permission-limited identity must not become a known app, risky app, blocked target, or game.
- Rollback readiness for broad app blocking is not claimed by this slice.
- Audit custody must come from real service paths, not Portal-local state.

## Known Gaps

- No broad app blocking implementation is added.
- No network/domain blocking, managed-browser exact URL enforcement, unmanaged exact URL evidence, admin anti-tamper, rollback enforcement, Android child behavior, or iOS child behavior is claimed.
- Local validation intentionally avoids visible browser, Playwright, portal E2E, managed-browser-profile/intervention proof, and full `npm run validate` unless primary or the user asks.
