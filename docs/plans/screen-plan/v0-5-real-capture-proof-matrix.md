# V0.5 Real Capture Proof Matrix

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `V0.5 Real Capture Proof Matrix`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This is a merge-blocking proof matrix. A screen-capture slice cannot be called
done only because contracts and unit tests pass. It must prove that real runtime
activity triggers a real capture job, writes a real encrypted queue item, records
the real source trigger, and deletes the raw image after analysis or expiry.

## Proof Layers

| Layer                    | Purpose                                                                                | Required before done                   |
| ------------------------ | -------------------------------------------------------------------------------------- | -------------------------------------- |
| Controlled CI proof      | Deterministic local browser/app fixtures that run in automation                        | Yes                                    |
| Real local service proof | Real Rust service, real portal, real queue, real capture adapter where platform allows | Yes                                    |
| Operator live proof      | User or worker opens real sites/apps on the machine and captures artifacts             | Required before product-complete claim |
| Physical/platform proof  | OS permission, protected surface, and platform-specific capture behavior               | Required before platform claim         |

## Capture Trigger Proofs

Each trigger proof must write:

```text
output/screen-plan-proof/real-capture/<trigger-id>/
  00-scenario.md
  01-trigger-input.json
  02-source-evidence.json
  03-capture-job.json
  04-encrypted-queue-proof.json
  05-image-digest.txt
  06-analysis-or-skip-result.json
  07-deletion-proof.json
  08-portal-or-service-log.txt
  09-screenshot-of-parent-ui.png
```

## Required Trigger Scenarios

| Trigger id                            | Scenario                                                                                                             | Must prove                                                                                                                                             |
| ------------------------------------- | -------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `managed-browser-url-social-video`    | Managed browser navigates to a social/video page such as YouTube or a controlled YouTube-like fixture                | URL-change evidence caused capture or structured-extraction route; capture job cites browser evidence; no capture if structured evidence fully answers |
| `managed-browser-url-education-video` | Managed browser navigates to an education video page such as an educational YouTube/Vimeo page or controlled fixture | Education/video trigger captured or skipped with structured proof; output keeps category as evidence, not policy                                       |
| `managed-browser-url-social-feed`     | Managed browser navigates to a social feed/sign-in page such as Facebook or controlled fixture                       | Social/feed route triggers capture or structured extraction; evidence cites URL/title/source                                                           |
| `managed-browser-url-game`            | Managed browser opens browser game/cloud game/local game fixture                                                     | Browser-game trigger links screenshot summary to browser/game evidence                                                                                 |
| `native-app-foreground`               | A real app such as Notepad, Calculator, Paint, VS Code, or a fixture app becomes foreground                          | Foreground-app-change trigger creates active-window route when parent setting allows                                                                   |
| `native-game-foreground`              | A native game or controlled game-window fixture becomes foreground                                                   | Native-game trigger creates active-window route and links app/game session refs                                                                        |
| `unknown-process-foreground`          | Unknown executable/window fixture becomes foreground                                                                 | Unknown-process trigger creates capture route or manual-required state by policy                                                                       |
| `launcher-foreground`                 | Game launcher/store fixture or real launcher becomes foreground                                                      | Launcher-only state is visible and does not count as active game proof                                                                                 |
| `policy-ambiguity`                    | Existing evidence is ambiguous under parent rules                                                                    | Capture request is created only when policy allows screen clarification                                                                                |
| `parent-manual-test-capture`          | Parent clicks manual test capture                                                                                    | Manual trigger creates one capture and audit event                                                                                                     |
| `timed-cadence-active-window`         | Parent sets cadence to a short test interval such as 2 seconds                                                       | At least three captures happen at bounded intervals; debounce prevents flood                                                                           |
| `timed-cadence-full-display-opt-in`   | Parent explicitly opts into full-display cadence for a test interval                                                 | Full-display capture proves strict opt-in, cadence spacing, and deletion                                                                               |
| `disable-stops-cadence`               | Parent disables screen analysis during cadence                                                                       | No new capture jobs are created after disable event                                                                                                    |
| `protected-surface-skip`              | Protected/secure/permission-required surface is active                                                               | Capture is skipped and protected/degraded state is recorded                                                                                            |

## Operator Live URL Proof Set

The operator live proof is intentionally separate from CI because public sites
change and may require login, regional routing, or cookie banners. It still must
be run before claiming product completeness.

Minimum operator live URLs:

- YouTube random/ordinary video.
- YouTube or Vimeo educational video.
- Vimeo ordinary video.
- Facebook public/sign-in/feed surface, depending on what is legally accessible.
- Browser game or cloud-game surface.
- Shopping page.
- School/productivity page.
- A clearly unsupported or protected surface.

For each live URL:

- record the URL or redacted URL;
- record source evidence from browser/app;
- record capture or structured-extraction decision;
- record image digest if captured;
- record deletion proof;
- record parent portal screenshot.

## Timed Capture Requirements

Timed capture cannot be hand-waved. Tests may use seconds instead of minutes,
but the same scheduler must be used.

- Parent setting controls cadence.
- Parent setting controls scope.
- Scheduler records intended cadence and actual capture timestamps.
- Debounce and minimum interval prevent flooding.
- Queue backpressure is visible.
- Disabling capture stops future cadence jobs.
- Existing queued images still delete by success or TTL.

## Done Gate

Screen capture is not done until:

- at least one browser-use trigger fires real capture or real structured skip;
- at least one app-use trigger fires real capture;
- at least one timed cadence proof captures multiple frames;
- disabled setting prevents capture;
- raw image encryption and deletion are proved;
- portal/service evidence shows the trigger source;
- proof artifacts are written under `output/screen-plan-proof/real-capture`.
