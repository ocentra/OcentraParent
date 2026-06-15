# Tracking Plan Coverage Audit

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Plan Coverage Audit`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This file proves that the pasted scope from the planning chats was not lost.

## Attachment Map

| Source                                                                                            | Used for                                                                                                       |
| ------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| `C:\Users\sujan\.codex\attachments\f6b10f30-7802-442a-8199-cd6dbe7b9bcb\pasted-text.txt`          | First long draft: product behavior, tracking modes, contract families, platform proof, tests, UI, workpacks.   |
| `C:\Users\sujan\.codex\attachments\da33059b-0d1f-4432-a963-7cea423b32c0\pasted-text.txt`          | GPT guide: browser-plan style split, mermaid/workpack/checklist structure, worker instructions, proof routing. |
| `https://chatgpt.com/s/t_6a1f742c71dc8191882dace37fcff4fe`                                        | Checked, but inaccessible beyond login shell/title in this environment.                                        |
| `docs/features/location-geofence-device-status.md`                                                | Repo source of truth for the feature boundary and current status.                                              |
| `docs/expectations/location-geofence.md` and related expectation docs listed in `source-index.md` | Acceptance contract and adjacent boundary rules.                                                               |

## Coverage Checklist

This audit requires all of these to stay represented:

- source index;
- current snapshot;
- full-scope plan;
- AI/safety plan;
- platform deep dive;
- test blueprint;
- UI/UX guide;
- implementation checklist;
- pasted-content coverage;
- numbered workpacks with proof roots;
- manual-required platform gaps;
- product-doc/checklist update discipline;
- no-claim rules for LAN/IP/AI/nearby places/background behavior.

## Covered Pasted Scope

- [ ] Parent can know where child device was reported.
- [ ] Last-known/live/stale/offline states.
- [ ] Geofence home/school/activity/safe/restricted zones.
- [ ] Expected-place schedule: school/home/activity.
- [ ] Nearby-place intelligence: cinema, mall, hospital, bar, etc.
- [ ] AI analysis over structured evidence.
- [ ] Warning/urgent/critical notification policy.
- [ ] Parent acknowledgement.
- [ ] Holiday/exception state.
- [ ] False-alarm handling.
- [ ] Child check-in.
- [ ] Temporary live tracking.
- [ ] Missing device mode.
- [ ] Battery/connectivity status.
- [ ] Retention/delete/export controls.
- [ ] Android/iOS background permission proof.
- [ ] Desktop hint-only states.
- [ ] Platform extension checklists.
- [ ] Tests, Playwright, manual platform proof.
- [ ] Proof packs and merge gates.
- [ ] Browser-plan style README shape.
- [ ] Source index.
- [ ] Current snapshot.
- [ ] Full-scope plan.
- [ ] AI/safety analysis plan.
- [ ] Platform deep dive.
- [ ] Test blueprint.
- [ ] UI/UX guide.
- [ ] Implementation checklist.
- [ ] Coverage audit.
- [ ] Workpack structure.
- [ ] Progress reconciliation.
- [ ] Codex worker instructions.

## Consolidation Decisions

- Kept the first draft's dedicated Google Places/POI adapter by adding a
  separate provider workpack instead of burying it inside generic nearby-place
  work.
- Kept the GPT guide's browser/app-plan style by making source, snapshot,
  checklist, proof routing, and worker instructions first-class docs.
- Split journal/read-model proof and rollout/PR gate into separate workpacks
  because they are shared proof responsibilities, not platform adapters.
- Reworded pasted examples into repo-safe planning language and ASCII text.
- Left platform documentation as planning input only; implementation workers
  must re-verify official docs and produce proof.

## Not Claimed Until Proof

- [ ] Continuous real-time tracking.
- [ ] Precise place detection from low-accuracy GPS.
- [ ] Emergency determination from AI alone.
- [ ] Hospital/bar/cinema exact presence from nearby POI alone.
- [ ] Background Android tracking without permission proof.
- [ ] Background iOS tracking without permission proof.
- [ ] Remote/cloud sync by default.
- [ ] Remote AI by default.
- [ ] Managed-device/lost-mode behavior without platform proof.

## Known Transformations

- The pasted drafts had encoding artifacts. The plan rewrites user-facing docs
  as ASCII.
- The first draft proposed 32 workpacks, including a dedicated Google Places /
  POI adapter. The second GPT guide proposed a
  30-base-workpack browser-plan style plus platform extension checklists. This
  folder preserves both by using 33 base workpacks: the 30-style base plus the
  dedicated provider adapter, journal/read-model, and rollout/PR-gate workpacks
  from the first draft.
- TypeScript-like examples are represented as contract families and tests, not
  copied into runtime code.
- External platform facts are planning inputs that implementation workers must
  re-verify against current official docs.
