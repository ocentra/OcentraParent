# V0.5 Screen Platform Deep Dive

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `V0.5 Screen Platform Deep Dive`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

Every platform claim requires a proof tier and artifact path. This doc describes target platform paths; it does not claim they are implemented.

## Proof Tiers

| Tier | Meaning                                             |
| ---- | --------------------------------------------------- |
| P0   | Documentation/spec only.                            |
| P1   | Fake/local adapter contract proof only.             |
| P2   | Hosted CI build/contract proof.                     |
| P3   | Local developer machine proof.                      |
| P4   | Child-device proof.                                 |
| P5   | Managed/privileged deployment proof.                |
| P6   | External authority/legal/privacy proof when needed. |

## Windows

Primary path:

- Windows.Graphics.Capture.
- Capture picker/consent.
- Display or app/window capture.
- User-visible capture indicator/border where required.
- Local queue plus local OCR.
- Protected-surface and degraded-state handling.

Proof:

- support check;
- picker proof;
- display capture;
- active window capture;
- managed browser capture;
- delete proof.

## macOS

Primary path:

- ScreenCaptureKit.
- Screen Recording privacy permission.
- Display/window/app capture.
- PPPC/MDM optional managed permission path.

Proof:

- permission UI;
- display capture;
- window capture;
- local OCR;
- deletion proof;
- MDM/PPPC manual-required unless proved.

## Linux

Primary path:

- X11 capture where available.
- Wayland portal/PipeWire where available.
- Compositor-specific status.

Proof:

- X11 proof;
- GNOME/KDE Wayland proof;
- unsupported compositor proof;
- manual-required states.

Linux must not claim universal capture.

## Android

Primary path:

- MediaProjection.
- Consent per session.
- Foreground service.
- Android app-window/full-display modes where supported.
- User-visible capture indicator.
- Stop callback.
- Local OCR.
- Deletion proof.

Hard rule:

```text
Do not claim silent continuous background screenshots.
MediaProjection needs visible user/session semantics and platform proof.
```

## iOS / iPadOS

Primary path:

- ReplayKit / broadcast extension.
- Explicit user/session capture.
- In-app or user-started capture.
- Not arbitrary background capture.

Hard rule:

```text
Do not claim arbitrary background screen capture of other apps.
Treat iOS screen visibility as explicit ReplayKit/session-based or not-claimed.
```
