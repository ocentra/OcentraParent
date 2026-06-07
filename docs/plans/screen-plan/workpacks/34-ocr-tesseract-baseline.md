# 34 OCR Tesseract Baseline

## Target State

Simple local OCR baseline is evaluated before VLM.

## MVP Boundary

This is AI-pass work unless a lightweight OCR hook is needed for capture MVP proof. Capture MVP should only leave typed OCR job/result hooks.

## Current State

The Tesseract baseline source/license/docs check is complete. The upstream
project and tessdoc pages identify Tesseract as the Apache-2.0 OCR engine
candidate, and tessdoc records that newer Windows builds do not have an
official upstream Windows installer.

The B-lane Windows worktree installed `tesseract-ocr.tesseract`
5.5.0.20241111 through winget and resolved the binary at
`C:\Program Files\Tesseract-OCR\tesseract.exe`. The proof runs that local
runtime against a retained real managed-browser public Vimeo screenshot
artifact and extracts expected visible terms (`vimeo`, `video`, `player`).
The proof now records process duration, CPU time, and peak working set for the
main extraction plus three derived failure-mode scenarios from the same real
screenshot: alternate page segmentation, downscaled small text, and cropped
player UI. The current shell PATH still does not see `tesseract`, so the script
records the standard install path explicitly.

PaddleOCR/PP-OCR comparison remains open before selecting a production OCR
runtime.

## Checklist

- [x] Verify current Tesseract project/docs and license.
- [x] Test local packaging on Windows first.
- [x] Test basic UI text extraction.
- [x] Measure CPU/memory/runtime.
- [x] Record failure modes on small fonts/messy UI.
- [ ] Compare against PaddleOCR/PP-OCR before selecting.

## Proof

- `output/screen-plan-proof/34-ocr-tesseract-baseline/proof-summary.json`.
- `output/screen-plan-proof/34-ocr-tesseract-baseline/vimeo-public-video-tesseract-output.txt`.
- `output/screen-plan-proof/34-ocr-tesseract-baseline/vimeo-public-video-tesseract-failure-modes.txt`.
- Runtime status currently records `runtime-extraction-proved`.
- Model/runtime quality notes remain open until Tesseract is compared with
  PaddleOCR/PP-OCR and a production OCR runtime is selected through a separate
  quality/resource gate.

Validation:

```powershell
node --check scripts/test/screen-ocr-tesseract-baseline-proof.mjs
node scripts/test/screen-ocr-tesseract-baseline-proof.mjs
```
