# Parent Client Artifact Matrix

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `PARENT_CLIENT_ARTIFACT_MATRIX.md`
> Kind: plan reference document.
> Read when: The parent client distribution route needs the canonical artifact split.
> Stop rule: Do not continue into sibling plans unless the selected workpack names a handoff.
> Proves: the artifact split, claim boundary, and proof intent stated below.
> Does not prove: implementation completion, product readiness, or sibling-plan status.
> Proof rule: Update the assigned workpack, proof pointer, and route docs when this matrix changes.

<!-- /agent-capsule -->

Purpose: keep the parent client distribution route honest about which artifact is being proved.

| Artifact                      | What it is                                                       | What it may claim                                              | What it must not claim                                           |
| ----------------------------- | ---------------------------------------------------------------- | -------------------------------------------------------------- | ---------------------------------------------------------------- |
| Parent web portal             | Hosted parent client surface under `apps/portal`                 | Route, auth, cache, and environment separation proof           | Child runtime packaging, setup completion, or device trust       |
| Parent desktop shell/package  | Tauri shell and desktop package preview                          | Launch, local-service bridge, signing/update/rollback boundary | Child device authority, capture, or mobile parity                |
| Parent Android package        | Parent Android app/package preview                               | Device install, signer, and store/manual-required state        | Child-agent runtime distribution or policy behavior              |
| Parent iOS package            | Parent iOS app/package preview                                   | Device install, provisioning, and store/manual-required state  | Child-agent runtime distribution or background service parity    |
| Parent route bridge           | Typed bridge between parent client and local-service/setup state | Readiness and launch contracts only                            | Setup journey ownership, child runtime claims, or policy control |
| Update and rollback artifacts | Update manifest, checksum, SBOM, rollback payload                | Update state, rollback state, and negative-failure evidence    | Production release claims without real artifact proof            |

## Matrix rules

- Proof is collected per artifact, not per folder.
- Setup is a handoff into install state, not a package-proof substitute.
- Parent client distribution is parent-only; child runtime distribution belongs to a separate plan.
- Scaffold-only mobile rows remain manual-required until a real device/install/store proof exists.
