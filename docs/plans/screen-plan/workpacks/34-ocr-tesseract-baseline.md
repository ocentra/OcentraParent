# 34 OCR Tesseract Baseline

## Target State

Simple local OCR baseline is evaluated before VLM.

## MVP Boundary

This is AI-pass work unless a lightweight OCR hook is needed for capture MVP proof. Capture MVP should only leave typed OCR job/result hooks.

## Checklist

- [ ] Verify current Tesseract project/docs and license.
- [ ] Test local packaging on Windows first.
- [ ] Test basic UI text extraction.
- [ ] Measure CPU/memory/runtime.
- [ ] Record failure modes on small fonts/messy UI.
- [ ] Compare against PaddleOCR/PP-OCR before selecting.

## Proof

- OCR baseline proof folder.
- Model/runtime status and quality notes.
