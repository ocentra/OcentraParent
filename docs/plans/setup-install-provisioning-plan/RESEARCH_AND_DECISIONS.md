<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `Setup Install Research And Decisions`
> Kind: research-backed architecture decision map.
> Read when: WP01/WP03/WP04/WP06 touches web deployment, install journey, platform readiness, or rollout claims.
> Stop rule: do not browse again unless a provider/platform fact has changed or this doc names an open research question.
> Proves: research basis only.
> Does not prove: deployment, installer readiness, platform support, or setup completion.

<!-- /agent-capsule -->

# Setup Install Research And Decisions

## Current accepted direction

```text
family.ocentra.ca is a public family information/download/account-entry surface.
It does not collect private child activity data.
It routes registration/login to account identity.
It routes package/signing/update claims to runtime distribution plans.
It routes real pairing/protocol/device trust claims to LAN/device-trust plans.
This plan owns the visible journey and readiness state machine across those systems.
```

## External source anchors

- Cloudflare Pages can deploy full-stack applications to Cloudflare's network and supports Git provider deploys, direct upload, C3, Pages Functions, rollbacks, redirects, and custom domains: https://developers.cloudflare.com/pages/
- Cloudflare Workers static assets can host static application assets behind a Worker route when the shared Cloudflare worker/control-plane owns the public route: https://developers.cloudflare.com/workers/static-assets/
- Tauri updater, signing, packaging, and distribution behavior belong to the parent runtime distribution plan, not this setup journey plan: https://v2.tauri.app/distribute/updater/
- Android package/visibility and permission behaviors are platform-specific and must be proved by platform/runtime owners before Android child/parent install claims: https://developer.android.com/training/package-visibility
- macOS distribution claims require the runtime/package owner to handle signing/notarization and attach proof before product-ready wording: https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution

## Decision D01: public family site is not account/family authority

Accepted:

```text
family.ocentra.ca may explain, route, and hand off.
It may present login/register/download/support/privacy/status entry points.
It may accept account-entry data only through explicit account flows.
It must not collect private child activity data on public pages.
```

Rejected:

```text
public website owns household membership
public website owns child profile or child device registry
marketing analytics/contact capture silently acts as setup telemetry
website copy claims setup/product readiness without proof
```

## Decision D02: setup journey is state machine first

Required major states:

```text
publicInfo
accountEntry
accountReady
householdReady
parentBootstrapIssued
parentAppInstalled
childBootstrapIssued
childAgentInstalled
permissionsReady
pairingPending
pairingConfirmed
policyBaselineReady
dataCustodyStatusKnown
setupComplete
setupBlocked
manualRequired
```

No single state implies the next state.

## Decision D03: parent bootstrap and child bootstrap are separate

Parent bootstrap:

```text
parent account/household authority
parent bootstrap code/link
parent package selection
parent package download/install/launch
parent controller readiness
```

Child bootstrap:

```text
child profile/household authority
child pairing code/link/QR
child package selection
disclosure/consent
child package install/service start
permissions
signed readiness/hello
parent confirmation
```

These must not be merged into one download/install claim.

## Decision D04: platform state language is fixed

Allowed platform states:

```text
unsupported
planned
previewOnly
manualRequired
readyForTest
productionReady
blocked
```

Production-ready requires owning plan proof for packaging, signing, update/rollback, permissions, support, privacy/legal, and negative cases.

## Decision D05: first-run UI must show incomplete states honestly

The UI must visibly show:

```text
live local
LAN
parent cache
parent-owned storage
stale
degraded
unavailable
manual-required
unsupported
blocked
```

Do not hide missing setup pieces behind a green check.

## Decision D06: proof manifest is a product safety gate

Every rollout claim needs:

```text
proof artifact
command log
screenshot or route-state artifact when UI changes
negative/degraded/manual-required cases
adjacent owner handoff
no-claim boundaries
```

## Open research questions

- Final public family-site hosting choice remains open until Cloudflare-control-plane and production-distribution plans decide Pages vs Worker static-assets ownership.
- Parent runtime installer/update channel is owned by parent-client-runtime-distribution-plan.
- Child runtime installer/permission path is owned by child-agent-runtime-distribution-plan and app-plan.
- Real pairing proof remains owned by LAN/device-trust work.
