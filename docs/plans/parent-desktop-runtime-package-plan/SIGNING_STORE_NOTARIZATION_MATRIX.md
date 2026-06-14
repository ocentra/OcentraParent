# Signing, Store, and Notarization Matrix

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `SIGNING_STORE_NOTARIZATION_MATRIX.md`
> Kind: plan reference document.

<!-- /agent-capsule -->

| Artifact                     | Signing state                        | Store/notarization state                             | Claim allowed                    |
| ---------------------------- | ------------------------------------ | ---------------------------------------------------- | -------------------------------- |
| Parent web portal            | deploy signature / environment proof | hosted release proof                                 | parent web distribution only     |
| Parent desktop shell/package | code signing / installer signing     | notarization or package-store proof where applicable | parent desktop distribution only |
| Parent Android package       | app signing / bundle signing         | Play / sideload / manual-required proof              | parent Android distribution only |
| Parent iOS package           | app signing / provisioning           | TestFlight / App Store / manual-required proof       | parent iOS distribution only     |

## Rules

- Signing proof must name the artifact.
- Store proof must name the store or distribution channel.
- Notarization proof must not be used as a blanket production claim.
- Manual-required states stay visible until a real artifact exists.
