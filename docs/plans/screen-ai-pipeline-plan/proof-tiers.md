# Screen AI Pipeline Proof Tiers

Unsupported, protected, permission-denied, or degraded states are not completion
proof for capture, analysis, or action claims. They are negative evidence that
the product handled a missing capability honestly.

| Tier | Meaning | Can Claim |
| --- | --- | --- |
| P0_CONTRACT | Schema, parser, DTO, policy compiler, and state-machine tests. | Contract shape is correct. |
| P1_FIXTURE_SIMULATION | Controlled local fixtures and simulated sources. | Logic behaves against known fixtures. |
| P2_HOSTED_CI | GitHub-hosted Linux, Windows, and macOS build/test jobs. | Code compiles and hosted-safe behavior works on those runners. |
| P3_LOCAL_DEV_MACHINE | Real local desktop machine proof, such as this Windows worker host. | Desktop adapter claim for the exercised host only. |
| P4_PHYSICAL_DEVICE | Real Android, iOS, or separate desktop device proof. | Device behavior claim for the exercised device class. |
| P5_AUTHORITY_ENROLLED_DEVICE | Device Owner, MDM, supervised, Endpoint Security, App Control, or similar authority proof. | Hard-control claim for that enrolled authority setup. |
| P6_PRODUCTION_PILOT | Real opt-in family pilot with rollback and support notes. | Pilot readiness claim. |

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

Mac and iOS proof require real user-assisted hardware/session evidence when the
current worker machine cannot exercise the platform. Until that happens, the
plan can claim only compile/degraded behavior for those platforms, not capture
capability.
