# Screen Service WinRT OCR Source Snapshot

- Live browser surface: data:text/plain;charset=utf-8,School%20portal%20account%20jane%40example.com%0AParent%20phone%20555-010-1234%0Apassword%20reset%20token%20visible
- Browser title: data:text/plain;charset=utf-8,School portal account jane%40example.com%0AParent phone 555-010-1234%0Apassword reset token visible - Google Chrome
- Pixel capture: Rust agent service timed cadence active-window capture.
- Evidence queue: service encrypted temp queue, drained after analysis.
- OCR runtime: Windows `Windows.Media.Ocr.OcrEngine` inside service adapter process.
- Parent-selected redaction policy: output\screen-ai-pipeline-proof\service-winrt-ocr-redaction\parent-redaction-policy.json
- OCR terms found: jane, 555, password
- Raw captured image artifact: not retained; adapter temp image deleted after OCR.
