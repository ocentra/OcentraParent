# Screen AI Pipeline Proof Tiers

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `Screen AI Pipeline Proof Tiers`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Unsupported, protected, permission-denied, or degraded states are not completion
proof for capture, analysis, or action claims. They are negative evidence that
the product handled a missing capability honestly.

| Tier                         | Meaning                                                                                    | Can Claim                                                      |
| ---------------------------- | ------------------------------------------------------------------------------------------ | -------------------------------------------------------------- |
| P0_CONTRACT                  | Schema, parser, DTO, policy compiler, and state-machine tests.                             | Contract shape is correct.                                     |
| P1_FIXTURE_SIMULATION        | Controlled local fixtures and simulated sources.                                           | Logic behaves against known fixtures.                          |
| P2_HOSTED_CI                 | GitHub-hosted Linux, Windows, and macOS build/test jobs.                                   | Code compiles and hosted-safe behavior works on those runners. |
| P3_LOCAL_DEV_MACHINE         | Real local desktop machine proof, such as this Windows worker host.                        | Desktop adapter claim for the exercised host only.             |
| P4_PHYSICAL_DEVICE           | Real Android, iOS, or separate desktop device proof.                                       | Device behavior claim for the exercised device class.          |
| P5_AUTHORITY_ENROLLED_DEVICE | Device Owner, MDM, supervised, Endpoint Security, App Control, or similar authority proof. | Hard-control claim for that enrolled authority setup.          |
| P6_PRODUCTION_PILOT          | Real opt-in family pilot with rollback and support notes.                                  | Pilot readiness claim.                                         |

Every proof artifact must record:

- required proof tier
- current proof tier
- status
- artifact path
- missing proof reason when not proved
- whether degraded evidence is being treated as a non-claim

For this plan, a successful Windows capture claim requires P3 evidence with
actual image bytes captured locally, encrypted into temporary custody, raw bytes
deleted, and proof artifacts written under
`output/screen-plan-proof/real-capture/`.

## Platform Adapter Rule

Real platform adapters are part of the work, not optional follow-up language.

CI should be used to prove compile, schema, hosted-runner behavior, and honest
degraded states on Windows, Linux, and macOS where GitHub-hosted runners are
available. CI is not a substitute for an actual platform capture claim unless it
captures real image bytes on that runner and records custody/deletion proof.

Android proof must use a real emulator or physical device path for the claimed
capability, such as MediaProjection consent/foreground-service behavior. Docker,
WSL, and Android Studio are acceptable local tools for setup and validation
where they can exercise the real adapter boundary.

Current stacked branch Android proof records the emulator side of that rule at
`output/screen-plan-proof/android-mediaprojection/proof-summary.json`: Android
API 35 MediaProjection consent UI, foreground service, captured frame digest,
and raw temporary frame deletion. This is adapter proof for the exercised
emulator only. It does not satisfy P4 physical-device parity and does not claim
silent background capture.

Linux proof must split X11 and Wayland claims. X11 can claim only a real X11
desktop/session capture. Wayland must prove the desktop-portal/PipeWire
permission/session path. WSL/WSLg compile or fixture runs are not Linux
child-device capture proof unless the same adapter captures real pixels and
records custody/deletion.

Current stacked branch Linux proof records the WSLg/X11 selected-window side of
that rule at `output/screen-plan-proof/linux-wslg/proof-summary.json`: WSLg
display session, real X11 window capture, encrypted temporary custody, and raw
temporary PNG deletion. This does not satisfy WSLg root-display capture, native
Wayland portal capture, or broad Linux compositor parity.

Mac and iOS proof require real user-assisted hardware/session evidence when the
current worker machine cannot exercise the platform. Before that evidence
exists, the plan can claim only compile/degraded behavior for those platforms,
not capture capability.
