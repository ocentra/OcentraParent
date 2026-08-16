# UI/UX Requirements Guide

<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `UI/UX Requirements Guide`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

Network UI must help parents understand evidence without implying surveillance
or control that the product has not proved.

## Parent Surfaces

Required surfaces:

- network activity dashboard;
- current flow/domain timeline;
- domain/IP/protocol/process detail drawer;
- evidence grade badge;
- source and custody label;
- cascade plan view;
- cross-slice evidence bundle view;
- analyzer alert and detection detail view;
- AI audit narrative with cited evidence refs;
- household risk budget and threshold view;
- policy preview and decision view;
- intervention result view;
- event history and proof metric view;
- platform capability matrix;
- manual-required and unavailable state explanations;
- export/delete/retention state where applicable.

## Display Rules

The UI must distinguish:

- domain-known versus IP-only;
- process-attributed versus process-unknown;
- DNS observed versus DNS cache versus reverse DNS versus browser-confirmed;
- CDN/shared infrastructure versus platform-specific proof;
- encrypted-content-unavailable versus content-known;
- dry-run preview versus actual action;
- observe-only versus enforced;
- adapter unavailable versus adapter failed;
- manual-required versus unsupported versus not configured.
- deterministic classifier versus analyzer alert versus AI detection;
- risk-budget pressure versus actual policy action;
- signature alert urgency versus adapter authority.

## Required Limitation States

These states must appear when true:

- exact URL unknown;
- exact video unknown;
- page content unknown;
- message content unknown;
- search query unknown;
- hidden destination behind VPN/proxy unknown;
- encrypted DNS hides destination;
- router-only source cannot prove local process;
- shared CDN is ambiguous;
- platform proof missing.
- analyzer alert did not authorize enforcement;
- AI audit is advisory;
- risk budget threshold not configured;
- broker-backed routing not enabled or not proved.

## Network Evidence Drawer

The drawer should show:

- evidence id;
- observed time, first seen, last seen;
- device and child profile refs;
- source adapter and source quality;
- local endpoint;
- remote endpoint;
- protocol/application protocol candidate;
- process/app/browser refs where proved;
- domain evidence refs and attribution state;
- byte/count/duration summary where supported;
- analyzer alert refs where present;
- detection result refs where present;
- AI audit report refs where present;
- risk budget and threshold refs where present;
- evidence grade and confidence;
- uncertainty reason codes;
- must-not-claim list;
- custody and retention labels.

## Cascade View

The cascade view should show:

```text
network candidate
  -> browser check
  -> app/game check
  -> screen check if enabled and needed
  -> local AI queue if evidence bundle exists
  -> risk budget and threshold check
  -> policy decision
  -> adapter status
  -> audit result
```

The UI must not hide skipped checks. Skipped, unavailable, disabled, and
manual-required states are product information.

## Policy Authoring

Policy UI may expose:

- observe;
- warn;
- ask parent;
- time limit;
- block DNS;
- block IP;
- block flow;
- rate limit;
- adjust risk threshold;
- require managed browser;
- trigger screen summary;
- manual review.

Policy UI must keep authority clear:

- parent writes the rule;
- child-device agent evaluates the rule;
- adapter acts only after proof;
- portal does not run capture, policy evaluation, or enforcement.

## Platform Matrix

The matrix should list each platform/capability pair with one of:

- implemented;
- observe-only;
- dry-run-only;
- manual-required;
- permission-required;
- unavailable;
- scaffold-only;
- not implemented.

Each row should link or cite the latest proof artifact when implemented. Missing
proof should be visible, not hidden behind generic "coming soon" language.

## Child-Facing States

Child-facing ask, warn, limit, and block screens must be age-appropriate,
accessible, evidence-minimal, and policy-backed.

They must show only:

- child-safe reason code;
- next action;
- parent rule or approved explanation token;
- remaining wait/limit state where applicable.

They must not show:

- raw domains when the parent rule does not allow it;
- exact URLs;
- packet details;
- private content;
- AI narrative text written for the parent;
- unsupported claims about intent or page content.

Educational guidance is optional content and must cite a parent-authored rule or
approved content token. Network evidence does not generate child education copy
directly.

## Visual Proof Requirements

Parent-facing UI work needs proof for:

- empty state;
- live service-backed populated state;
- stale/degraded state;
- manual-required state;
- policy dry-run state;
- action result state;
- narrow mobile layout where applicable;
- accessibility of evidence grade and limitation labels.
