# Tracking Proof Tiers

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Proof Tiers`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This file is the proof taxonomy for tracking work. It prevents two bad
outcomes:

- CI-only evidence being inflated into a physical-device product claim.
- physical-device or enrolled-device proof being treated as a normal GitHub CI
  failure.

The same tier model should be copied into the LAN, browser, and app-game plan
folders when those lanes are free to edit their own plan docs.

## Tier Enum

```ts
type ProofTier =
  | 'P0_CONTRACT'
  | 'P1_FIXTURE_SIMULATION'
  | 'P2_HOSTED_CI'
  | 'P3_LOCAL_DEV_MACHINE'
  | 'P4_PHYSICAL_DEVICE'
  | 'P5_AUTHORITY_ENROLLED_DEVICE'
  | 'P6_PRODUCTION_PILOT';

type ProofStatus =
  | 'proved'
  | 'simulated'
  | 'manual_required'
  | 'authority_required'
  | 'not_claimed'
  | 'blocked'
  | 'failed';
```

## Tier Meaning

| Tier | Name                            | What it proves                                                                                                                                                      |
| ---- | ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| P0   | Contract proof                  | Schemas, DTOs, branded ids, state machines, policy compiler inputs, and no OS or device behavior.                                                                   |
| P1   | Fixture/simulation proof        | CI-safe fake adapters, fake GPS, fake geofence, fake provider, deterministic fixtures, and no product platform claim.                                               |
| P2   | Hosted CI proof                 | GitHub-hosted Ubuntu/Windows/macOS build and test, Rust/TypeScript checks, SQLite/journal replay, Playwright, Android emulator, and iOS simulator where configured. |
| P3   | Local developer machine proof   | Local Windows, WSL, Docker, Android Studio, MacBook, or other developer hardware proof.                                                                             |
| P4   | Physical device proof           | Real Android, iPhone/iPad, MacBook, Windows laptop, or other device proof for physical/mobile behavior.                                                             |
| P5   | Authority/enrolled device proof | Android Device Owner, iOS supervised/MDM, macOS Endpoint Security/MDM/PPPC, Windows AppLocker/App Control, or equivalent special authority proof.                   |
| P6   | Production pilot proof          | Opt-in real family schedule/device/battery/network proof with logs, screenshots, rollback, and privacy review.                                                      |

## Status Rule

Do not require P4/P5 proof in normal CI. For every proof item, record:

```text
Required proof tier:
Current proof tier:
Current status:
Proof artifact path:
Manual-required reason:
```

If a feature needs Android physical device, iOS physical device, MDM, Device
Owner, Endpoint Security, AppLocker, App Control, or real home LAN, mark it
`manual_required` or `authority_required` unless the matching proof artifact
exists.

CI passing P0/P1/P2 is enough to mark code-ready for the covered boundary. It is
not enough to mark a physical-device, background-mobile, enrolled-device, or
production product claim proved.

Fail the branch only when code/docs pretend P4/P5/P6 proof exists while the
artifact is missing, or when the required lower-tier proof fails.

## Tracking CI Matrix

| Claim                       | GitHub hosted CI    | Android Studio | Android physical device                  | MacBook              | iOS physical device | Product claim status rule                                              |
| --------------------------- | ------------------- | -------------- | ---------------------------------------- | -------------------- | ------------------- | ---------------------------------------------------------------------- |
| Location evidence schema    | P0/P2               | P0/P2          | P0/P2                                    | P0/P2                | P0/P2               | Code-ready when contracts pass.                                        |
| Geofence state machine      | P1/P2               | P1/P2          | P4                                       | P1/P2                | P4                  | Product claim waits for target platform proof.                         |
| Simulated GPS               | P1/P2               | P1/P3          | P3/P4 if developer mock mode is explicit | P1/P3 simulator      | P3/P4 if explicit   | Simulation never becomes real background proof.                        |
| Android background geofence | no final proof      | P3 partial     | P4 required                              | n/a                  | n/a                 | `manual_required` until physical Android artifact exists.              |
| iOS region monitoring       | no final proof      | n/a            | n/a                                      | P3 simulator partial | P4 required         | `manual_required` until physical iOS artifact exists.                  |
| Battery saver / app killed  | no final proof      | P3 partial     | P4 required                              | n/a                  | P4/manual           | `manual_required` until physical device artifact exists.               |
| Nearby POI ambiguity        | P1/P2 fake provider | P1/P3          | P3/P4                                    | P1/P3                | P3/P4               | Code-ready from fake provider; provider claim waits for adapter proof. |
| Parent alert/escalation     | P0/P1/P2            | P0/P1/P3       | P4 optional for UX                       | P0/P1/P3             | P4 optional for UX  | Policy code can be ready without platform notification product claim.  |
| Retention/delete/export     | P0/P1/P2            | P2/P3          | P4 optional                              | P2/P3                | P4 optional         | Code-ready when journal/read-model proof and UI delete proof pass.     |

## Current Environment Mapping

| Environment                        | Valid tracking proof                                                                                                                                                    | Non-claim                                                                                                                   |
| ---------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| GitHub hosted Ubuntu/Windows/macOS | contracts, Rust/TypeScript tests, journal/SQLite replay, policy/compiler proof, fake provider/adapter proof, Playwright UI, Android emulator, iOS simulator build/tests | real mobile background reliability, enrolled device authority, real home-network/LAN conditions, production family behavior |
| Windows PC                         | Rust/Node proof, SQLite/journal, portal screenshots, Android Studio emulator, local notification, Windows desktop hints if implemented                                  | iOS physical proof, Android physical proof unless a phone is connected, enrolled-device proof unless configured             |
| WSL                                | Linux parser/build/journal proof and fake Linux adapters                                                                                                                | full Linux desktop foreground, NetworkManager, Wayland/X11 focus, real Flatpak/Snap behavior                                |
| Docker                             | contracts, API, fake adapters, fake providers, replay, policy, AI/mock servers                                                                                          | real OS foreground, LAN discovery, mobile background, system blocking                                                       |
| Android Studio emulator            | build, instrumented UI tests, permission UI, foreground simulated location, local DB, foreground service notification                                                   | final background geofence reliability, OEM killing, Doze, real cell/Wi-Fi transitions                                       |
| Android physical device            | real permission flows, geofence, battery/degraded state, offline queue, reboot/killed behavior                                                                          | Device Owner unless a dedicated enrolled test device is used                                                                |
| MacBook                            | macOS build/runtime proof, iOS simulator, iOS signing pipeline, physical iOS via trusted device                                                                         | iOS physical proof unless the iPhone/iPad is attached/trusted                                                               |
| iOS physical device                | Core Location, Always/When In Use flow, region monitoring, significant-change/visits, local notifications, child check-in UX                                            | FamilyControls/ManagedSettings unless entitlement/setup exists                                                              |

## Manual Proof Artifact Contract

Manual proof roots must write artifacts under:

```text
output/<plan>-proof/<workpack-id>/
  00-run-metadata.json
  01-device-metadata.json
  02-permission-state.json
  03-input-scenario.json
  04-runtime-evidence.ndjson
  05-journal-records.ndjson
  06-sqlite-readmodel.json
  07-policy-decision.json
  08-action-or-alert-result.json
  09-screenshots/
  10-video-or-logcat-if-mobile.txt
  11-manual-notes.md
  12-result-summary.md
