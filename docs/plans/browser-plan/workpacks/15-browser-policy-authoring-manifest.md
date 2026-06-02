# 15 Browser Policy Authoring Manifest

## Where We Are

`parent-domain` contains browser control manifest, catalog, full-catalog data,
policy value/update contracts, coverage matrix, and questionnaire forest data.
The portal also has a browser rules questionnaire component.

## Where We Want To Be

Browser policy UI renders from typed authoring manifests and writes only through
validated policy update commands. This workpack also carries the pasted
managed Chrome/Edge policy-writer scope: the UI and domain contracts expose
only typed, capability-gated inputs for browser policies such as incognito,
guest/profile creation, history deletion, safe search, restricted mode, and
URL allow/block lists.

## Scope

- Authoring manifest sections/fields/options.
- Policy value document.
- Effective policy compile output.
- Preview, patch, replace, rollback commands.
- Capability-aware visibility and disabled states.
- Managed Chrome/Edge policy-writer inputs for disabling incognito, disabling
  guest browsing, disabling profile adding, limiting history deletion where
  supported, enabling safe search or restricted mode where supported, and
  writing URL allow/block lists.
- Browser-game policy questions for educational games, unknown games,
  cloud-gaming approval, game purchases/accounts, unblocked portals, WebGL/canvas
  games, and time budgets when contracts exist.
- No UI-invented policy questions.

## Touched Paths

- `packages/parent-domain/src/browser-control-manifest.ts`
- `packages/parent-domain/src/browser-control-policy.ts`
- `packages/parent-domain/src/browser-policy-questionnaire-forest*.ts`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/BrowserRulesQuestionnaire.tsx`

## Tests And Proof

- Manifest field/write path tests.
- Policy update validation tests.
- Managed Chrome/Edge policy-writer input tests for supported, unsupported,
  degraded, and manual-required states.
- Portal questionnaire fixture tests.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/<workpack-id>/00-source-snapshot.md` or explicit docs-only N/A reason.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after contracts exist.
- [ ] Raw evidence artifacts captured where applicable: bridge/CDP payloads, managed session state, unmanaged process rows, journal entries, SQLite/read-model rows, policy decisions, and action results.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; if no UI changed, `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: no default profile attach, no unowned bridge, no unmanaged exact URL claim, no raw debugger URL exposure, and no AI direct enforcement.
- [ ] Manual platform proof captured for real browser/OS claims, including OS/browser version, command steps, screenshots/logs, and manual-required labels.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Policy authoring does not prove enforcement or evidence availability.
