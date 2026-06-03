# Tracking Plan Coverage Audit

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

- [x] Parent can know where child device was reported.
- [x] Last-known/live/stale/offline states.
- [x] Geofence home/school/activity/safe/restricted zones.
- [x] Expected-place schedule: school/home/activity.
- [x] Nearby-place intelligence: cinema, mall, hospital, bar, etc.
- [x] AI analysis over structured evidence.
- [x] Warning/urgent/critical notification policy.
- [x] Parent acknowledgement.
- [x] Holiday/exception state.
- [x] False-alarm handling.
- [x] Child check-in.
- [x] Temporary live tracking.
- [x] Missing device mode.
- [x] Battery/connectivity status.
- [x] Retention/delete/export controls.
- [x] Android/iOS background permission proof.
- [x] Desktop hint-only states.
- [x] Platform extension checklists.
- [x] Tests, Playwright, manual platform proof.
- [x] Proof packs and merge gates.
- [x] Browser-plan style README shape.
- [x] Source index.
- [x] Current snapshot.
- [x] Full-scope plan.
- [x] AI/safety analysis plan.
- [x] Platform deep dive.
- [x] Test blueprint.
- [x] UI/UX guide.
- [x] Implementation checklist.
- [x] Coverage audit.
- [x] Workpack structure.
- [x] Progress reconciliation.
- [x] Codex worker instructions.

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
