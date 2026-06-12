<!-- agent-capsule -->

> Agent Capsule
> Doc: Competitor Capability Map
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Competitor Capability Map

This map records what serious parental-control products expose to parents and
how Ocentra tracks the corresponding product requirement. It exists to stop
hand-wavy comparisons. If a competitor has a capability, Ocentra must either
build it, explicitly reject it, or track it as a gap.

This is not marketing copy. The product-facing summary belongs in
[README.md](../README.md). The requirement contract belongs in
[product-constitution.md](product-constitution.md) and
[product-capability-checklist.md](product-capability-checklist.md).
Per-feature follow-up docs are indexed in [feature-list.md](feature-list.md).

## Source Baseline

Use official product or support sources before updating this map:

- [Google Family Link parental controls](https://support.google.com/families/answer/15077835?hl=en)
- [Apple Screen Time parental controls](https://support.apple.com/en-us/108806)
- [Microsoft Family Safety screen time](https://support.microsoft.com/en-US/family-safety/set-screen-time-limits-across-devices)
- [Microsoft Family Safety app blocking](https://support.microsoft.com/en-US/family-safety/block-or-unblock-apps-with-microsoft-family-safety)
- [Bark parental control app](https://www.bark.us/learn/the-app/)
- [Qustodio features](https://www.qustodio.com/en/features/)
- [Norton Family](https://us.norton.com/products/norton-family)
- [Net Nanny features](https://www.netnanny.com/features/)
- [Kaspersky Safe Kids features](https://usa.kaspersky.com/safe-kids/features)
- [Canopy](https://canopy.us/)
- [Kidslox statistics](https://kidsloxsupport.zendesk.com/hc/en-us/articles/115003904334-What-kind-of-statistics-does-Kidslox-provide)
- [FamilyTime features](https://familytime.io/features/)
- [FamiSafe](https://famisafe.wondershare.com/)

Refresh the links when the product positioning changes or before using this map
for public claims.

## Capability Comparison

| Capability                         | Competitor baseline                                                                                                          | Ocentra requirement                                                                                                                                | Current Ocentra status                                                                              | Gap policy                                                                                          |
| ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| Family setup and child profiles    | Google, Apple, Microsoft, Bark, Qustodio, and others center setup around family groups, child profiles, and parent apps.     | Parent account, household, child profile, child device, parent-controller, parent-observer, and recovery states.                                   | In progress through contracts and role/read-model proof.                                            | Must be a first-run product flow before consumer release.                                           |
| Co-parent and observer roles       | Platform ecosystems support family organizers/guardians or supervised roles.                                                 | One active household controller lease, observers read-only, revocation and stale/replayed command rejection.                                       | In progress in LAN/controller proof.                                                                | Finish parent portal role UI and remote/LAN route behavior.                                         |
| App inventory                      | Competitors show apps and usage summaries.                                                                                   | App/process/game inventory, package identity, category, running and foreground time, evidence refs.                                                | In progress through app/game evidence and package identity proof.                                   | Keep inventory distinct from enforcement; finish parent-visible catalog and unknown-state handling. |
| App block and app limits           | Google, Apple, Microsoft, Qustodio, Norton, Net Nanny, Bark, Kidslox, and others support app limits or blocking by platform. | Typed parent rule, evidence-backed policy decision, platform adapter, timer/recovery, audit, child-facing explanation.                             | In progress for Windows owned-process time-limit proof; broad platform blocking remains incomplete. | Must prove each platform adapter before claiming parity.                                            |
| Install approval and purchases     | Google Play and Apple ecosystems support approval or content restriction paths.                                              | Store/install approval contract, platform capability status, parent approval audit, fallback where the OS does not allow it.                       | Gap.                                                                                                | Build platform-specific request/approval proof and source metadata.                                 |
| Screen time schedules              | All major competitors expose daily limits, downtime, school time, or bedtime modes.                                          | Policy schedules, time budgets, grace periods, bonus time, ask-parent, override, and timer recovery.                                               | Partial through policy/enforcement contracts and timer proof.                                       | Needs complete parent UI, child request flow, and runtime recovery proof.                           |
| Web filtering and categories       | Google, Apple, Microsoft, Qustodio, Norton, Net Nanny, Canopy, Bark, and others expose content filters.                      | Managed browser URL evidence, domain/network summaries, parent categories, AI-assisted classification, typed policy, adapter-specific enforcement. | In progress for URL/tab evidence and browser/domain adapter proof; broad filtering incomplete.      | Must separate managed-browser, unmanaged-browser, DNS/network, and screen-derived claims.           |
| Search and platform restrictions   | Google, Apple, and Microsoft expose ecosystem-specific controls.                                                             | Platform-specific restriction contracts and honest unavailable/manual-required states.                                                             | Planned/gap.                                                                                        | Track per ecosystem instead of pretending generic parity.                                           |
| Video safety                       | Google has YouTube controls; social-focused tools monitor video/account contexts.                                            | URL/video evidence, local AI or local screen/video summary, confidence, parent rule, decision, enforcement/audit.                                  | Partial architecture; not product-complete.                                                         | Prove actual video input, model path, confidence, decision, and parent explanation before claiming. |
| Social app controls                | Bark, Qustodio, FamiSafe, FamilyTime, and others emphasize social monitoring or app/social controls.                         | Social app category rules, session evidence, message/content source contracts where permitted, alerts, and privacy settings.                       | Gap/in progress only as app/category policy intent.                                                 | Implement first-class social contracts, sources, privacy settings, and alerts.                      |
| Message/content monitoring         | Bark and some competitors alert on messages, photos, videos, calls, or SMS depending on platform.                            | Explicit source permissions, local-only analysis where possible, parent-visible risk alerts, no hidden collection.                                 | Gap.                                                                                                | Decide platform-by-platform scope; require privacy/legal review before implementation.              |
| Location, geofence, SOS, battery   | Google, Apple, Microsoft, Qustodio, Bark, FamilyTime, FamiSafe, Kaspersky, and others expose location features.              | Location evidence, geofence rules, last-known location, device/battery state, SOS/check-in, custody and retention settings.                        | Capability guide exists; runtime product gap.                                                       | Build runtime contracts, platform permission proof, retention, alerts, and UI.                      |
| Notifications and alerts           | Mature competitors send alerts and summaries.                                                                                | Minimal-detail notification payloads, authenticated drill-in, delivery status, quiet hours, escalation, privacy boundary.                          | Planned for V3.                                                                                     | Must implement provider boundary and parent-visible delivery status.                                |
| Reports and digests                | Competitors expose daily/weekly summaries and activity history.                                                              | Parent-owned reports with evidence references, trend summaries, assistant Q&A, and storage/export status.                                          | In progress through Activity report persistence/family fanout/MIA context proof.                    | Finish parent-facing reports, history, and cited assistant answers.                                 |
| Remote parent access               | Most competitors support parent app or web access away from home.                                                            | LAN first, optional relay, route status, revocation, parent-owned storage, stale/offline states, no default Ocentra evidence custody.              | Planned/in progress through V0.9 LAN and V2 relay docs.                                             | V2 must close remote-control parity without violating data custody.                                 |
| Screenshots or live screen         | Some competitors provide screenshots or visibility features.                                                                 | Local screen-analysis summaries by default; raw image retention only by explicit parent setting and local custody.                                 | Partial local screen queue; live screenshot product is not complete.                                | Decide whether to compete directly or document privacy tradeoff.                                    |
| Tamper/uninstall resistance        | Platform ecosystems and device-management products provide some controls.                                                    | Service integrity, uninstall alerts, device-owner/managed profile or entitlement proof, visible degraded status.                                   | Gap/scaffold-only.                                                                                  | Needs security design; no stealth or privilege escalation claims.                                   |
| Multi-device household             | Competitors expect multiple child devices and parent devices.                                                                | Device roles, pairing, trusted registry, route selection, controller lease, observers, stale/offline states.                                       | In progress through V0.9 proof gates.                                                               | Needs physical household proof and parent UI.                                                       |
| AI assistant for setup and control | Competitors vary; most do not expose a full local policy operator.                                                           | Parent can ask AI to set schedules, explain decisions, draft rules, preview actions, and tune policies from evidence.                              | In progress through local AI, parent assistant contracts, and MIA context proof.                    | Finish parent portal chat/action flow and action execution boundary.                                |
| Local-first privacy                | Competitors are usually cloud or ecosystem-account centered.                                                                 | Child evidence local by default, parent-owned storage optional, cloud only for account/relay/notification/status boundaries.                       | Core product requirement; many runtime paths in progress.                                           | Every remote feature must preserve custody labels and opt-in paths.                                 |
| Production distribution            | Competitors ship app stores, installers, support, and billing.                                                               | Signed installers, store paths, update channel, support/legal/privacy docs, package proof, billing/entitlements.                                   | Scaffold/in progress.                                                                               | Required before external users.                                                                     |

## Product Response

The response to this map is not to copy every competitor behavior blindly. The
response is to make every choice explicit.

Required response for each capability:

- `build`: roadmap milestone, expectation docs, module owner, validation gate;
- `reject`: parent-facing reason, privacy/security/platform rationale, support
  answer;
- `track`: status, blocker, next proof, and owner.

## Current Highest-Risk Gaps

1. Parent product completion: setup, child profiles, rule authoring, schedules,
   approvals, reports, notifications, and AI actions must feel like one product.
2. Mobile child-agent proof: Android and iOS claims need real device,
   permission, entitlement, store/signing, and runtime evidence.
3. Social/message/video productization: the intent exists, but monitoring,
   privacy boundaries, platform permissions, and parent alerts need first-class
   docs and contracts.
4. Location/geofence parity: many parents expect this; Ocentra must build it or
   explicitly position away from it.
5. Remote away-from-home control: local-first cannot mean useful only at home.
   V2 must provide a parent-owned or minimal-custody route.
6. Production distribution and support: installers, stores, updates, privacy
   docs, and support are part of the product, not polish.
