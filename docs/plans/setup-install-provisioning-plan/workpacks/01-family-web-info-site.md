# Workpack 01: Family Web Info Site

Goal: define `family.ocentra.ca` as the public family product entry surface.

Owns: informational pages, invite/code entry, install/download entry, privacy promise copy, no-child-data boundary, deployment shape, bootstrap-code display states, and route handoff to account registration.

Does not own: account/session implementation, installer build mechanics, portal app internals, or child activity storage.

Expected shape:

- Separate Vite surface or Cloudflare Pages/Workers app with deploy/preview route.
- Clear distinction between informational browsing and authenticated account actions.
- Invite link entry, manual code entry, and QR/deep-link entry are explicit.
- No child activity collection on public pages.
- Minimal telemetry only if explicitly documented, privacy-safe, and disabled or consented where required.
- Links to registration/login, installer download, support, privacy, and status.
- Parent install link handoff and bootstrap code state display are visible.

Expected proof:

- Route/content audit.
- Build/deploy preview proof when implemented.
- Privacy/data collection review.
- Broken-link and basic accessibility proof.

Failure: treating marketing analytics, contact capture, or child setup state as hidden website data collection.

## Execution Detail

Minimum context:

- `docs/expectations/family-setup.md`
- `docs/expectations/release-installer.md`
- `docs/expectations/data-custody.md`
- `docs/roadmaps/roadmap-v1-0-local-mvp.md`
- `docs/roadmaps/roadmap-v8-production-hardening.md`

Agent decision tree:

- If the task is content-only, update page intent, route names, privacy wording, and proof expectations; do not touch app/runtime docs.
- If the task needs registration, route to `account-identity-family-plan` after defining the website handoff.
- If the task needs installer downloads, route to `parent-client-runtime-distribution-plan` after defining the visible download state.
- If the task needs data collection or telemetry, route to `data-custody-storage-plan` before claiming it is privacy-safe.

Required output:

- Public page map: home, download, register/login, privacy, support, status, install help.
- Data collection matrix: none, anonymous operational telemetry, explicit account data, forbidden child data.
- Deployment shape: separate Vite/Cloudflare Pages or Workers app, preview URL expectation, production domain expectation.
- Copy constraints: no unproven enforcement, no "we store nothing" overclaim, no vague privacy promise.

Expected tests/proof names:

- `setup.public-site.route-map`
- `setup.public-site.no-child-activity-collection`
- `setup.public-site.data-collection-matrix`
- `setup.public-site.privacy-copy-no-overclaim`
- `setup.public-site.download-entry-visible`
- `setup.public-site.registration-handoff-visible`
- `setup.public-site.support-privacy-status-links`
- `setup.public-site.link-check`
- `setup.public-site.accessibility-basic`
- `setup.public-site.cloudflare-preview-build`
- `setup.public-site.custom-domain-manual-required`

Proof artifact expectations:

- `01-public-site-route-map-proof.md`
- `01-no-child-data-collection-proof.md`
- `01-privacy-copy-no-overclaim-proof.md`
- `01-link-accessibility-proof.md`
- `01-deploy-preview-proof.md`
