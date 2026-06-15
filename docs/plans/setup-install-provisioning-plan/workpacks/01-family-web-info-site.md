<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `WP01 Family Web Info Site`
> Kind: assigned implementation/research workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: do not implement account logic, installer build mechanics, or child activity storage here.
> Proves: public family-site route/data-boundary shape only after proof artifacts exist.
> Does not prove: deployed site, registration readiness, installer readiness, or setup readiness.
> Proof rule: before DONE, write all WP01 proof artifacts and command log.

<!-- /agent-capsule -->

# WP01 Family Web Info Site

## Goal

Define `family.ocentra.ca` as the public family product entry surface with honest privacy, download, support, status, and account-entry routing.

## Required inputs

```text
RESEARCH_AND_DECISIONS.md
docs/expectations/family-setup.md
docs/expectations/release-installer.md
docs/expectations/data-custody.md
docs/roadmaps/roadmap-v1-0-local-mvp.md
docs/roadmaps/roadmap-v8-production-hardening.md
```

## Owned scope

```text
public page map
public data collection boundary
privacy/no-overclaim wording
download/register/support/privacy/status entry map
Cloudflare Pages or Workers static-assets deployment shape
preview/custom-domain/manual-required state
```

## Out of scope

```text
account/session implementation
installer package build/signing/update
child activity storage
portal shell internals
payment checkout
```

## Expected output

```text
home route
download route
register/login entry route
privacy route
support route
status route
install help route
invite/code entry route state
public data collection matrix
Cloudflare deploy shape decision or blocker
```

## Required proof root

```text
output/setup-install-provisioning-plan-proof/01-family-web-info-site/
```

Required artifacts:

```text
00-public-site-route-map-proof.md
01-no-private-activity-data-proof.md
02-data-collection-matrix.md
03-privacy-copy-no-overclaim-proof.md
04-link-accessibility-proof.md
05-deploy-preview-proof-or-blocker.md
16-validation-commands.log
```

## Acceptance criteria

- [ ] Public page map exists.
- [ ] Public data collection matrix exists.
- [ ] Private child activity data is explicitly forbidden on public pages.
- [ ] Privacy copy avoids vague or unproven claims.
- [ ] Register/login and download are handoff links, not owned runtime flows.
- [ ] Support/privacy/status links are defined.
- [ ] Cloudflare Pages or Workers static-assets deploy shape is selected or blocked.
- [ ] Preview/custom-domain state is explicit.
- [ ] Focused commands pass or blocker recorded.

## Focused commands

```bash
node -e "console.log('family-web-info-site-docs-only')"
npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan
```

If a site package exists later:

```bash
npm run build --workspace @ocentra-parent/family-site
npm run test --workspace @ocentra-parent/family-site
```

## Negative states

- Public site collects private activity data.
- Download button implies installer is production-ready without package proof.
- Register button implies household/device authority is implemented without account proof.
- Privacy copy says “nothing is stored” without a data-custody proof.
- Custom domain is claimed live without deployment proof.

## Manual-required gaps

Deployment, custom domain, analytics, account handoff, and downloads remain manual-required until the owning proof exists.

## Fill before DONE

```text
Workpack id and branch:
Public route/data-boundary changes:
Touched files:
Validation commands and results:
Proof artifacts:
Known gaps/manual-required states:
No-claim boundaries:
```