```

Android background geofence proof root:

```text
output/tracking-plan-proof/android-background-geofence/
  00-run-metadata.json
  01-device-metadata.json
  02-permission-state.json
  03-geofence-definition.json
  04-location-events.ndjson
  05-geofence-transitions.ndjson
  06-alert-decision.json
  07-parent-ui-screenshot.png
  08-logcat.txt
  09-result-summary.md
```

iOS region monitoring proof root:

```text
output/tracking-plan-proof/ios-region-monitoring/
  00-run-metadata.json
  01-device-metadata.json
  02-authorization-state.json
  03-region-definition.json
  04-location-events.ndjson
  05-region-transitions.ndjson
  06-alert-decision.json
  07-screenshots/
  08-xcode-test-log.txt
  09-result-summary.md
```

## Done State Vocabulary

Use these states in tracking workpack reports:

```text
not_started
contracts_done
fixture_proof_done
hosted_ci_done
local_dev_proof_done
physical_device_manual_required
authority_enrollment_required
physical_device_proved
authority_proved
product_claim_ready
```

Example:

```text
Android geofence:
contracts_done
fixture_proof_done
hosted_ci_done
local_dev_proof_done
physical_device_manual_required
product_claim_ready: false
```

That is honest progress, not failure.

## Official Proof References

- GitHub-hosted runners support Ubuntu, Windows, and macOS runner images, and
  GitHub documents Android SDK hardware acceleration support on hosted Linux
  runners:
  https://docs.github.com/en/actions/reference/runners/github-hosted-runners
- Android Gradle managed devices can create, deploy, and tear down configured
  virtual or remote physical devices for automated tests:
  https://developer.android.com/studio/test/managed-devices
- Android geofencing supports enter, exit, and dwell behavior:
  https://developer.android.com/develop/sensors-and-location/location/geofencing
- Android 11+ background location requires the user to enable background
  location through settings rather than the runtime permission dialog:
  https://developer.android.com/develop/sensors-and-location/location/permissions/background
- GitHub self-hosted runners can be added to repositories, organizations, or
  enterprises, and public fork safety must be considered:
  https://docs.github.com/en/actions/how-tos/manage-runners/self-hosted-runners/add-runners
- AWS Device Farm offers real physical Android/iOS/web app device testing as a
  later cloud-device option:
  https://docs.aws.amazon.com/devicefarm/latest/developerguide/welcome.html
