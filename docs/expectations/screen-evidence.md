<!-- agent-capsule -->

> Agent Capsule
> Doc: Screen Evidence Analysis Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Screen Evidence Analysis Expectations

Screen evidence is a high-sensitivity local analysis layer. It can help classify
what the child is visibly doing when browser, app/game, and network evidence are
ambiguous, but it must be local-first, disclosed, encrypted while queued, and
summarized into typed evidence before policy or enforcement uses it.

This feature is parent-controlled. Ocentra provides the capability, local privacy
boundary, schema validation, deletion/audit trail, and portal controls; the
parent decides whether to enable it, how often it runs, which triggers are active,
and what household policy should happen from the resulting evidence.

Implementation planning lives in
[Local Screen Evidence Analysis Queue Architecture](../architecture/local-screen-evidence-analysis-queue.md).
That architecture is docs/spec only until a later runtime slice adds contracts,
queue storage, capture, OCR/vision, portal, policy, or enforcement code.

## Outcome Bar

Parent outcome:

- A parent can understand visible activity categories such as game, video,
  school, chat, adult content, violence, bypass tool, or unknown when local
  screen analysis supports that claim.
- A parent can see a local text/JSON summary, confidence, source evidence
  references, and policy result.
- A parent can enable or disable screen analysis, choose supported cadence and
  trigger options, and see the current setting before it affects policy.
- A parent can tell whether the original image was deleted after analysis,
  expired without analysis, or retained only because an explicit future retention
  policy allows it.

Child-device outcome:

- The Rust agent captures screenshots only on the child device or local/LAN
  boundary approved for this product.
- The local OCR/vision model analyzes queued images locally. Screen images do
  not leave the child PC for remote/API AI or cloud processing under this
  feature.
- The Rust agent stores structured summaries and evidence references in the
  journal/query store, then deletes the temporary image after successful
  processing.
- The Rust agent applies policy only after validating structured AI output and
  resolving parent rules.
- The Rust agent treats portal settings as typed parent intent; it does not
  silently enable screen capture because the code path exists.
- Ocentra-hosted services do not store or process child screen images,
  screen-analysis summaries, SQLite evidence, journals, reports, or parent rules
  by default.

## Data Scope

Screen evidence may record:

- Capture timestamp, screen/window/source ids, foreground app evidence id,
  browser evidence id, app/game session id, and network digest id where
  available.
- Encrypted temporary image queue item with TTL and processing status.
- Local OCR/vision summary, visible category candidates, confidence, extracted
  text snippets when allowed, risk signals, redaction notes, and source evidence
  references.
- Image digest/hash and deletion status after processing.

Screen evidence must not record these by default. They require a later explicit
parent-controlled feature and matching technical/privacy boundary:

- Permanent raw screenshot retention.
- Cloud/remote/API AI upload of screenshots.
- Password fields, secure desktop, lock screen, credential prompts, or OS-protected
  surfaces.
- Keystrokes, microphone audio, camera video, decrypted network payloads, browser
  secrets, tokens, or cookies.
- Hidden capture claims. Product UI and docs must disclose that screen analysis
  may run locally.

## Trust Boundary

- Rust owns capture scheduling, encrypted queue storage, TTL, deletion, journal
  writes, query-store ingest, and policy handoff.
- Local AI/OCR reads only queued screen image jobs and source evidence references.
  It does not scan the OS, network, browser, files, or process list itself.
- AI output is structured classification/summary evidence, not enforcement.
- Rust validates AI JSON against schema before storing it or allowing policy to
  consume it.
- Parent rules and deterministic policy decide whether to allow, warn, block,
  time-limit, ask-parent, or do nothing.
- Ocentra-owned defaults may provide conservative capability states and category
  labels, but household actions come from parent-authored rules.
- Parent portal displays summaries, decision evidence, and disclosure; it does
  not capture screenshots.

## Expected Deliverables

- Screen capture capability/status contract.
- Parent opt-in settings contract for enablement, cadence, triggers, strict
  mode, OCR snippets, redaction, TTL, deletion, and policy-use state.
