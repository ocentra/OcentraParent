# Pasted Content Coverage Audit

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `Pasted Content Coverage Audit`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This audit records the final read-through of the browser-plan pasted content.
The source attachments were consolidated into repo-owned plan docs instead of
copied as unmanaged notes. The attachment map below records documentation
landing points only; it does not prove that the mapped workpacks, checklist
rows, or proof artifacts are complete in this checkout.

## Attachment Map

| Attachment                             | Source Theme                                                                | Covered By                                                                                                                                                                                                                                                    | Coverage Notes                                                                                                                                                                              |
| -------------------------------------- | --------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fc988e85-8842-42da-b7a7-b111e19f0bf4` | Managed Browser Control Full Scope Plan                                     | [README](README.md), [source index](source-index.md), [current snapshot](current-browser-snapshot.md), [full scope plan](v0-5-managed-browser-full-scope-plan.md), [test blueprint](v0-5-managed-browser-test-blueprint.md), [workpacks](workpacks/)          | Documented. The original 24-workpack idea was normalized to the current 24 browser workpacks. The pasted managed Chrome/Edge policy-writer scope is explicitly carried in workpacks 15 and 16. |
| `fae49da5-3d40-4a0a-b158-fbae5466089e` | Managed Browser Test Blueprint                                              | [test blueprint](v0-5-managed-browser-test-blueprint.md), [implementation checklist](implementation-checklist.md), [workpack 23](workpacks/23-e2e-and-manual-proof-artifacts.md), [workpack 24](workpacks/24-rollout-checklist-and-pr-gate.md)                | Documented. The implementation checklist now requires proof packs, command logs, raw evidence JSON, UI screenshots, security-negative proof, and manual-platform proof.                     |
| `8ad36886-78d5-45e5-9c63-9ac444f4d7dc` | Browser URL And Video AI Intelligence Plan                                  | [URL/video AI intelligence plan](v0-5-browser-url-video-ai-intelligence-plan.md), [full scope plan](v0-5-managed-browser-full-scope-plan.md), [UI/UX guide](ui-ux-requirements-guide.md), [implementation checklist](implementation-checklist.md)             | Documented. The 25 enhancement split is tracked as `AI-01` through `AI-25` in the main checklist.                                                                                           |
| `a6f35b03-7f79-4cc1-8585-661f0a4ff266` | Browser URL And Video AI Intelligence Plan With Repo Boundary Notes         | [URL/video AI intelligence plan](v0-5-browser-url-video-ai-intelligence-plan.md), [source index](source-index.md), [feature docs linked from source index](source-index.md)                                                                                   | Documented. This duplicate/expanded source was merged with the URL/video AI plan while preserving the rule that AI is evidence, parent policy decides, and enforcement requires adapter proof. |
| `2d5e9e42-bbff-4d55-9ea0-b05eca5b6c08` | Social Media, Account Creation, Feed, Short-Video, And Platform Gating Plan | [social platform/account/feed gating plan](v0-5-social-platform-account-feed-gating-plan.md), [full scope plan](v0-5-managed-browser-full-scope-plan.md), [UI/UX guide](ui-ux-requirements-guide.md), [implementation checklist](implementation-checklist.md) | Documented. The 24 social enhancement split is tracked as `SOCIAL-01` through `SOCIAL-24` in the main checklist.                                                                            |
| `e114d402-2da9-4250-bc22-93a47e38c4b6` | Social Platform Plan With External Context                                  | [social platform/account/feed gating plan](v0-5-social-platform-account-feed-gating-plan.md), [source index](source-index.md)                                                                                                                                 | Documented. External platform context was retained as planning context without turning platform claims into product claims.                                                                 |
| `61871d5b-6dfc-449f-aa36-5b194da02f08` | Browser Games, Cloud Gaming, WebGL Games, And Game Portal Gating Plan       | [browser games/cloud gaming plan](v0-5-browser-games-cloud-gaming-gating-plan.md), [full scope plan](v0-5-managed-browser-full-scope-plan.md), [UI/UX guide](ui-ux-requirements-guide.md), [implementation checklist](implementation-checklist.md)            | Documented. The 24 browser-game enhancement split is tracked as `GAME-01` through `GAME-24` in the main checklist.                                                                          |
| `f3f3baa3-eb95-4c6f-923f-5492e3f6a809` | Browser Game Plan With External Context                                     | [browser games/cloud gaming plan](v0-5-browser-games-cloud-gaming-gating-plan.md), [source index](source-index.md)                                                                                                                                            | Documented. Browser/cloud/UGC game context was retained while keeping native games under app/game evidence boundaries.                                                                      |

## Coverage Checklist

Audit note (2026-06-16): the checklist below remains open in this checkout. Use
this file to track doc landing points and unresolved coverage gaps, not to
infer runtime completion.

- [ ] Managed browser proves exact URL/tab.
- [ ] Unmanaged browser proves bypass/process use only.
- [ ] Network/domain proves destination only, not exact page or active tab.
- [ ] Extension/native host is optional helper, not foundation.
- [ ] Browser inventory includes family, product, channel, install type,
      executable/path ref, package family, signature/hash ref, process state,
      management tier, capability state, and reason codes.
- [ ] Windows inventory includes registry, known paths, Start Menu shortcuts,
      AppX/MSIX packages, running processes, signature/hash, and default handler
      posture.
- [ ] macOS, Linux, Android, iOS, Safari, Firefox, mobile, and extension paths
      stay platform-specific/manual-required until real proof exists.
- [ ] Managed profile store, launcher, loopback-only bridge, CDP `/json/version`
      and `/json/list`, target evidence, and active-tab uncertainty are represented.
- [ ] Bridge custody rejects default profile, wrong profile, wrong process,
      wrong port, non-loopback endpoint, stale session, malformed target, and raw
      debugger URL leakage.
- [ ] Journal and SQLite/read-model proof comes before portal, policy, or AI
      consumers.
- [ ] Portal UI requirements include managed sessions, current tab evidence,
      active-state labels, unmanaged browser use, degraded/manual-required states,
      policy preview, intervention rows, and malicious/long-value layout safety.
- [ ] Managed intervention covers observe, dry-run, warn, redirect/block,
      child-facing hold screens, journal/action refs, and portal proof.
- [ ] Windows AppLocker/App Control stays real-platform/manual-proof gated.
- [ ] Managed Chrome/Edge policy writer inputs are explicit: incognito, guest,
      profile adding, history deletion where supported, safe search, restricted
      mode, and URL allow/block lists.
- [ ] URL/video AI pipeline includes URL shape, memory lookup, metadata,
      hidden managed analysis load, AI result, provider route, policy evaluator,
      post-analysis action, queue/scheduling, cache/knowledge graph, and degraded
      states.
- [ ] URL/video examples include YouTube, Shorts, Vimeo, TikTok, Instagram,
      Facebook, Twitch, X/Twitter, Reddit, Discord, livestreams, multilingual
      content, self-harm, misinformation, rage-bait, and educational video.
- [ ] Social platform scope includes Facebook, Instagram, Messenger, TikTok,
      Snapchat, YouTube, YouTube Shorts, Vimeo, Twitch, Discord, Reddit, X/Twitter,
      Pinterest, Roblox/social-game surfaces, unknown social sites, fake/new
      accounts, feeds/reels/shorts, livestreams, messaging/contact risk,
      upload/post, parent approvals, AI, memory/cache, and audit proof.
- [ ] Social privacy boundaries preserve permission/source/confidence and do
      not claim private-message reading, native per-reel blocking, raw screenshot
      storage, or platform teen-setting trust without separate proof.
- [ ] Browser-game scope includes browser portals, HTML5/WebGL/canvas/iframe
      games, unblocked school sites, educational games, Roblox web flows, Minecraft
      account/launch flows, cloud gaming, Xbox Cloud Gaming, GeForce NOW, Amazon
      Luna, Boosteroid, Steam remote/web surfaces, itch.io, CrazyGames, Poki,
      Miniclip, Coolmath Games, Kongregate-style portals, microtransactions, loot
      boxes, multiplayer chat, UGC games, unsafe content, AI, approvals, time
      budgets, and proof.
- [ ] Browser-game evidence includes URL/domain shape, runtime signals,
      metadata, hidden analysis safety, optional screen/OCR/vision summary, network
      limits, cloud-gaming ambiguity, and degraded states for canvas/iframe/cloud
      surfaces.
- [ ] Test/proof expectations include unit, integration, contract, adapter
      fixture, security, persistence, E2E, Playwright UI, manual browser/platform,
      performance, and CI proof gates.
- [ ] Required proof artifacts include source snapshots, raw JSON/logs,
      journal/read-model output, policy/action refs, UI screenshots, child-facing
      screenshots where actions occur, security-negative proof, and manual platform
      evidence.

## Consolidation Decisions

- The pasted managed-browser base split remains 24 workpacks, but macOS/Linux
  adapter specifics are consolidated into cross-platform matrix workpack 05
  until there is real implementation proof.
- The pasted managed Chrome/Edge policy-writer workpack is carried by workpacks
  15 and 16 so policy authoring and compiler ownership stay together.
- URL/video AI, social/account/feed gating, and browser-game/cloud-gaming are
  tracked as enhancement checklists inside the main checklist rather than added
  as 73 extra base workpack files.
- External research context is kept in the enhancement docs as planning context;
  product claims still require repo proof.
