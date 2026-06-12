# Workpack 01: Family Web Info Site

Goal: define `family.ocentra.ca` as the public family product entry surface.

Owns: informational pages, install/download entry, privacy promise copy, no-child-data boundary, deployment ownership, and route handoff to account registration.

Does not own: account/session implementation, installer build mechanics, portal app internals, or child activity storage.

Expected shape:

- Separate Vite surface or Cloudflare Pages/Workers app with deploy/preview route.
- Clear distinction between informational browsing and authenticated account actions.
- No child activity collection on public pages.
- Minimal telemetry only if explicitly documented, privacy-safe, and disabled or consented where required.
- Links to registration/login, installer download, support, privacy, and status.

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
- If the task needs installer downloads, route to `parent-desktop-runtime-package-plan` after defining the visible download state.
- If the task needs data collection or telemetry, route to `data-custody-storage-plan` before claiming it is privacy-safe.

Required output:

- Public page map: home, download, register/login, privacy, support, status, install help.
- Data collection matrix: none, anonymous operational telemetry, explicit account data, forbidden child data.
- Deployment shape: separate Vite/Cloudflare Pages or Workers app, preview URL expectation, production domain expectation.
- Copy constraints: no unproven enforcement, no "we store nothing" overclaim, no vague privacy promise.

Expected tests/proof names:

- `family-web.no-child-activity-collection`
- `family-web.route-and-link-map`
- `family-web.privacy-copy-no-overclaim`
- `family-web.cloudflare-preview-build`
- `family-web.registration-handoff`

Proof artifact expectations:

- Build/deploy log when implementation exists.
- Route screenshot or rendered artifact for each public page.
- Link check report.
- Data collection review note.
- Explicit skipped-risk note for any page not yet implemented.
