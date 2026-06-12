<!-- agent-capsule -->

> Agent Capsule
> Doc: Browser Control Coverage Matrix
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Browser Control Coverage Matrix

Status: worker D coverage record for PR #132 draft follow-up.

This matrix reconciles `docs/browser-control-schema-proposal.md` with
`docs/browser-policy-settings-catalog.md`. It is not a UI spec for C to invent
new questions from. C should render from the typed authoring manifest and use
the typed update protocol.

## Source Additions In This Slice

- `browser.executionMode` writes to `/browserPolicy/executionMode` and covers
  observe, dry-run, warn/ask, and enforce mode.
- `discovery.scanInstalledBrowsers` writes to
  `/browserPolicy/discovery/scanInstalledBrowsers`.
- `discovery.scanRunningBrowsers` writes to
  `/browserPolicy/discovery/scanRunningBrowsers`.
- `discovery.detectUnmanagedBrowsers` writes to
  `/browserPolicy/discovery/detectUnmanagedBrowsers`.
- Managed setup and provisioning stay represented through managed-browser
  intent plus capability/manual-required state. This branch does not claim that
  install, repair, native host provisioning, platform app control, router,
  firewall, store policy, or mobile device-owner controls are implemented.

## Candidate MVP Coverage

| Candidate item                                                                      | Coverage                                                                                                                                                |
| ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Enable browser controls                                                             | Implemented manifest control: `browser.enabled`                                                                                                         |
| Mode: observe, dry-run, warn/ask, enforce                                           | Implemented manifest control: `browser.executionMode` plus `browser.defaultPosture`                                                                     |
| Require managed browser for exact web rules                                         | Implemented manifest controls: `managedBrowser.mode`, `evidence.requiredProof`; runtime still rejects dishonest exact-URL proof                         |
| Scan installed browsers                                                             | Implemented manifest/control path: `discovery.scanInstalledBrowsers`; adapter proof remains capability-gated                                            |
| Scan running browsers                                                               | Implemented manifest/control path: `discovery.scanRunningBrowsers`; adapter proof remains capability-gated                                              |
| Detect unmanaged browsers                                                           | Implemented manifest/control path: `discovery.detectUnmanagedBrowsers` plus unmanaged classification targets                                            |
| Allow managed browser                                                               | Implemented through `managedBrowser.mode` and `managedBrowser.allowedFamilies`                                                                          |
| Launch or repair managed browser setup                                              | Represented through `managedBrowser.launchMode` and capability/manual-required state; no fake repair implementation                                     |
| Allow URL/domain/title evidence from managed browser                                | Implemented through `evidence.urlScope` and `evidence.requiredProof`                                                                                    |
| Redact query strings                                                                | Implemented through `evidence.urlScope` values such as `full-url-without-query`                                                                         |
| Keep exact URL evidence for selected retention                                      | Implemented through `retention.exactUrl`                                                                                                                |
| Allow unmanaged browser: monitor, warn, ask, relaunch, block                        | Implemented through `unmanagedBrowser.mode`                                                                                                             |
| Choose covered browsers: Edge, Chrome, Chrome for Testing, unsupported as unmanaged | Implemented through `managedBrowser.allowedFamilies` and unmanaged classification targets                                                               |
| Rule targets                                                                        | Represented through `rules.allowedTargetTypes` and `rules.items`                                                                                        |
| Rule actions                                                                        | Implemented through `rules.allowedActions` and rule action plans                                                                                        |
| Time budgets                                                                        | Daily and counting mode are direct controls; session/site/domain/blackout remain nested rule/schedule/budget shapes for a future richer authoring slice |
| Parent approvals                                                                    | Implemented through `approvals.requiredFor` and `approvals.unansweredDefault`                                                                           |
| Reports                                                                             | Implemented through `reports.visibleFields`                                                                                                             |
| Proof requirement                                                                   | Implemented through `evidence.requiredProof`, `evidence.whenProofUnavailable`, and capability state                                                     |
| Data custody                                                                        | Implemented through `custody.allowedUses`                                                                                                               |
| Audit                                                                               | Implemented through `audit.requiredFields`; audit state remains runtime policy                                                                          |

