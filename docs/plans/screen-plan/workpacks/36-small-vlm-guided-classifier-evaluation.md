# 36 Small VLM Guided Classifier Evaluation

## Target State

Small local VLM is used only for guided classification on safe crops when structured/OCR evidence is insufficient.

## MVP Boundary

This is AI-pass work. Capture MVP should define the route/result contracts and defer model choice.

## Checklist

- [ ] Verify configured local Gemma-family image capability if it is the parent/device default.
- [ ] Evaluate Qwen2.5-VL or other candidates only if default runtime is insufficient.
- [ ] Use detector-specific JSON prompts.
- [ ] Limit image pixels and crop regions.
- [ ] Reject open-ended descriptions.
- [ ] Measure runtime and quality.
- [ ] Record uncertainty/manual-required behavior.

## Proof

- Guided detector test set.
- Model capability proof.
- Resource proof.
