# 34 OCR Tesseract Baseline

## Target State

Simple local OCR baseline is evaluated before VLM.

## MVP Boundary

This is AI-pass work unless a lightweight OCR hook is needed for capture MVP proof. Capture MVP should only leave typed OCR job/result hooks.

## Current State

The Tesseract baseline source/license/docs check is complete. The upstream
project and tessdoc pages identify Tesseract as the Apache-2.0 OCR engine
candidate, and tessdoc records that newer Windows builds do not have an
official upstream Windows installer. The B-lane Windows worktree does not have
`tesseract` on `PATH`, so local extraction, CPU/memory/runtime measurement,
failure-mode capture, and PaddleOCR comparison remain blocked behind
install/package proof.

## Checklist

- [x] Verify current Tesseract project/docs and license.
- [~] Test local packaging on Windows first.
- [ ] Test basic UI text extraction.
- [ ] Measure CPU/memory/runtime.
- [ ] Record failure modes on small fonts/messy UI.
- [ ] Compare against PaddleOCR/PP-OCR before selecting.

## Proof

- `output/screen-plan-proof/34-ocr-tesseract-baseline/proof-summary.json`.
- Runtime status currently records `runtime-unavailable`.
- Model/runtime quality notes remain open until a local Tesseract binary and
  language data are installed/proved.

Validation:

```powershell
node --check scripts/test/screen-ocr-tesseract-baseline-proof.mjs
node scripts/test/screen-ocr-tesseract-baseline-proof.mjs
```
