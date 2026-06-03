# Production Distribution And Support

## Parent Outcome

Parents can install, update, uninstall, subscribe, get support, understand
privacy/data handling, and trust that platform-specific releases are real, not
scaffold artifacts.

## Ocentra Requirement

Production distribution is part of the product. A CI package preview is not a
shipping claim. Signed installers, app stores, update channels, support docs,
privacy/legal docs, billing, and release proof must be explicit.

## Roadmap And Expectations

- Roadmap: V7 subscription and monetization, V8 production hardening.
- Expectations: [release installer](../expectations/release-installer.md),
  [billing](../expectations/billing.md),
  [platform deliverables](../expectations/platform-deliverables.md),
  [static analysis/security](../expectations/static-analysis-security.md),
  [documentation](../expectations/documentation.md).
- Modules: `crates/agent-updater`, `platforms/android`, `platforms/ios`,
  release scripts, root README, public website/account surface.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
production distribution, remote parent access, billing/subscription, and mobile
coverage.

All mature competitors ship through app stores, installers, support surfaces,
billing, privacy documents, and update channels. Ocentra must not look like a
dev-only repo when it claims consumer readiness.

## Current Ocentra State

- Windows MSI/updater scaffolding exists.
- Cross-platform package previews exist as CI mechanics.
- Parent desktop Tauri package proof exposes built-portal frontend state,
  Rust-service backend kind, package service-manager launch ownership, service
  health endpoint, runtime readiness, fixed port/process ownership,
  connect-or-degrade behavior, route/source/custody labels, support redaction,
  preview/manual-required release states, platform-matrix split rows, and
  blank-window guard state without treating Vite as a packaged backend.
- Windows package lifecycle proof installs/starts the service wrapper and probes
  the Rust service health endpoint; this remains CI/mechanical proof, not signed
  release proof.
- Parent desktop release-support proof now records update/rollback posture,
  signing/store claim boundaries, package-runtime evidence, support diagnostic
  redaction fields, CI artifact expectations, production-support incident
  handoff metadata, parent consent, disclosed support-bundle data classes,
  support-safe diagnostic references, and manual platform proof requirements.
  The typed proof requires built portal dist, Rust-service boundary, package
  service-manager launch ownership, fixed loopback process ownership,
  connect-or-degrade behavior, signed-channel update posture, and explicit
  not-signing/not-production/non-upload non-claims.
- `mobile-child-agent-capability-proof` adds package/runtime hook evidence for
  Android debug APK/checksum, Android package-local status, Android device
  install/manual Play signing, iOS Xcode target, iOS simulator status, iOS
  signing, and iOS TestFlight/device proof without promoting any store or
  signing claim.
- Parent mobile route-status/service-bridge proof now records Android parent
  mobile, iOS parent mobile, Android child agent, and iOS child agent as
  separate claim boundaries. It proves typed local-service, LAN-service,
  cloud-relay, parent-cache, parent-owned-storage, mobile-package, observer
  read-only, controller-takeover manual-required, LAN AI provider degraded or
  unavailable, phone-local-model disabled, and package/signing/store
  manual-required states without promoting mobile controller authority or child
  mobile parity.
- `billing-account-endpoint-contract-proof` defines endpoint-domain route ids,
  API paths, headers, query params, and contract-version labels for account
  status, plan/entitlement snapshot, subscription status, device-limit decision,
  and account download/update/status surfaces. It is route contract proof only:
  no Stripe SDK, billing provider backend, account backend, portal UI, updater
  runtime, or child-activity custody is implemented.
- `billing-entitlement-contract-proof` defines parent-domain contracts for
  plan entitlement rows, entitlement snapshots, subscription status sync events,
  device-limit decisions, parent-visible failure states, local-safety fallback,
  evidence-export retention, and explicit billing non-claims. It is contract
  proof only: no Stripe SDK, billing provider backend, account backend,
  entitlement signing runtime, portal UI, child-device consumption, or
  child-activity custody is implemented.
