# 35 OCR PaddleOCR PP-OCR Evaluation

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `35 OCR PaddleOCR PP-OCR Evaluation`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Target State

Preferred OCR research path evaluates PaddleOCR/PP-OCR packaging, quality, runtime, and local-only proof.

## MVP Boundary

This is AI-pass work. Do not block capture MVP on final OCR selection.

## Checklist

- [ ] Verify current PaddleOCR/PP-OCR docs.
- [ ] Test Windows packaging and runtime dependencies.
- [ ] Test local-only execution; current PaddleOCR 3.x / PP-OCRv5 executes
      locally but extracts zero text, while a pinned PaddleOCR 2.x fallback runs
      locally.
- [ ] Compare UI text extraction quality against Tesseract; the pinned 2.x
      fallback matches the baseline terms, while current PP-OCRv5 remains
      rejected for this Windows route.
- [ ] Measure CPU/GPU/memory/runtime; the pinned 2.x fallback records CPU, peak
      RSS, init, and predict timing, and the current 3.x candidate records
      runtime timings before its zero-text rejection.
- [ ] Decide whether the child device or a trusted household mesh provider
      should run it.

## Proof

- OCR candidate comparison.
- Packaging proof.
- Local-only/no-upload proof.

Current evaluation proof:

```powershell
node scripts/test/screen-ocr-paddleocr-evaluation-proof.mjs
```

Artifact:

```text
output/screen-plan-proof/35-ocr-paddleocr-ppocr-evaluation/proof-summary.json
```

The current proof verifies official PaddleOCR 3.x / PP-OCRv5 documentation,
current PyPI candidate versions for `paddleocr` and `paddlepaddle`, local Python
and pip availability, whether PaddleOCR/PaddlePaddle/Tesseract runtimes are
installed, the local-only default gate, official local model-cache custody, and
an explicit local runtime attempt when requested:

```powershell
$env:OCENTRA_RUN_PADDLEOCR_LOCAL="1"; node scripts/test/screen-ocr-paddleocr-evaluation-proof.mjs
```

The current Windows runtime attempt installs/imports `paddleocr` 3.6.0 and
`paddlepaddle` 3.3.1, caches official PP-OCRv5 model files locally under the
user PaddleX cache, and runs against the retained real public Vimeo screenshot
used by the Tesseract baseline. It fails before text extraction with a
PaddlePaddle oneDNN/PIR runtime error:
`ConvertPirAttribute2RuntimeAttribute not support [pir::ArrayAttribute<pir::DoubleAttribute>]`.
The proof also records the packaging risk that the user Python environment has
protobuf 5.29.5 while `mediapipe` requires protobuf `<5`.

The same proof can also run an explicitly prepared isolated Python 3.10 fallback
venv when requested:

```powershell
$env:OCENTRA_RUN_PADDLEOCR_LOCAL="1"
$env:OCENTRA_RUN_PADDLEOCR_2X_LOCAL="1"
node scripts/test/screen-ocr-paddleocr-evaluation-proof.mjs
```

That pinned fallback uses `paddleocr` 2.7.0.3, `paddlepaddle` 2.6.2, and
`numpy<2` in `output/screen-plan-proof/35-ocr-paddleocr-ppocr-evaluation/paddleocr-2x-py310-venv`.
It completed local inference against the retained Vimeo screenshot, extracted
15 text strings, matched the baseline `vimeo`, `video`, and `player` terms,
and recorded init/predict timing, CPU time, and peak RSS in
`output/screen-plan-proof/35-ocr-paddleocr-ppocr-evaluation/paddleocr-2x-py310-runtime.log`.
This proves a local PaddleOCR-family fallback can analyze the same real
screenshot, but the evaluation decision is still to not select PaddleOCR/PP-OCR
for the current production OCR route.

The proof does not call hosted OCR or select PaddleOCR as the production OCR
runtime.

## Current Decision

- Do not select PaddleOCR/PP-OCR as the current production OCR runtime.
- Treat current PaddleOCR 3.x / PP-OCRv5 as rejected on this Windows CPU lane
  because it executes locally but extracts zero text from the retained real
  proof image.
- Treat the pinned PaddleOCR 2.x fallback as local-runtime proved but
  not production-selected because it still needs explicit dependency pinning,
  model-cache custody policy, broader quality/resource proof, and product
  ownership review.
- Keep Tesseract/WinRT as the production-leading local OCR paths in this lane
  until the current PaddleOCR path or the pinned fallback passes production OCR
  selection gates.
- Route hard OCR cases through a trusted household mesh provider only after the
  provider proves the same local model-cache custody, no-hosted-OCR boundary,
  runtime success, lease ownership, and child-agent result validation.

## Completion Note

This workpack is complete as an OCR candidate evaluation and non-selection
record. It does not claim PaddleOCR production readiness. Future work can reopen
a new PaddleOCR selection gate only with a new model/package candidate or a
deliberate decision to productionize the pinned 2.x fallback.