- Screen analysis queue contract with encrypted image reference, TTL, retry
  count, source evidence refs, and status.
- Local screen analysis result contract with summary, categories, confidence,
  risk signals, redaction notes, evidence refs, image digest, and deletion state.
- Local model/runtime status for OCR/vision analysis.
- Policy target contract for visible activity categories and screen-derived risk
  signals.
- Enforcement handoff from validated policy decisions only.
- Parent-control contracts for enablement, cadence, triggers, strict mode,
  retention/deletion behavior, and dry-run versus enforcement.
- Portal summary view that does not expose raw images by default.
- Disclosure copy that accurately explains local-only, parent-controlled screen
  analysis.
- Source/custody labels for child-device temp queue, child-device journal,
  child-device query store, local/LAN service response, parent-device cache,
  parent-owned export, Ocentra-hosted non-activity metadata, and unavailable
  states.

## Runtime Flow

```text
timer or trigger
  -> Rust captures screen/window image
  -> encrypted temporary image queue
  -> local OCR/vision analysis worker
  -> schema-valid JSON summary
  -> Rust validates output
  -> journal and SQLite summary/evidence refs
  -> delete temporary image
  -> deterministic policy evaluator
  -> enforcement adapter or permission request
  -> parent-visible audit result
```

The common local screen-analysis path should be:

1. Rust schedules a screenshot every configured interval, such as five minutes by
   default or one minute in stricter parent-selected mode, and may also capture
   on triggers such as foreground app change, managed browser URL change,
   game/app foreground start, or unusual network activity.
2. Rust skips or records unavailable/degraded status for lock screen, secure
   desktop, password prompts, protected surfaces, or disabled capability states.
3. Rust stores the screenshot as an encrypted temporary queue job with source
   evidence references and a short TTL.
4. Local OCR/vision AI processes the queue and returns structured JSON.
5. Rust validates the JSON schema. Invalid output becomes
   `screen-analysis-invalid` and cannot drive enforcement.
6. Rust stores the text/JSON summary, confidence, categories, evidence refs,
   image digest, and deletion status.
7. Rust deletes the temporary image after successful processing, or after TTL
   expiry if analysis cannot complete.
8. Policy evaluates the screen-derived category/risk together with browser,
   app/game, network, session, and parent rule evidence.
9. Enforcement acts only from a typed policy decision and records the result.

## Scheduling And Retention

- Default cadence should be configurable, with a conservative default such as
  five minutes.
- Strict mode may allow one-minute cadence when the parent explicitly enables it.
- Triggered captures may be more useful than fixed cadence for foreground window,
  managed URL, app/game, or unusual-network changes.
- Parent settings decide whether cadence capture, trigger capture, strict mode,
  and policy use are enabled.
- Temporary images are encrypted at rest while queued.
- Temporary images are deleted after successful local analysis.
- Failed jobs retry only within a short TTL, then delete the image and record
  `screen-analysis-failed` or `screen-analysis-expired`.
- Delete-pending or delete-failed jobs are visible health states and must not
  become silent long-term image retention.
- Stored long-term evidence should be the structured summary, confidence,
  evidence refs, image digest, and deletion state, not the raw image.

## Acceptance

- A local run can queue, analyze, summarize, store, and delete a screenshot
  without sending the image off the child PC.
- The stored summary has schema-valid categories, confidence, evidence refs, and
  deletion state.
- The portal can show current parent-selected settings, who changed them, and
  whether screen analysis is observe-only, dry-run, or enforcement-eligible.
- Policy can consume screen-derived categories in dry-run without enforcing.
- Enforcement, when enabled, requires a typed policy decision and cannot act on
  raw AI text.
- Low-confidence, invalid, failed, expired, or unavailable analysis produces
  unknown, warn, ask-parent, or no-op according to parent rules.
- Parent-facing UI/docs present local screen analysis as an explicit
  parent-controlled option, not a hidden default.

## Done Signal

A local run captures a screen image into an encrypted temporary queue, processes
it with a local OCR/vision model, stores only a schema-valid summary and evidence
references, deletes the image, and lets policy use the summary without any
screen image leaving the child PC.
