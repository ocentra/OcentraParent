# WP03 Classification And Correlation

Scope: classify domains/flows and correlate with app, browser, foreground, and screen signals without inventing certainty.

Source rows: `03-network-implementation-checklist-and-workpacks.md` rows 21-30.

Read next:

- `../01-network-evidence-and-intervention-full-scope-plan.md`
- `../TEST_PROOF_EXPECTATIONS.md`
- `../../browser-plan/AGENTS.md`, `../../app-plan/AGENTS.md`, or `../../screen-plan/AGENTS.md` only when that correlation source is selected

Expected outcome:

- Domain normalization and public suffix behavior are deterministic.
- Domain/category intelligence, social/video/game/cloud-gaming, VPN/proxy/Tor/tunnel, remote desktop/torrent/download, process/app, managed browser, unmanaged browser, app/game foreground, and screen summary correlations are separate evidence inputs.
- Ambiguity is retained as evidence grade, not converted to false certainty.

Expected tests/proof:

- `network.domain-normalization.public-suffix`
- `network.classifier.category-fixtures`
- `network.classifier.vpn-proxy-tunnel`
- `network.classifier.remote-desktop-torrent-download`
- `network.correlation.browser-app-screen`
- `network.classifier.ambiguity-evidence-grade`
- Proof includes false-positive/false-negative cases and cross-plan source references.

Failure conditions:

- Do not infer app usage from DNS alone when app/browser/screen correlation is missing.
- Do not claim a hidden VPN destination from tunnel metadata.
- Do not collapse CDN/shared-host evidence into exact product/content identity.
