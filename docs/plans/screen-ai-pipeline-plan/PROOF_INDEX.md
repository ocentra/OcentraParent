<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `Screen AI Pipeline Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR readiness/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Screen AI Pipeline Proof Index

## Current audited truth

- No retained `output/screen-ai-pipeline-proof/` directory currently exists in this checkout.
- No `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` currently exists.
- Do not trust historical checked rows until proof is regenerated and retained.

## Proof root

```text
output/screen-ai-pipeline-proof/
```

## Directory routing

- Proof directories are scenario-based, not workpack-file-stem based.
- Follow the exact scenario IDs named by the assigned workpack or checklist row.
- Common examples in this plan include:

```text
output/screen-ai-pipeline-proof/service-foreground/
output/screen-ai-pipeline-proof/service-cadence/
output/screen-ai-pipeline-proof/service-analysis/
output/screen-ai-pipeline-proof/live-operator/
output/screen-ai-pipeline-proof/final-product-path/
```

## Artifact-shape warning

- Current plan docs use two artifact shapes:
  - scenario-local `proof-summary.json` artifacts named throughout the workpacks and `implementation-checklist.md`
  - the richer numbered scenario bundle defined in `pipeline-proof-matrix.md`
- Resolve the expected artifact shape in the assigned workpack before checking any row.

## Minimum retained evidence

```text
<exact artifact(s) named by the assigned workpack>
<exact command log for the run or blocker>
```
