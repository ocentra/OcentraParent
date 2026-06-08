# Social Managed Browser Policy Execution Proof

Generated: 2026-06-08T22:27:48.495Z
Branch: codex/d-runtime-ready
Commit: 74ce3b0e558717650a93545921b21a36fe9b04b2
Base: 624290167ea79fc9c3bf59b1d06f1a7461113292

Managed browser evidence: test-results/managed-browser-composited-block-proof/2026-06-08T22-27-34-441Z.json
Managed browser screenshot: test-results/managed-browser-composited-block-proof/2026-06-08T22-27-34-441Z-screenshots/chrome-stable-composited-block-youtube.png
Live surface: real-youtube-watch-page
Child-agent endpoint: /api/browser/intervention/page

Final policy execution claimed: true
Browser mutation observed: true
Child intervention executed: true
Managed intervention enforced: true

No-claim boundaries preserved:
- Unmanaged browser claimed: false
- Broad OS enforcement claimed: false
- Provider delivery attempted: false
- Native app control claimed: false
- Apple platform claimed: false

This proof chains a parent-domain social policy decision candidate to a real
managed-browser composited block run. The managed-browser harness loads a
real YouTube watch page, captures it through CDP, renders the shared child
intervention page through the Rust child-agent endpoint, and observes the
browser tab on the intervention endpoint. The proof does not claim unmanaged
browser control, broad OS enforcement, provider delivery, native app control,
or Apple platform support.
