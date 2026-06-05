# 35 OCR PaddleOCR PP-OCR Evaluation

## Target State

Preferred OCR research path evaluates PaddleOCR/PP-OCR packaging, quality, runtime, and local-only proof.

## MVP Boundary

This is AI-pass work. Do not block capture MVP on final OCR selection.

## Checklist

- [x] Verify current PaddleOCR/PP-OCR docs.
- [~] Test Windows packaging and runtime dependencies.
- [~] Test local-only execution.
- [ ] Compare UI text extraction quality against Tesseract.
- [ ] Measure CPU/GPU/memory/runtime.
- [~] Decide whether child device or family hub should run it.

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
already installed, and the local-only default gate. It intentionally does not
install packages, download models, call hosted OCR, or claim production quality.

## Current Decision

- Do not select PaddleOCR/PP-OCR as the production OCR runtime yet.
- Continue to treat this as an evaluation candidate until local package install,
  model-cache custody, no-upload inference, Tesseract comparison, and
  CPU/GPU/memory/runtime measurements pass.
- If the candidate is heavy on child devices, route hard OCR cases through the
  family AI hub only after that LAN/local custody path is runtime-proved.
