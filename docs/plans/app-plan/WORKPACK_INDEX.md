<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-plan`
> Doc: `WORKPACK_INDEX.md`
> Kind: workpack chooser with code-first Phase 1 state.
> Read when: After `PLAN_STATE.md` and `CODE_AUDIT.md`.
> Stop rule: Open only the selected workpack.
> Proves: routing and audited Phase 1 state; not Phase 2/3 acceptance.

<!-- /agent-capsule -->

# Native Apps Plan Workpack Index

Audit date: 2026-08-15. All 95 imported workpacks have reviewed code/test
topology. `Complete` means no source/test-writing gap in the workpack's bounded
scope; it does not mean tests, Enforcer, proof, CI, or product acceptance are
complete. See [CODE_AUDIT.md](CODE_AUDIT.md) for exact evidence and gaps.

| Workpack | Phase 1 code/test state |
| --- | --- |
| [01 contract boundary and effect schemas](workpacks/01-contract-boundary-and-effect-schemas.md) | Complete |
| [02 source index and doc reconciliation](workpacks/02-source-index-and-doc-reconciliation.md) | Complete, docs-only |
| [03 current app snapshot and gap map](workpacks/03-current-app-snapshot-and-gap-map.md) | Complete, docs-only |
| [04 app identity model](workpacks/04-app-identity-model.md) | Complete |
| [05 installed app inventory model](workpacks/05-installed-app-inventory-model.md) | Complete |
| [06 windows installed app inventory adapter](workpacks/06-windows-installed-app-inventory-adapter.md) | Complete |
| [07 windows store uwp appx inventory adapter](workpacks/07-windows-store-uwp-appx-inventory-adapter.md) | Complete |
| [08 windows process runtime evidence adapter](workpacks/08-windows-process-runtime-evidence-adapter.md) | Complete |
| [09 windows foreground app evidence adapter](workpacks/09-windows-foreground-app-evidence-adapter.md) | Complete |
| [10 cross platform authority matrix](workpacks/10-cross-platform-authority-matrix.md) | Complete |
| [11 app category and risk taxonomy](workpacks/11-app-category-and-risk-taxonomy.md) | Complete |
| [12 app sessionization and duration engine](workpacks/12-app-sessionization-and-duration-engine.md) | Complete |
| [13 journal and sqlite app ingest](workpacks/13-journal-and-sqlite-app-ingest.md) | Complete |
| [14 app read models and service events](workpacks/14-app-read-models-and-service-events.md) | Complete |
| [15 parent portal app inventory running session surfaces](workpacks/15-parent-portal-app-inventory-running-session-surfaces.md) | **Incomplete** |
| [16 new app and unknown app approval flow](workpacks/16-new-app-and-unknown-app-approval-flow.md) | **Incomplete** |
| [17 risk app detection](workpacks/17-risk-app-detection.md) | **Incomplete** |
| [18 policy target compiler for app rules](workpacks/18-policy-target-compiler-for-app-rules.md) | **Incomplete** |
| [19 time budget schedule bonus time integration](workpacks/19-time-budget-schedule-bonus-time-integration.md) | **Incomplete** |
| [20 child facing app warning block request ux](workpacks/20-child-facing-app-warning-block-request-ux.md) | **Incomplete** |
| [21 windows owned process terminate time limit proof](workpacks/21-windows-owned-process-terminate-time-limit-proof.md) | Complete |
| [22 broad blocking proof gates](workpacks/22-broad-blocking-proof-gates.md) | Complete, coordination-only |
| [23 app ai classifier digest boundary](workpacks/23-app-ai-classifier-digest-boundary.md) | Complete |
| [24 platform extension checklist and proof routing](workpacks/24-platform-extension-checklist-and-proof-routing.md) | Complete, coordination-only |
| [25 install and uninstall approval handoff](workpacks/25-install-and-uninstall-approval-handoff.md) | Complete, bounded handoff |
| [26 performance and service health](workpacks/26-performance-and-service-health.md) | **Incomplete, tests-only harness gap; no production-source gap** |
| [27 e2e and manual proof artifacts](workpacks/27-e2e-and-manual-proof-artifacts.md) | Complete, proof-routing only |
| [28 rollout checklist and pr gate](workpacks/28-rollout-checklist-and-pr-gate.md) | Complete, coordination-only |
| [29 rust protocol evidence identity parity](workpacks/29-rust-protocol-evidence-identity-parity.md) | Complete |
| [30 rust protocol authority classifier parity](workpacks/30-rust-protocol-authority-classifier-parity.md) | Complete |
| [31 journal sqlite authority classifier storage](workpacks/31-journal-sqlite-authority-classifier-storage.md) | Complete |
| [32 live process snapshot source](workpacks/32-live-process-snapshot-source.md) | Complete |
| [33 live process journal sqlite bridge](workpacks/33-live-process-journal-sqlite-bridge.md) | Complete |
| [34 service capture app game live process bridge](workpacks/34-service-capture-app-game-live-process-bridge.md) | Complete |
| [35 service app game recurring freshness](workpacks/35-service-app-game-recurring-freshness.md) | Complete |
| [36 live foreground window source](workpacks/36-live-foreground-window-source.md) | Complete |
| [37 service foreground capture bridge](workpacks/37-service-foreground-capture-bridge.md) | Complete |
| [38 service authority classifier surface evidence](workpacks/38-service-authority-classifier-surface-evidence.md) | Complete |
| [39 authority classifier read model counts](workpacks/39-authority-classifier-read-model-counts.md) | Complete |
| [40 app game boundary read model event](workpacks/40-app-game-boundary-read-model-event.md) | Complete |
| [41 live windows inventory source](workpacks/41-live-windows-inventory-source.md) | Complete |
| [42 service windows inventory capture bridge](workpacks/42-service-windows-inventory-capture-bridge.md) | Complete |
| [43 live windows store package source](workpacks/43-live-windows-store-package-source.md) | Complete |
| [44 service windows store package capture bridge](workpacks/44-service-windows-store-package-capture-bridge.md) | Complete |
| [45 live windows registry inventory source](workpacks/45-live-windows-registry-inventory-source.md) | Complete |
| [46 service windows registry capture bridge](workpacks/46-service-windows-registry-capture-bridge.md) | Complete |
| [47 backend source freshness read model](workpacks/47-backend-source-freshness-read-model.md) | Complete |
| [48 portal source freshness surface](workpacks/48-portal-source-freshness-surface.md) | **Incomplete** |
| [49 category risk policy routing](workpacks/49-category-risk-policy-routing.md) | **Incomplete** |
| [53 notification intent contract](workpacks/53-notification-intent-contract.md) | Complete |
| [54 policy readiness portal renderer](workpacks/54-policy-readiness-portal-renderer.md) | Complete |
| [56 notification service read model](workpacks/56-notification-service-read-model.md) | Complete |
| [58 notification local outbox bridge](workpacks/58-notification-local-outbox-bridge.md) | **Complete for bounded Phase 1; Phase 2 passed; proof open** |
| [59 notification scheduler bridge](workpacks/59-notification-scheduler-bridge.md) | **Bounded Phase 1 complete; focused Phase 2 green; proof open** |
| [60 notification audit history bridge](workpacks/60-notification-audit-history-bridge.md) | **Bounded Phase 1 complete; focused Phase 2 green; proof open** |
| [61 notification provider preflight](workpacks/61-notification-provider-preflight.md) | **Bounded Phase 1 complete; focused Phase 2 green; proof open** |
| [62 notification preference preflight](workpacks/62-notification-preference-preflight.md) | **Bounded Phase 1 source/test reviewed; focused execution and proof open** |
| [63 source freshness source panel polish](workpacks/63-source-freshness-source-panel-polish.md) | **Incomplete** |
| [64 notification provider status handoff](workpacks/64-notification-provider-status-handoff.md) | **Incomplete** |
| [65 notification preference status handoff](workpacks/65-notification-preference-status-handoff.md) | **Incomplete** |
| [66 notification parent surface intent](workpacks/66-notification-parent-surface-intent.md) | **Production code drafted; expected tests/validation deferred** |
| [67 notification parent surface renderer](workpacks/67-notification-parent-surface-renderer.md) | **Production code drafted; expected tests/validation deferred** |
| [74 source freshness policy consumption](workpacks/74-source-freshness-policy-consumption.md) | Complete, bounded contract |
| [75 source freshness preview gate](workpacks/75-source-freshness-preview-gate.md) | Complete, bounded projection |
| [76 source gated policy preview read model](workpacks/76-source-gated-policy-preview-read-model.md) | Complete, bounded projection |
| [78 source gated policy preview timer handoff](workpacks/78-source-gated-policy-preview-timer-handoff.md) | Complete, bounded projection |
| [79 source gated policy preview timer status](workpacks/79-source-gated-policy-preview-timer-status.md) | Complete, bounded projection |
| [81 source gated policy preview timer runtime readiness](workpacks/81-source-gated-policy-preview-timer-runtime-readiness.md) | Complete, bounded projection |
| [82 source gated policy preview timer scheduler persistence](workpacks/82-source-gated-policy-preview-timer-scheduler-persistence.md) | Complete, bounded projection |
| [83 source gated policy preview timer audit rollback handoff](workpacks/83-source-gated-policy-preview-timer-audit-rollback-handoff.md) | Complete, bounded projection |
| [84 source gated policy preview timer audit rollback read model](workpacks/84-source-gated-policy-preview-timer-audit-rollback-read-model.md) | Complete, bounded projection |
| [85 source gated policy preview timer audit rollback parent surface intent](workpacks/85-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent.md) | Complete, bounded projection |
| [86 source gated policy preview timer service readiness handoff](workpacks/86-source-gated-policy-preview-timer-service-readiness-handoff.md) | Complete, bounded projection |
| [87 source gated policy preview timer service readiness read model](workpacks/87-source-gated-policy-preview-timer-service-readiness-read-model.md) | Complete, bounded projection |
| [88 source gated policy preview timer service readiness protocol handoff](workpacks/88-source-gated-policy-preview-timer-service-readiness-protocol-handoff.md) | Complete, bounded projection |
| [89 source gated policy preview timer service readiness protocol read model](workpacks/89-source-gated-policy-preview-timer-service-readiness-protocol-read-model.md) | Complete, bounded projection |
| [90 source gated policy preview timer service readiness protocol command handoff](workpacks/90-source-gated-policy-preview-timer-service-readiness-protocol-command-handoff.md) | Complete, bounded projection |
| [91 source gated policy preview timer service readiness service handler handoff](workpacks/91-source-gated-policy-preview-timer-service-readiness-service-handler-handoff.md) | Complete, bounded projection |
| [92 source gated policy preview timer service readiness read api handoff](workpacks/92-source-gated-policy-preview-timer-service-readiness-read-api-handoff.md) | Complete, bounded projection |
| [93 source gated policy preview timer service readiness read api response handoff](workpacks/93-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff.md) | Complete, bounded projection |
| [94 source gated policy preview timer service readiness read api response consumer handoff](workpacks/94-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff.md) | Complete, bounded projection |
| [95 source gated policy preview timer service readiness response consumer parent surface handoff](workpacks/95-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff.md) | Complete, bounded projection |
| [96 source gated policy preview timer service readiness response consumer parent surface read model handoff](workpacks/96-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff.md) | Complete, bounded projection |
| [97 source gated policy preview timer service readiness response consumer parent surface status handoff](workpacks/97-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff.md) | Complete, bounded projection |
| [98 source gated policy preview timer service readiness response consumer parent surface status read model handoff](workpacks/98-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff.md) | Complete, bounded projection |
| [99 source gated policy preview timer service readiness response consumer parent surface status read model parent surface handoff](workpacks/99-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff.md) | Complete, bounded projection |
| [100 source gated policy preview timer service readiness response consumer parent surface status read model parent surface read model handoff](workpacks/100-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff.md) | Complete, bounded projection |
| [101 source gated policy preview timer service readiness response consumer parent surface status read model parent surface read model](workpacks/101-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model.md) | Complete, bounded projection |
| [102 source gated policy preview timer service readiness response consumer parent surface status read model parent surface read model service handoff](workpacks/102-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff.md) | **Route-only; no App production source required; shared dependency validation open** |
| [103 timer service read model handoff](workpacks/103-timer-service-read-model-handoff.md) | Complete |
| [104 timer service event handoff](workpacks/104-timer-service-event-handoff.md) | Complete |
| [105 timer service read api handoff](workpacks/105-timer-service-read-api-handoff.md) | Complete |
| [106 timer service read api response handoff](workpacks/106-timer-service-read-api-response-handoff.md) | Complete |
| [107 timer service read api response consumer handoff](workpacks/107-timer-service-read-api-response-consumer-handoff.md) | Complete |
| [108 timer service read api response consumer parent surface handoff](workpacks/108-timer-service-read-api-response-consumer-parent-surface-handoff.md) | Complete |

## Resume rule

The first implementation frontier is WP18/WP49. Select it through the graph,
claim exact files, and keep the audit/index/graph synchronized as code truth
changes. Do not reopen completed bounded projection packets merely because their
historical proof paths are missing; proof is Phase 3.
