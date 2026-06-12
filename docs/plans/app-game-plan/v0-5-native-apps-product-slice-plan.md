# V0.5 Native Apps Product Slice Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `V0.5 Native Apps Product Slice Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Native app product meaning is separate from native game product meaning even
though both use the shared app/game evidence spine.

## Scope

Native app scope includes:

- productivity apps;
- school apps;
- chat apps;
- social native apps;
- video native apps;
- music apps;
- utility apps;
- VPN, proxy, tunnel, DNS changer, and Tor apps;
- remote desktop and screen sharing apps;
- download manager and torrent apps;
- AI/chatbot and local LLM apps;
- developer tools;
- creative and office apps;
- email apps;
- store, installer, and updater apps;
- system and security apps;
- unknown apps;
- portable apps;
- accessibility-sensitive apps.

Browser pages and browser games remain outside this product slice.

## App Categories

Native app category candidates include:

- school;
- productivity;
- browser;
- social;
- messaging;
- video;
- music;
- AI chatbot;
- developer tool;
- creative;
- office;
- email;
- remote desktop;
- VPN/proxy;
- download/torrent;
- file sharing;
- store/installer;
- system;
- security;
- settings;
- unknown.

Categories are policy inputs with source/confidence, not automatic decisions.

## App Policy Targets

Native app policy targets include:

- specific app identity;
- package id;
- bundle id;
- AppUserModelId;
- desktop entry id;
- executable hash;
- publisher signature;
- category;
- unknown apps;
- newly installed apps;
- portable apps;
- VPN/proxy apps;
- remote desktop apps;
- download/torrent apps;
- AI/chatbot apps;
- all non-system apps.

## App Actions

Native app actions include:

- allow;
- observe;
- warn;
- ask parent;
- time limit;
- bonus time;
- block launch;
- terminate running;
- hide app;
- suspend app;
- shield app;
- require parent approval;
- manual required;
- unavailable.

Every action must carry capability state and authority tier. Strong actions stay
manual-required until platform proof exists.

## Unknown And New App Flow

New or unknown app candidates may arise from:

- new inventory row;
- new executable observed;
- new package installed;
- new Start Menu shortcut;
- new desktop entry;
- new app bundle;
- new Android package;
- new iOS token or MDM app query row;
- portable executable launched;
- installer/updater ran.

Parent options should include allow once, allow this app, allow this category,
ask child why, block if supported, or report only. Unsupported or unproved block
paths must render manual-required.

## Risk App Handling

Risk candidates include VPN/proxy, remote desktop, torrent/download,
installer/updater, AI/chatbot, messaging, social, video, and unknown-risk apps.
Risk is a candidate with source/confidence and evidence refs. It is not a direct
enforcement decision.

## Parent UI Requirements

The parent UI must show:

- installed/detectable apps;
- running apps;
- foreground active app;
- recent app sessions;
- daily rollups;
- unknown/new apps;
- risk app candidates;
- app approval requests;
- app rules and schedule outcomes;
- capability/platform status;
- evidence details and audit timeline.

The UI must not expose raw private executable paths, command lines with secrets,
chat/message content, keystrokes, screenshots, launcher credentials, or
decrypted network payloads.

## Done Signal

The native app product slice is credible when parents can see app presence,
running state, foreground state, duration, unknown/risk state, policy outcome,
and capability status without confusing weak evidence with authority.
