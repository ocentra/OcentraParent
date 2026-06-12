# V0.5 Screen Intelligence Router Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `V0.5 Screen Intelligence Router Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Core Decision

Do not make VLM the first step. Screen intelligence should route through the cheapest safe evidence path that can answer the policy question.

Required order:

```text
browser/app/network evidence first
managed-browser structured capture second
cheap OCR third
small local VLM only when needed
big local or household-provider VLM only for hard cases
remote/API VLM disabled for raw screenshots by default
```

This order is not a ban on using the selected local model. If the chosen local model can perform the image task, it remains eligible for guided VLM classification after the router decides cheaper evidence is insufficient. Model capability, runtime cost, and quality proof belong to the AI evaluation pass.

## MVP Boundary

The first MVP focuses on capture and routing:

- capture scope contracts;
- managed browser capture;
- active app/window capture;
- selected app/window capture;
- native game/app/launcher/unknown-process triggers;
- encrypted temporary queue;
- deletion proof;
- summary/read-model alignment;
- AI job/result contract rows must map every candidate run to a typed queue job identifier and explicit result status matrix (`accepted`, `denied`, `retry`, `manual-review`) before implementation.

The second pass focuses on AI processing:

- OCR engine selection;
- selected local model image capability proof;
- candidate VLM comparison;
- household provider fallback with child-agent validation;
- detector prompt quality proof;
- model resource scheduling under load.

## Shared Evidence Layer

Screen evidence is not browser-only. It is a shared evidence layer across:

- browser;
- apps;
- native games;
- browser games;
- social/video;
- bypass tools;
- unknown activity;
- tracking/check-in context when needed.

Capture scope decides where it applies. Policy slice decides what the summary means.

## Capture Scope Hierarchy

1. Managed browser page/window capture.
   Best for browser pages, browser games, YouTube, social web, web signup.
2. Active app/window capture.
   Best for native apps, native games, launchers, VPN tools, chat apps.
3. Selected app/window capture.
   Best when parent wants only one app/game monitored.
4. Full display/full screen capture.
   Highest risk. Strict opt-in only.
5. Unsupported / permission-required / protected surface.
   Honest degraded state.

## Route Contract

```ts
type ScreenAnalysisRoute =
  | 'no_screen_needed'
  | 'browser_structured_only'
  | 'browser_dom_and_metadata'
  | 'browser_clipped_screenshot_ocr'
  | 'browser_viewport_vlm'
  | 'os_active_window_ocr'
  | 'os_active_window_vlm'
  | 'selected_app_window_ocr'
  | 'selected_app_window_vlm'
  | 'full_screen_manual_required'
  | 'household_provider_required'
  | 'parent_approved_remote_required'
  | 'unavailable';
```

```ts
type ScreenAnalysisRouteInput = {
  childProfileRef: string;
  deviceRef: string;

  browserEvidenceRef?: string;
  appForegroundEvidenceRef?: string;
  appGameSessionRef?: string;
  networkDigestRef?: string;
  policyAmbiguityRef?: string;
  trackingCheckInRef?: string;

  targetQuestion:
    | 'is_social_media_visible'
    | 'is_video_visible'
    | 'is_game_visible'
    | 'is_school_work_visible'
    | 'is_bypass_tool_visible'
    | 'is_adult_content_visible'
    | 'is_violence_visible'
    | 'is_chat_visible'
    | 'is_unknown_risky_activity'
    | 'extract_small_visible_text'
    | 'confirm_page_category'
    | 'is_payment_or_signup_visible';

  sensitivity: 'low' | 'medium' | 'high';

  parentMode: 'off' | 'observe_only' | 'dry_run' | 'enforcement_eligible';

  allowedCaptureScope: 'managed_browser_only' | 'active_window' | 'selected_app_window' | 'full_screen' | 'none';

  localModelStatusRefs: string[];
  latencyBudgetMs: number;
};
```

## Router Rules

- If managed browser URL/title/DOM answers the question, do not capture screen.
- If app/game/process/session evidence answers the question, do not capture screen.
- If OCR answers the question, do not use VLM.
- If low confidence, use VLM only on the smallest safe crop.
- If VLM is unavailable, return unknown/manual-required instead of remote API.
- If remote/API is requested, allow only parent-approved redacted summaries by default.

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

## Game And App Flow

```text
native game running
  -> app/game evidence detects process/foreground/session
  -> screen trigger fires on app/game foreground start
  -> capture active window if allowed
  -> OCR/VLM guided detector asks:
       is this game visible?
       is it multiplayer/chat?
       is it violent/adult/casino-like?
       is it educational?
       is it a launcher only?
       is there purchase/login/account/signup visible?
  -> summary goes to game policy
  -> policy decides allow/warn/ask/limit/block/manual-required
```

Screen capture helps classify visible game/app activity, but it does not replace app/game evidence.

Good evidence shape:

```text
Foreground evidence: unknown executable.
Screen summary: likely game screen, confidence 0.78.
Policy: unknown games require parent approval.
Action: ask parent / time-limit candidate.
```

For known games:

```text
Foreground evidence: Minecraft process.
Session evidence: 42 min foreground today.
Screen summary: game visible, no high-risk visual signal detected.
Policy: game budget applies.
```

For launchers:

```text
Foreground evidence: Steam launcher.
Screen summary: launcher/store visible, no child game process proof.
Policy: launcher-only state, not active game yet.
```

## Managed Browser Capture

Managed browser can use CDP screenshot capture only for managed browser pages/windows/crops. It must not become full desktop capture.

Preferred browser order:

1. CDP URL/title/meta.
2. DOM visible text extraction with safe limits.
3. Accessibility/selected structured signals where available.
4. CDP clipped screenshot OCR.
5. CDP screenshot VLM.

Use CDP screenshot only when:

- page is canvas/video/game/social feed;
- text is not in DOM;
- visual category is needed;
- DOM is too dynamic;
- platform hides useful info.

## Detector Packs

Use small detector-specific prompts instead of one open-ended prompt:

- `ScreenDetector.SocialFeed`
- `ScreenDetector.Chat`
- `ScreenDetector.VideoPlayer`
- `ScreenDetector.Game`
- `ScreenDetector.SchoolWork`
- `ScreenDetector.AdultContent`
- `ScreenDetector.Violence`
- `ScreenDetector.BypassTool`
- `ScreenDetector.CredentialPrompt`
- `ScreenDetector.PaymentPurchase`
- `ScreenDetector.UnknownActivity`

Every detector returns typed JSON and must not transcribe private messages, names, credentials, or full OCR text by default.
