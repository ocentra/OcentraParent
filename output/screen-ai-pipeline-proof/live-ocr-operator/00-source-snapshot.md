# Screen WinRT OCR Worker Source Snapshot

- Live browser surface: public pages opened in real Chromium.
- Native app surface: Windows Notepad opened with a generated local text file.
- Pixel capture: `ocentra-parent-screen-capture-adapter` real selected-window proof example.
- OCR runtime: Windows `Windows.Media.Ocr.OcrEngine` from user profile languages.
- Raw capture files are kept only as analysis temp files until OCR completes, then deleted.
