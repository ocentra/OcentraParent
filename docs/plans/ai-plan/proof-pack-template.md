# AI Proof Pack Template

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `AI Proof Pack Template`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

Each AI workpack that changes implementation must produce a proof pack.

## Proof Pack Contents

- Branch and commit.
- Workpack id.
- Touched paths.
- Contract changes.
- Runtime/service changes.
- Portal/UI changes.
- Feature docs/checklist updates.
- Validation commands and results.
- Test output paths.
- Proof script output paths.
- UI screenshot paths when UI changed.
- Real capture proof refs when AI consumes screen-derived evidence.
- Real AI analysis proof refs.
- Pipeline proof refs when this is the final `screen-ai-pipeline-plan` pass.
- Event chain manifest path when eventing is in scope.
- Mesh bridge proof path when household provider execution is in scope.
- Provider discovery/capability proof path when provider routing changes.
- AI work lifecycle proof path when queue behavior changes.
- Claim/lease/idempotency proof path when household providers can claim work.
- Result validation proof path when provider results cross runtime boundaries.
- Policy authority proof path when AI provider output can influence policy.
- Raw-payload custody proof path when screen-derived work is in scope.
- Mobile dormant/fallback proof path when mobile provider behavior is in scope.
- Topology/orphan-event proof path when event families are added.
- Known gaps and non-claims.

## Required Evidence

- Schema parser proof.
- Rust parity proof when Rust crosses the contract.
- Stored-evidence context proof when AI consumes evidence.
- Provider route/status proof when runtime changes.
- Invalid-output proof when model parsing changes.
- Policy integration proof when decisions change.
- Security negative proof when custody, memory, remote, or screenshots are in
  scope.
- Performance/resource proof when model execution or queue behavior changes.
- Real analysis proof when AI consumes browser/app/screen-derived evidence.
- Event topology proof for AI mesh events when eventing is in scope.
- Mesh bridge incoming/outgoing validation proof when cross-device behavior is
  in scope.
- Duplicate message/idempotency proof when LAN messages or provider claims are
  in scope.
- Claim/lease proof when household providers execute AI work.
- Expired lease and wrong-provider result rejection proof when leases exist.
- Provider-cannot-publish-policy/enforcement proof when providers return
  results.
- Child-agent-only policy authority proof when provider results feed policy.
- Raw screenshot no-transfer proof when screen-derived work can leave the
  child device.
- Final pipeline proof only when the workpack is the post-screen/post-AI
  integration pass.

## DONE Report Minimum

```text
Workpack:
Branch:
Commit:
Pushed:
Touched files:
Validation:
Proof artifacts:
UI screenshots:
Real capture proof refs:
Real AI analysis proof refs:
Pipeline proof refs:
Event chain refs:
Mesh bridge proof refs:
Claim/lease/idempotency proof refs:
Result validation proof refs:
Policy authority proof refs:
Raw-payload custody proof refs:
Feature doc/checklist:
Known gaps:
Non-claims:
```
