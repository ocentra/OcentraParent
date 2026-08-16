# 15 Browser Policy Authoring Manifest

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `15 Browser Policy Authoring Manifest`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
- `packages/parent-domain/tests/browser-control-contracts.test.ts`
- `packages/parent-domain/tests/browser-policy-questionnaire-forest.test.ts`
- No vendor/core-ui visual source was changed in this workpack; the existing
  questionnaire remains a typed-data consumer.

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
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/15-browser-policy-authoring-manifest/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior; this workpack repaired contract tests and the missing questionnaire computed-flag evaluator against existing typed fields.
- [ ] Rust/service/portal parity updated only after contracts exist; no Rust/service/portal runtime change was needed because WP15 is an authoring contract slice.
- [ ] Raw evidence artifacts captured where applicable: not applicable, because no bridge/CDP/session/journal/SQLite/policy-action runtime path changed.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed and `output/browser-plan-proof/15-browser-policy-authoring-manifest/06-ui-snapshots/ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: no default profile attach, no unowned bridge, no unmanaged exact URL claim, no raw debugger URL exposure, and no AI direct enforcement.
- [ ] Manual platform proof captured for real browser/OS claims: marked not applicable/manual-required because this workpack does not prove browser policy files are writable on a device.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Policy authoring does not prove enforcement or evidence availability. Managed
Chrome/Edge policy writing remains a typed authoring input with fallback states;
it does not prove browser policy file/registry writes, exact URL enforcement,
active-tab enforcement, unmanaged exact URL evidence, child-facing warning/block
delivery, or platform support.
