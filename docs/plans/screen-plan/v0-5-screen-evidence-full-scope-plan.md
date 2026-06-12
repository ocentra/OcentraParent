# V0.5 Screen Evidence Full Scope Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `V0.5 Screen Evidence Full Scope Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Core Rule

Screen evidence is a high-sensitivity local analysis layer. It starts disabled, requires parent opt-in, captures only approved scope, stores raw images only in an encrypted temporary queue, analyzes locally, stores typed summaries, deletes raw images, and lets policy consume only validated summaries and evidence refs.

Screen evidence is cross-slice. Browser, apps, native games, browser games, social/video, bypass-tool, unknown-activity, and tracking/check-in contexts may consume screen summaries when the policy question requires visual evidence.

Screen evidence does not replace source evidence:

```text
App/Game/Browser evidence says what surface is active.
Screen evidence helps understand what is visible.
Policy decides what to do.
```

The first MVP should finish capture, routing, encrypted queue, deletion, and summary alignment. Full OCR/VLM model selection and quality proof can continue in the AI plan or a second screen AI pass, as long as the capture MVP leaves typed hooks for processing results and policy actions.

## Product Modes

1. Off.
2. Capability only.
3. Local summary observe-only.
4. Local summary policy dry-run.
5. Local summary enforcement-eligible.
6. Optional raw screenshot retention.
7. Optional live view.
8. Unsupported / permission-required / manual-required.

## Normal V0.5 Path

```text
parent opt-in
  -> capability check
  -> intelligence router checks browser/app/network/session evidence first
  -> scheduler/trigger
  -> capture approved scope
  -> encrypted temp queue
  -> local OCR/vision
  -> schema-valid summary
  -> delete temp image
  -> journal + SQLite
  -> portal summary
  -> policy dry-run/enforcement handoff
```

## Non-Goals

- No hidden capture.
- No cloud screenshot upload by default.
- No permanent screenshot history by default.
- No live view by default.
- No capture of secure desktop, password prompts, lock screen, or protected surfaces.
- No enforcement from raw model text or raw pixels.
- No browser-only ownership of screen evidence.
- No open-ended "describe this screen" prompts.

## Expanded Capture Triggers

- `managed_browser_url_change`
- `browser_game_detected`
- `native_app_foreground_start`
- `native_game_foreground_start`
- `launcher_foreground_start`
- `unknown_process_foreground_start`
- `unusual_network_change`
- `policy_ambiguity`
- `parent_manual_test_capture`

## Core Contract Groups

### Screen Analysis Settings

Required fields:

- setting ID, child profile ref, device ref;
- enabled state;
- observe-only, policy-dry-run, or enforcement-eligible mode;
- cadence and trigger settings;
- allowed capture scope;
- OCR text setting and snippet limit;
- redaction mode;
- temporary image TTL;
- retry count;
- delete after success and delete after expiry;
- retain raw image false for V0.5 default mode;
- policy use flag;
- parent change audit fields.

### Screen Capability Status

Required states:

- disabled by parent;
- unsupported platform;
- unsupported scope;
- permission required;
- permission limited;
- protected surface;
- screen locked;
- session unavailable;
- model unavailable;
- queue unavailable;
- degraded;
- adapter error;
- ready.

Capability status is evidence/portal state, not a successful observation unless a capture job and validated result exist.

### Screen Capture Queue Job

Required fields:

- queue job ID and schema version;
- created/not-before/expiry timestamps;
- capture reason and capture scope;
- source ID and adapter ID;
- device/local-user refs;
- parent setting ref/version;
- related evidence refs;
- encrypted image ref;
- image digest;
- image size/format if safe;
- queue status;
- retry counters;
- failure/unavailable reason;
- deletion required;
- deletion status/proof ref;
- child-device temporary queue custody.

### Screen Analysis Result

Required fields:

- result ID and schema version;
- queue job ID;
- source image digest;
- source evidence refs;
- analyzed timestamp;
- model runtime ref;
- visible activity categories;
- risk signals;
- OCR snippets only when allowed;
- redaction notes;
- confidence;
- uncertainty reasons;
- parent summary;
- child-safe summary when needed;
- deletion state/proof ref;
- custody state.

## Done Standard

This is complete only when a real parent can enable screen analysis, see truthful capability state, trigger approved capture, get local OCR/vision summaries, verify raw image deletion, inspect parent-readable evidence, and see policy use without raw screenshot leakage.
