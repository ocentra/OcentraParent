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

## Ownership boundary

```text
setup-install-provisioning-plan owns public route map, public/private data boundary, privacy wording, link map, and deployment blocker state.
account, package/distribution, custody, payment, portal shell, and child activity storage owners remain separate.
```

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

## Required proof fields

The selected proof must name, at minimum:

```text
route_map_state
data_collection_state
child_activity_boundary_state
privacy_copy_state
download_link_state
register_link_state
support_privacy_status_link_state
deploy_shape_state
preview_state
custom_domain_state
manual_required_state
no_deployed_site_claim
no_account_claim
no_installer_claim
no_setup_ready_claim
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

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
Workpack id and branch: WP01 Family Web Info Site / codex/tracking-plan-full-continuation-a
Public route/data-boundary changes: defined the public route map, no-private-activity boundary, data collection matrix, privacy copy rules, link map, and deploy blocker state for family.ocentra.ca.
Touched files: output/setup-install-provisioning-plan-proof/01-family-web-info-site/00-public-site-route-map-proof.md, output/setup-install-provisioning-plan-proof/01-family-web-info-site/01-no-private-activity-data-proof.md, output/setup-install-provisioning-plan-proof/01-family-web-info-site/02-data-collection-matrix.md, output/setup-install-provisioning-plan-proof/01-family-web-info-site/03-privacy-copy-no-overclaim-proof.md, output/setup-install-provisioning-plan-proof/01-family-web-info-site/04-link-accessibility-proof.md, output/setup-install-provisioning-plan-proof/01-family-web-info-site/05-deploy-preview-proof-or-blocker.md, output/setup-install-provisioning-plan-proof/01-family-web-info-site/16-validation-commands.log, docs/plans/setup-install-provisioning-plan/workpacks/01-family-web-info-site.md, docs/plans/setup-install-provisioning-plan/CHECKLIST_INDEX.md, docs/plans/setup-install-provisioning-plan/PLAN_STATE.md, docs/plans/setup-install-provisioning-plan/WORKPACK_INDEX.md
Validation commands and results: `node -e "console.log('family-web-info-site-docs-only')"` PASS; `npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan` PASS; `npm run build --workspace @ocentra-parent/production-domain` PASS; `npm run test --workspace @ocentra-parent/production-domain` PASS (56 files, 208 tests)
Proof artifacts: output/setup-install-provisioning-plan-proof/01-family-web-info-site/00-public-site-route-map-proof.md, output/setup-install-provisioning-plan-proof/01-family-web-info-site/01-no-private-activity-data-proof.md, output/setup-install-provisioning-plan-proof/01-family-web-info-site/02-data-collection-matrix.md, output/setup-install-provisioning-plan-proof/01-family-web-info-site/03-privacy-copy-no-overclaim-proof.md, output/setup-install-provisioning-plan-proof/01-family-web-info-site/04-link-accessibility-proof.md, output/setup-install-provisioning-plan-proof/01-family-web-info-site/05-deploy-preview-proof-or-blocker.md, output/setup-install-provisioning-plan-proof/01-family-web-info-site/16-validation-commands.log
Known gaps/manual-required states: no live family-site deployment proof, no custom-domain proof, no browser-level accessibility smoke, Cloudflare Pages vs Workers ownership still unresolved by the owning plans.
No-claim boundaries: no deployed site, no installer readiness, no account/session implementation, no child activity custody, no LAN or child-device implementation, no production-ready installer claim.
```
