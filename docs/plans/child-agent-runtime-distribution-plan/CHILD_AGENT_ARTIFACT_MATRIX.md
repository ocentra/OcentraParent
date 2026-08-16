# Child Agent Artifact Matrix

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Doc: `CHILD_AGENT_ARTIFACT_MATRIX.md`
> Kind: plan reference document.

<!-- /agent-capsule -->

| Artifact                          | What it is                          | What it may claim                           | What it must not claim                        |
| --------------------------------- | ----------------------------------- | ------------------------------------------- | --------------------------------------------- |
| Child Windows package             | Child agent Windows package preview | service lifecycle and respawn proof         | parent client distribution or setup ownership |
| Child macOS package               | Child agent macOS package preview   | launchd lifecycle and notarization proof    | parent client distribution or setup ownership |
| Child Linux package               | Child agent Linux package preview   | service-manager lifecycle and package proof | parent client distribution or setup ownership |
| Child Android package             | Child agent Android package preview | install proof and custody/device-owner gaps | parent client distribution or setup ownership |
| Child iOS capability package      | Child agent iOS capability preview  | provisioning/manual-required state          | desktop-style service claims                  |
| Parent-authorized uninstall proof | Revocation and uninstall artifact   | custody-bound uninstall behavior            | stealth persistence                           |

## Matrix rules

- Proof is collected per artifact, not per folder.
- Parent client distribution is separate.
- Mobile rows must show manual-required gaps honestly.
- Service respawn and uninstall resistance are platform-specific claims.