- `billing-subscription-device-limit-failure-proof` extends that contract proof
  with subscription status proof rows, over-limit device activation denial,
  trusted existing-device grace/manual states, all current billing failure kinds,
  retained evidence-export access, parent-visible resolution labels, and
  existing-local-safety continuation. It remains contract proof only and does
  not add provider, backend, portal, or child-device runtime code.
- Billing/support/public website/store distribution are planned or incomplete.

## Current Gap

Need production signing, release channels, app store paths, Play/TestFlight,
support docs, privacy/legal docs, billing entitlement flows, update rollback,
public download/account/status surfaces, production support workflows, and real
signed installer/update-channel/store evidence beyond CI-mechanical
package/runtime proof.

## Checklist

- [ ] Windows signing and installer release proof.
- [ ] Parent desktop package runtime proof tied to installer/update smoke.
      Current `parent-desktop-release-support-proof` covers typed package
      runtime evidence and CI/manual-required release posture; signed
      installer/update smoke remains a production gap.
- [ ] Updater channel, rollback, and failure status.
- [ ] macOS signing/notarization if shipped.
- [ ] Android Play signing/store proof if shipped. Current
      `mobile-child-agent-capability-proof` row keeps Play signing planned and
      device install manual-required.
- [ ] Android parent mobile route-status/service bridge proof. Current
      `parent-mobile-service-bridge-proof` and
      `parent-mobile-controller-observer-handoff-proof` keep Android parent
      mobile observer/request-first, controller authority manual-required,
      parent cache stale, parent-owned storage offline, and child-agent parity
      unclaimed.
- [ ] iOS TestFlight/App Store proof if shipped. Current
      `mobile-child-agent-capability-proof` row keeps signing-required,
      TestFlight/device proof manual-required, and App Store planned.
- [ ] iOS parent mobile route-status/service bridge proof. Current
      `parent-mobile-service-bridge-proof` and
      `parent-mobile-controller-observer-handoff-proof` keep iOS parent mobile
      controller-candidate behavior manual-required, LAN/provider routing
      unavailable, parent cache stale, parent-owned storage offline, and
      entitlement/TestFlight/device proof unclaimed.
- [ ] Billing/subscription/account flow. Current
      `billing-account-endpoint-contract-proof` covers endpoint contracts for
      account status, entitlement snapshot, subscription status, device-limit
      decision, and download/update/status surfaces only; billing provider
      integration, account backend, entitlement runtime, and UI remain
      incomplete. Current `billing-entitlement-contract-proof` covers typed plan,
      entitlement snapshot, subscription sync, device-limit decision, failure
      behavior, evidence-export retention, and no-safety-shutdown contract proof
      only; provider integration, backend storage/signing, runtime delivery,
      portal UI, and child-device consumption remain incomplete.
      `billing-subscription-device-limit-failure-proof` adds subscription
      status proof rows, over-limit device-denial rules, existing-device
      grace/manual paths, and all current billing failure kinds while keeping
      provider/runtime work unimplemented.
- [ ] Privacy, retention, export/delete, and support docs.
- [ ] Support bundle redaction proof scaffold and incident process. Current
      release-support proof requires parent consent, incident/status metadata,
      visible support-bundle data-class disclosure, support-safe diagnostic
      references, manual-required production support states, and blocks tokens,
      child activity, raw URLs, screenshots, journals, SQLite snapshots, private
      paths, commands, keystrokes, clipboard data, and message contents from
      support output. Real support backend upload, billing/account escalation,
      public account lookup, remote support, and production SLA remain
      unimplemented/manual-required.

## Next AI Instructions

Do not promote scaffold package previews to product support. Every release claim
must name platform, signing/store state, smoke proof, support path, and known
limitations. The product checklist now records this boundary: parent desktop
package runtime release-support proof is preview/mechanical evidence only, not
signing, store, notarization, Play, TestFlight, production updater rollback, or
child mobile parity proof.