## Catalog Major Section Coverage

| Catalog section                                   | Coverage                                                                                                                 |
| ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| How To Read This Catalog                          | Documentation-only; represented by the matrix status vocabulary                                                          |
| Global Rule Dimensions                            | Represented through structured rule targets, schedule IDs, budget IDs, priorities, and future policy scopes              |
| Master Browser Control Settings                   | Implemented by `browser.enabled`, `browser.executionMode`, and `browser.defaultPosture`                                  |
| Browser Discovery Settings                        | Implemented by the three discovery intent controls; real OS scan support remains capability-gated                        |
| Browser Coverage Settings                         | Implemented by managed browser families and unmanaged classification targets                                             |
| Managed Browser Setup Settings                    | Represented by managed-browser intent and manual-required capability state; install/repair/provisioning not claimed      |
| Managed Browser Operation Settings                | Represented by launch mode and bridge requirements; process control is adapter-gated                                     |
| Unmanaged Browser Handling Settings               | Implemented by unmanaged mode, grace seconds, relaunch URL, and classification targets                                   |
| URL And Tab Evidence Settings                     | Implemented by evidence URL scope, proof level, proof fallback, and never-collect controls                               |
| Rule Target Settings                              | Represented by allowed target types and rule items                                                                       |
| Rule Action Settings                              | Implemented by allowed actions and rule action plans; close/redirect actions remain adapter-gated                        |
| Observe Versus Enforce Settings                   | Implemented by `browser.executionMode` and default posture                                                               |
| Schedule Settings                                 | Represented by schedules and rule schedule IDs; not expanded into one preset per field                                   |
| Time Budget Settings                              | Direct daily/counting controls plus nested budget references; richer per-site/session controls are future authoring work |
| Parent Approval Settings                          | Implemented by approval triggers and unanswered defaults                                                                 |
| Override Settings                                 | Represented by approval and rule precedence; temporary override storage is future protocol work                          |
| Downloads Settings                                | Implemented by download mode and blocked types; interception remains capability-gated                                    |
| Search Settings                                   | Represented by search target type and never-collect restrictions                                                         |
| Video And Channel Settings                        | Represented by video-channel target type                                                                                 |
| Private, Incognito, Tor, And Anti-Bypass Settings | Represented by unmanaged classification and bridge requirements; hard blocking needs adapter proof                       |
| Network And Domain Fallback Settings              | Represented by proof fallback and degraded capability state                                                              |
| Browser App And Process Settings                  | Represented by browser-process targets and counting mode                                                                 |
| Child-Facing Experience Settings                  | Typed in `childFacing`; C owns visual rendering                                                                          |
| Parent Report Settings                            | Implemented by report visible fields                                                                                     |
| Portal Display Settings                           | Represented by report visible fields plus capability state; C owns display                                               |
| Portal Action Settings                            | Represented by typed get, preview, patch, replace, and rollback commands                                                 |
| Portal AI Settings                                | Typed in `portalAi`; no raw browser content claim                                                                        |
| Data Source And Custody Settings                  | Implemented by custody allowed uses                                                                                      |
| Retention Settings                                | Implemented by exact URL retention; generic retention state remains runtime policy                                       |
| Audit Settings                                    | Implemented by audit required fields; audit state remains runtime policy                                                 |
| Capability Failure Settings                       | Represented by fallbacks and capability state                                                                            |
| Conflict Resolution Settings                      | Represented by rule priority and stale revision checks                                                                   |
| Local AI Browser Settings                         | Represented by Portal AI settings and evidence references; raw content stays blocked unless explicitly reviewed          |
| Never-Collect Settings                            | Implemented by evidence never-collect controls                                                                           |
| Platform Settings                                 | Represented by platform capability state; OS/device-owner/native-host work remains manual or future                      |
| Setup And Provisioning Settings                   | Represented by managed-browser intent plus manual-required capability state                                              |
| Notifications And Escalation Settings             | Approval triggering is covered; notification delivery is future non-browser-control work                                 |
| Gaps To Decide Before UI Contracts                | Tracked here so UI does not add arbitrary catalog questions                                                              |
