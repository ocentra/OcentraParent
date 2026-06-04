# App + Game Workpacks

These workpacks execute the shared app/game evidence spine without creating
duplicate app and game systems. Each workpack must fill its AI worker checklist
before `DONE` or PR-ready reporting.

## Base Sequence

1. Contract boundary and Effect schemas.
2. Source index and doc reconciliation.
3. Current app/game snapshot and gap map.
4. App/game identity model.
5. Inventory evidence model.
6. Windows installed app/game inventory adapter.
7. Windows Store/UWP/AppX/MSIX inventory adapter.
8. Windows process runtime evidence adapter.
9. Windows foreground app/game evidence adapter.
10. Launcher evidence and game candidate model.
11. Cross-platform authority matrix.
12. App and game category/risk taxonomy.
13. Sessionization and duration engine.
14. Journal and SQLite ingest.
15. Read models and service events.
16. Parent portal app/game dashboard surfaces.
17. Unknown app/game approval flow.
18. Native game budgets and launcher policy.
19. Policy target compiler for app/game rules.
20. Time budget, schedule, and bonus-time integration.
21. Child-facing app/game warning and request UX.
22. Windows owned-process terminate time-limit proof.
23. Broad blocking proof gates.
24. AI classifier digest boundary.
25. Platform extension checklist and proof routing.
26. Install, uninstall, purchase, and store handoffs.
27. Performance and service health.
28. E2E, manual proof, rollout, and PR gate.
29. Rust protocol evidence identity parity.
30. Rust protocol authority classifier parity.
31. Journal/SQLite authority classifier storage.
32. Live process snapshot source.
33. Live process journal SQLite bridge.
34. Service capture app/game live process bridge.
35. Service app/game recurring freshness.
36. Live foreground window source.
37. Service foreground capture bridge.
38. Service authority/classifier surface evidence.
39. Authority/classifier read-model counts.
40. App/game boundary read-model event.
41. Live Windows inventory source.
42. Service Windows inventory capture bridge.
43. Live Windows Store package source.
44. Service Windows Store package capture bridge.
45. Live Windows registry inventory source.
46. Service Windows registry capture bridge.
47. Backend source freshness read-model rows.
48. Category/risk policy routing.
49. Policy readiness service read model.

## Standard AI Worker Checklist

- [ ] Confirm source docs read: folder README, source index, current snapshot,
      shared evidence spine plan, native apps slice, native games slice,
      platform deep dive, test blueprint, UI/UX guide, main checklist, and this
      workpack.
- [ ] Confirm browser-game scope remains in browser-plan.
- [ ] Confirm apps and games share low-level evidence but keep separate product
      meaning.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel app-control or game-control
      truth created.
- [ ] Before-state source snapshot recorded in
      `output/app-game-plan-proof/<workpack-id>/00-source-snapshot.md`.
- [ ] Contracts updated first where behavior changes.
- [ ] Rust/service/portal parity updated only after TypeScript contracts exist.
- [ ] Tests/proof listed in this workpack and test blueprint are implemented or
      explicitly manual-required with reason.
- [ ] Security/no-claim negative proof captured where applicable: inventory is
      not use, runtime is not foreground, foreground is not content, launcher is
      not game, AI cannot enforce, manual-required cannot execute.
- [ ] Feature/expectation/product-checklist/README update decision recorded.
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before
      `DONE`.
