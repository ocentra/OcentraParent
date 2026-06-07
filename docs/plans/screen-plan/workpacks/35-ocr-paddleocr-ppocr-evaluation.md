# 35 OCR PaddleOCR PP-OCR Evaluation

## Target State

Preferred OCR research path evaluates PaddleOCR/PP-OCR packaging, quality, runtime, and local-only proof.

## MVP Boundary

This is AI-pass work. Do not block capture MVP on final OCR selection.

## Checklist

- [x] Verify current PaddleOCR/PP-OCR docs.
- [x] Test Windows packaging and runtime dependencies.
- [~] Test local-only execution.
- [~] Compare UI text extraction quality against Tesseract.
- [~] Measure CPU/GPU/memory/runtime.
- [x] Decide whether the child device or a trusted household mesh provider
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

The proof does not call hosted OCR or select PaddleOCR as the production OCR
runtime.

## Current Decision

- Do not select PaddleOCR/PP-OCR as the production OCR runtime yet.
- Treat PaddleOCR/PP-OCR as runtime-blocked on this Windows CPU lane until the
  PaddlePaddle inference error is fixed or a pinned alternate runtime is proved.
- Keep Tesseract/WinRT as the only runtime-proved local OCR paths in this lane
  until PaddleOCR completes local inference and produces comparable text.
- Route hard OCR cases through a trusted household mesh provider only after the
  provider proves the same local model-cache custody, no-hosted-OCR boundary,
  runtime success, lease ownership, and child-agent result validation.
