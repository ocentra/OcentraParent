# AI Proof Pack Template

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
Feature doc/checklist:
Known gaps:
Non-claims:
```
