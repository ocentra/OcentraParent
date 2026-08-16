# V0.5 Screen AI Analysis Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `V0.5 Screen AI Analysis Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Rule

Local OCR/vision output is evidence, not authority.

Do not run a VLM first. The screen intelligence router must try structured evidence and OCR before VLM.

This is not a ban on a capable chosen local model. If the configured parent/device model supports image input and passes local privacy/performance/quality proof, the guided VLM step can use it. Model choice remains a parent/device AI configuration decision, with Gemma-family local runtime expected as the current default until a model plan changes it.

## Required Route Order

```text
0. Existing evidence:
   managed URL, app foreground, title, domain, process, network digest.

1. Browser/app structured extraction:
   DOM/title/meta/URL/platform/process/session evidence.

2. Targeted OCR:
   crop/region/low-res screenshot only.

3. Guided VLM:
   yes/no/multi-label classification from the smallest safe image.

4. Household AI provider mesh:
   heavier VLM only for hard cases when the same-device cheap path is
   uncertain. Provider execution is worker-only; the evidence-owning child
   agent validates the result before policy.

5. Remote/API:
   disabled for raw screenshots by default.
```

## Inputs

- Queued encrypted image job.
- Capture reason.
- Capture scope.
- Foreground app/window evidence ref.
- Browser evidence ref.
- App/game session ref.
- Network digest ref.
- Parent setting version.
- Redaction policy.
- OCR snippet setting.

## Outputs

- Visible activity categories.
- Risk signals.
- Confidence.
- Uncertainty reasons.
- OCR snippets if allowed.
- Redaction notes.
- Summary for parent.
- Image digest.
- Deletion state.
- Source evidence refs.

## AI Cannot

- Enforce policy directly.
- Scan OS/files/network/browser.
- Retain raw image.
- Output unbounded raw OCR text.
- Capture passwords/credentials.
- Bypass protected surfaces.
- Send image to remote AI by default.
- Answer open-ended prompts such as "describe everything on this screen".

## Validation

- Rust validates AI output before storage.
- Invalid JSON becomes invalid/unknown state.
- Missing refs become invalid state.
- Invalid confidence becomes invalid state.
- Unsupported categories become invalid state.
- Raw text overflow becomes invalid/redacted state.
- Deletion state is required.

## Parent Summary Requirements

Summaries must be:

- short;
- parent-readable;
- linked to evidence refs;
- explicit about confidence;
- explicit about uncertainty;
- free of raw screenshot paths;
- free of secrets and credential text.

## OCR Versus VLM

Use OCR for:

- visible text;
- site/app names;
- button labels;
- login/signup page text;
- search terms;
- video titles;
- chat app names;
- warning text;
- school assignment text;
- platform words such as YouTube, Roblox, Discord, Instagram, or Steam.

Use guided VLM for:

- game screen detection;
- video player detection;
- social feed detection;
- chat/messaging screen detection;
- school-work visual detection;
- adult/violent-looking content detection;
- bypass/proxy tool visual detection;
- account signup/payment/purchase visual detection.

Bad prompt:

```text
Describe everything on this screen.
```

Good prompt:

```text
Return JSON only.
Task: classify visible activity category.
Allowed categories: school, video, chat, game, social, shopping, adult_content, violence, bypass_tool, productivity, unknown.
Do not transcribe private text.
Do not include names, messages, credentials, or personal data.
Return confidence and uncertainty reasons.
```

## Model Stack Recommendation

Default path:

```text
structured evidence + OCR
```

OCR path:

```text
PaddleOCR/PP-OCR if packaging works
Tesseract as fallback/simple baseline
```

VLM path:

```text
small local guided classifier only when structured/OCR evidence is insufficient
household AI provider mesh for heavier VLM jobs
remote disabled for raw screenshots
```

Model selection pass:

```text
verify configured Gemma-family model image capability first if it is the parent/device default
evaluate Qwen2.5-VL or other candidates only when Gemma/runtime quality is insufficient
keep model candidates behind same-device or household-provider privacy and
resource proof
do not block capture MVP on final VLM selection
```

Main LLM path:

```text
existing text model reasons over validated summaries
```

## Local AI Job Queue

Only one heavy model job should run at a time on a normal child PC. OCR can run when lightweight and CPU budget allows. VLM jobs must be small, cropped, guided, and rate-limited.

```ts
type LocalAiJobPriority =
  | 'p0_policy_blocking'
  | 'p1_child_current_activity'
  | 'p2_parent_waiting'
  | 'p3_background_screen_summary'
  | 'p4_report_enrichment'
  | 'p5_memory_refresh';
```

```ts
type ScreenAiJob = {
  jobId: string;
  jobType:
    | 'ocr_text_extract'
    | 'guided_vlm_classification'
    | 'browser_structured_extraction'
    | 'policy_summary'
    | 'report_enrichment';

  priority:
    | 'p0_policy_blocking'
    | 'p1_child_current_activity'
    | 'p2_parent_waiting'
    | 'p3_background_summary'
    | 'p4_report';

  inputRef: string;
  imageRef?: string;
  imageDigest?: string;
  cropRegion?: {
    x: number;
    y: number;
    width: number;
    height: number;
  };

  allowedModelKinds: Array<
    'structured_no_model' | 'ocr_light' | 'vlm_small' | 'vlm_household_provider' | 'remote_redacted_only'
  >;

  privacyMode: 'local_only' | 'household_provider_child_validated' | 'parent_approved_remote_redacted';

  timeoutMs: number;
  maxImagePixels?: number;
  maxOcrSnippets?: number;
  status: 'queued' | 'running' | 'completed' | 'timeout' | 'failed' | 'degraded' | 'skipped';
};
```
