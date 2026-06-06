# AI-20 Parent Explanation Runtime Source Snapshot

checkedAt=2026-06-06T01:29:02.632Z
branch=codex/browser-child-intervention-endpoint-flow
commit=b74ae680014f90585266fa4619b4a260fb0c5ada
sourceChildUxProof=test-results/browser-ai-child-ux-rendered-proof/2026-06-06T00-57-51-063Z.json
sourceChildUxScreenshot=test-results/browser-ai-child-ux-rendered-proof/2026-06-06T00-57-51-063Z-screenshots/chrome-stable-child-ux-warning.png

The parent explanation proof consumes the live AI-19 YouTube CDP child UX proof as source evidence.
The portal receives only a schema-decoded explanation bundle through the dedicated proof env var.
Raw target URL, page body, prompt text, screenshots, cookies, tokens, and browser storage are not rendered or stored in the parent explanation bundle.
