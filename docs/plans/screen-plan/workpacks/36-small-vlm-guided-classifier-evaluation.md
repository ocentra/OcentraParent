# 36 Small VLM Guided Classifier Evaluation

## Target State

Small local VLM is used only for guided classification on safe crops when structured/OCR evidence is insufficient.

## MVP Boundary

This is AI-pass work. Capture MVP should define the route/result contracts and defer model choice.

## Checklist

- [~] Verify configured local Gemma-family image capability if it is the parent/device default.
- [~] Evaluate Qwen2.5-VL or other candidates only if default runtime is insufficient.
- [~] Use detector-specific JSON prompts.
- [~] Limit image pixels and crop regions.
- [x] Reject open-ended descriptions.
- [ ] Measure runtime and quality.
- [~] Record uncertainty/manual-required behavior.

## Proof

- Guided detector test set.
- Model capability proof.
- Resource proof.

Current readiness proof:

```powershell
node scripts/test/screen-vlm-guided-classifier-readiness-proof.mjs
```

Artifact:

```text
output/screen-plan-proof/36-small-vlm-guided-classifier-evaluation/proof-summary.json
```

This proof reuses the existing local VLM execution-readiness contract proof to
show the guided worker template/version, local-only custody, bounded image-pixel
budget, deleted query-store requirement before completed status, and
manual-required behavior when the runtime is unavailable. The focused worker
contract now rejects open-ended prompts such as `Describe the screen in detail.`
before local worker handoff.

The current Windows-lane proof also records local provider command probes for
`ollama`, LM Studio `lms`, legacy `lmstudio`, and `llama-server`. The LM Studio
`lms` CLI is available at v0.0.47, but the local server is off/not running and
`lms ps` cannot list loaded models because it tries to spawn the missing
`C:\Program Files\LM Studio\LM Studio.exe` service path. `ollama`, legacy
`lmstudio`, and `llama-server` are not available on PATH in this worktree. It
does not run a local VLM, measure classifier quality, prove crop extraction, or
select a production VLM model.
