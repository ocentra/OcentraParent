import { BrowserChildInterventionResponsiveStyle } from './browser-child-intervention-responsive-style';

export const BrowserChildInterventionPageStyle = `
:root {
  color-scheme: dark light;
  --ocp-bg: #08111f;
  --ocp-bg-band: #0e2b3b;
  --ocp-bg-base-start: #07111f;
  --ocp-bg-base-mid: #0a1a2f;
  --ocp-bg-base-end: #081423;
  --ocp-bg-vignette: #02070d;
  --ocp-bg-warm: #f3efb0;
  --ocp-bg-mint: #a9efd8;
  --ocp-bg-aqua: #98ebee;
  --ocp-panel: #f7fbff;
  --ocp-panel-strong: #ffffff;
  --ocp-text: #132235;
  --ocp-muted: #5b7187;
  --ocp-line: #cfddea;
  --ocp-line-strong: #9eb7cb;
  --ocp-brand: #056b86;
  --ocp-brand-strong: #0d526a;
  --ocp-accent: #147a5e;
  --ocp-warn: #9b6400;
  --ocp-danger: #b42334;
  --ocp-focus: #0b84a5;
  --ocp-shadow: 0 24px 80px rgb(9 26 42 / 24%);
  --portal-app-background: #020814;
  --portal-frame-material-fill-alpha: 0.62;
  --portal-frame-surface-fill: rgb(4 22 36 / var(--portal-frame-material-fill-alpha));
  --portal-outline-header-action-content-hover-filter: drop-shadow(0 0 4px rgb(255 255 255 / 54%))
    drop-shadow(0 0 9px rgb(47 221 255 / 42%));
  --portal-outline-header-action-content-hover-scale: 1.06;
  --portal-outline-header-brand-logo-bg: rgb(2 10 18 / 78%);
  --portal-outline-header-brand-logo-border: rgb(120 238 255 / 82%);
  --portal-outline-header-brand-logo-shadow:
    0 0 0 1px rgb(217 247 231 / 22%), 0 0 7px rgb(47 221 255 / 54%), 0 0 14px rgb(47 221 255 / 22%);
  --portal-outline-header-brand-text: #ffffff;
  --portal-outline-header-brand-text-shadow: none;
  --portal-outline-header-brand-text-stroke: rgb(0 68 255 / 92%);
  --portal-outline-header-connector-fill: var(--portal-frame-surface-fill);
  --portal-outline-header-connector-filter: none;
  --portal-outline-header-connector-stroke: #2fddff;
  --portal-outline-header-frame-fill: var(--portal-frame-surface-fill);
  --portal-outline-header-frame-line-stroke: #2fddff;
  --portal-outline-header-icon-filter: drop-shadow(0 0 2px rgb(255 255 255 / 34%))
    drop-shadow(0 0 7px rgb(47 221 255 / 34%)) drop-shadow(0 0 10px rgb(255 64 180 / 14%));
  --portal-outline-header-label-shadow: 0 0 4px rgb(47 221 255 / 18%);
  --portal-outline-header-text: #edf8ff;
}

@media (prefers-color-scheme: dark) {
  :root {
    --ocp-bg: #06111e;
    --ocp-bg-band: #10293a;
    --ocp-bg-base-start: #07111f;
    --ocp-bg-base-mid: #0a1a2f;
    --ocp-bg-base-end: #081423;
    --ocp-bg-vignette: #02070d;
    --ocp-bg-warm: #f3efb0;
    --ocp-bg-mint: #a9efd8;
    --ocp-bg-aqua: #98ebee;
    --ocp-panel: #0d1c2b;
    --ocp-panel-strong: #12283a;
    --ocp-text: #f3f8fd;
    --ocp-muted: #a8bdd0;
    --ocp-line: #25465d;
    --ocp-line-strong: #3b7189;
    --ocp-brand: #65d8f0;
    --ocp-brand-strong: #9cecff;
    --ocp-accent: #54d69f;
    --ocp-warn: #ffd166;
    --ocp-danger: #ff8fa0;
    --ocp-focus: #85e7ff;
    --ocp-shadow: 0 28px 90px rgb(0 0 0 / 44%);
  }
}

:root[data-ocentra-theme='dark'] {
  color-scheme: dark;
  --ocp-bg: #06111e;
  --ocp-bg-band: #10293a;
  --ocp-bg-base-start: #07111f;
  --ocp-bg-base-mid: #0a1a2f;
  --ocp-bg-base-end: #081423;
  --ocp-bg-vignette: #02070d;
  --ocp-bg-warm: #f3efb0;
  --ocp-bg-mint: #a9efd8;
  --ocp-bg-aqua: #98ebee;
  --ocp-panel: #0d1c2b;
  --ocp-panel-strong: #12283a;
  --ocp-text: #f3f8fd;
  --ocp-muted: #a8bdd0;
  --ocp-line: #25465d;
  --ocp-line-strong: #3b7189;
  --ocp-brand: #65d8f0;
  --ocp-brand-strong: #9cecff;
  --ocp-accent: #54d69f;
  --ocp-warn: #ffd166;
  --ocp-danger: #ff8fa0;
  --ocp-focus: #85e7ff;
  --ocp-shadow: 0 28px 90px rgb(0 0 0 / 44%);
}

:root[data-ocentra-theme='light'] {
  color-scheme: light;
  --ocp-bg-base-start: #dcebfb;
  --ocp-bg-base-mid: #c5ddf7;
  --ocp-bg-base-end: #d7e8fb;
  --ocp-bg-vignette: #b5d0eb;
  --portal-app-background: #f3f7fb;
  --portal-frame-material-fill-alpha: 0.58;
  --portal-outline-header-action-content-hover-filter: drop-shadow(0 1px 1px rgb(0 8 14 / 58%))
    drop-shadow(0 0 5px rgb(255 211 106 / 32%));
  --portal-outline-header-brand-logo-bg: rgb(2 10 18 / 78%);
  --portal-outline-header-brand-logo-border: rgb(9 84 114 / 58%);
  --portal-outline-header-brand-logo-shadow: 0 0 0 1px rgb(255 255 255 / 60%), 0 4px 10px rgb(9 54 78 / 18%);
  --portal-outline-header-brand-text: #f7fcff;
  --portal-outline-header-brand-text-shadow: 0 1px 1px rgb(0 8 14 / 72%), 0 0 5px rgb(2 23 38 / 58%);
  --portal-outline-header-brand-text-stroke: rgb(2 23 38 / 76%);
  --portal-outline-header-icon-filter: drop-shadow(0 1px 1px rgb(0 8 14 / 42%))
    drop-shadow(0 0 3px rgb(255 211 106 / 14%));
  --portal-outline-header-label-shadow: 0 1px 1px rgb(0 8 14 / 52%), 0 0 3px rgb(7 53 75 / 34%);
  --portal-outline-header-text: #f7fcff;
}

@media (prefers-color-scheme: light) {
  :root:not([data-ocentra-theme='dark']) {
    --portal-app-background: #f3f7fb;
    --portal-frame-material-fill-alpha: 0.58;
    --portal-outline-header-action-content-hover-filter: drop-shadow(0 1px 1px rgb(0 8 14 / 58%))
      drop-shadow(0 0 5px rgb(255 211 106 / 32%));
    --portal-outline-header-brand-logo-bg: rgb(2 10 18 / 78%);
    --portal-outline-header-brand-logo-border: rgb(9 84 114 / 58%);
    --portal-outline-header-brand-logo-shadow: 0 0 0 1px rgb(255 255 255 / 60%), 0 4px 10px rgb(9 54 78 / 18%);
    --portal-outline-header-brand-text: #f7fcff;
    --portal-outline-header-brand-text-shadow: 0 1px 1px rgb(0 8 14 / 72%), 0 0 5px rgb(2 23 38 / 58%);
    --portal-outline-header-brand-text-stroke: rgb(2 23 38 / 76%);
    --portal-outline-header-icon-filter: drop-shadow(0 1px 1px rgb(0 8 14 / 42%))
      drop-shadow(0 0 3px rgb(255 211 106 / 14%));
    --portal-outline-header-label-shadow: 0 1px 1px rgb(0 8 14 / 52%), 0 0 3px rgb(7 53 75 / 34%);
    --portal-outline-header-text: #f7fcff;
  }
}

* {
  box-sizing: border-box;
}

html,
body {
  min-height: 100%;
}

body {
  background: var(--ocp-bg-base-end);
  color: var(--ocp-text);
  font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  margin: 0;
  min-width: 320px;
  overflow-x: hidden;
  position: relative;
}

.ocentra-child-site-backdrop {
  background: #05080d;
  inset: 0;
  overflow: hidden;
  pointer-events: none;
  position: fixed;
  z-index: 0;
}

.ocentra-child-site-backdrop img {
  display: block;
  filter: blur(5px) saturate(.9) brightness(.72);
  height: 100%;
  object-fit: cover;
  opacity: .9;
  transform: scale(1.018);
  width: 100%;
}

.ocentra-child-background {
  inset: 0;
  overflow: hidden;
  pointer-events: none;
  position: fixed;
  z-index: 0;
}

.ocentra-child-background-svg {
  opacity: 1;
}

body[data-ocentra-site-backdrop='true'] {
  background: #05080d;
}

body[data-ocentra-site-backdrop='true'] .ocentra-child-background {
  z-index: 1;
}

body[data-ocentra-site-backdrop='true'] .ocentra-child-background-svg-site {
  filter: grayscale(1) saturate(.14) brightness(1.16) contrast(.88);
  mix-blend-mode: soft-light;
  opacity: .42;
}

.ocentra-child-background-svg-light {
  display: none !important;
}

@media (prefers-color-scheme: light) {
  :root:not([data-ocentra-theme='dark']) .ocentra-child-background-svg-dark {
    display: none !important;
  }

  :root:not([data-ocentra-theme='dark']) .ocentra-child-background-svg-light {
    display: block !important;
  }
}

:root[data-ocentra-theme='light'] .ocentra-child-background-svg-dark {
  display: none !important;
}

:root[data-ocentra-theme='light'] .ocentra-child-background-svg-light {
  display: block !important;
}

:root[data-ocentra-theme='dark'] .ocentra-child-background-svg-dark {
  display: block !important;
}

:root[data-ocentra-theme='dark'] .ocentra-child-background-svg-light {
  display: none !important;
}

button,
textarea {
  font: inherit;
}

button {
  cursor: pointer;
}

.ocentra-child-page {
  align-items: center;
  display: grid;
  min-height: 100svh;
  padding: clamp(14px, 3.5vw, 40px);
  position: relative;
  z-index: 2;
}

.ocentra-child-panel {
  background:
    linear-gradient(180deg, color-mix(in srgb, var(--ocp-panel-strong) 92%, transparent), var(--ocp-panel)),
    var(--ocp-panel);
  border: 1px solid var(--ocp-line);
  border-radius: 8px;
  box-shadow: var(--ocp-shadow);
  margin: 0 auto;
  max-width: 1120px;
  overflow: hidden;
  position: relative;
  width: 100%;
}

body[data-ocentra-site-backdrop='true'] .ocentra-child-panel {
  background:
    linear-gradient(180deg, color-mix(in srgb, var(--ocp-panel-strong) 84%, transparent), transparent 66%),
    color-mix(in srgb, var(--ocp-panel) 84%, transparent);
  backdrop-filter: blur(10px) saturate(1.08);
  border-color: color-mix(in srgb, var(--ocp-line) 82%, transparent);
  box-shadow:
    0 26px 86px rgb(0 0 0 / 46%),
    inset 0 0 0 1px rgb(255 255 255 / 5%);
}

.ocentra-child-panel::before {
  background: linear-gradient(90deg, var(--ocp-brand), var(--ocp-accent), var(--ocp-warn));
  content: "";
  height: 5px;
  inset: 0 0 auto;
  position: absolute;
}

.portal-outline-header {
  --portal-outline-header-action-height: 44px;
  --portal-outline-header-action-icon-box: 34px;
  --portal-outline-header-action-icon-image: var(--portal-outline-header-action-icon-box);
  --portal-outline-header-action-width: 112px;
  --portal-outline-header-action-label-gap: 5px;

  align-items: center;
  background: transparent;
  color: var(--portal-outline-header-text);
  display: grid;
  gap: 0;
  grid-template-columns:
    var(--portal-outline-header-action-width)
    minmax(20px, 1fr)
    292px
    minmax(20px, 1fr)
    var(--portal-outline-header-action-width);
  justify-content: stretch;
  min-height: 58px;
}

.ocentra-child-outline-header {
  background:
    linear-gradient(90deg, color-mix(in srgb, var(--ocp-brand) 10%, transparent), transparent 60%),
    color-mix(in srgb, var(--ocp-bg-base-end) 72%, transparent);
  border-block-end: 1px solid var(--ocp-line);
  margin: 0;
  min-height: 64px;
  padding: 8px 12px;
}

.portal-outline-header__action,
.portal-outline-header__brand {
  align-items: center;
  background: var(--portal-outline-header-frame-fill);
  border: 1px solid var(--portal-outline-header-frame-line-stroke);
  border-radius: 4px;
  block-size: 56px;
  box-shadow:
    inset 0 0 0 1px rgb(5 86 118 / 72%),
    0 0 10px rgb(47 221 255 / 18%);
  color: inherit;
  display: inline-grid;
  min-height: 56px;
  min-width: 0;
  overflow: visible;
  position: relative;
  z-index: 1;
}

.portal-outline-header__action {
  appearance: none;
  align-self: center;
  block-size: var(--portal-outline-header-action-height);
  border-radius: 4px;
  cursor: pointer;
  font: inherit;
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  font-size: 14px;
  font-weight: 760;
  inline-size: var(--portal-outline-header-action-width);
  justify-content: center;
  justify-items: center;
  min-height: var(--portal-outline-header-action-height);
  padding: 0 6px;
  place-items: center;
}

.ocentra-child-outline-header-back,
.ocentra-child-outline-header-status {
  border-radius: 4px;
}

.portal-outline-header__action:last-child {
  inline-size: var(--portal-outline-header-action-width);
}

.ocentra-child-outline-header-status {
  cursor: default;
  pointer-events: none;
}

.portal-outline-header__action-content {
  align-items: center;
  block-size: calc(var(--portal-outline-header-action-height) - 8px);
  column-gap: var(--portal-outline-header-action-label-gap);
  display: grid;
  grid-template-columns: var(--portal-outline-header-action-icon-box) minmax(0, 1fr);
  inline-size: 100%;
  justify-content: stretch;
  justify-items: stretch;
  line-height: 1;
  position: relative;
  transform-origin: center;
  transition:
    filter 140ms ease,
    transform 140ms ease;
  z-index: 1;
}

.portal-outline-header__action-icon {
  align-items: center;
  align-self: center;
  background: transparent;
  border: 0;
  border-radius: 3px;
  display: inline-grid;
  height: var(--portal-outline-header-action-icon-box);
  justify-items: center;
  justify-self: start;
  position: relative;
  width: var(--portal-outline-header-action-icon-box);
  z-index: 2;
}

.portal-outline-header__action-icon-image {
  display: block;
  filter: var(--portal-outline-header-icon-filter);
  height: var(--portal-outline-header-action-icon-image);
  object-fit: contain;
  transition:
    filter 140ms ease,
    transform 140ms ease;
  width: var(--portal-outline-header-action-icon-image);
}

.portal-outline-header__action-label {
  align-self: center;
  display: flex;
  inline-size: 100%;
  justify-content: center;
  justify-self: stretch;
  min-inline-size: 0;
  position: relative;
  text-align: center;
  text-shadow: var(--portal-outline-header-label-shadow);
  transition:
    color 140ms ease,
    text-shadow 140ms ease;
  z-index: 2;
}

.portal-outline-header__action:hover,
.portal-outline-header__action:focus-visible {
  background: color-mix(in srgb, var(--portal-outline-header-frame-fill) 86%, var(--ocp-brand));
  border-color: var(--ocp-focus);
  color: inherit;
  outline: none;
}

.portal-outline-header__action:hover .portal-outline-header__action-content,
.portal-outline-header__action:focus-visible .portal-outline-header__action-content {
  filter: var(--portal-outline-header-action-content-hover-filter);
  transform: scale(var(--portal-outline-header-action-content-hover-scale));
}

.portal-outline-header__brand {
  --portal-outline-header-brand-logo-size: 56px;
  --portal-outline-header-brand-logo-slot: 54px;

  font-family: Didot, "Bodoni 72", "Bodoni MT", Georgia, serif;
  font-size: 15px;
  font-weight: 650;
  gap: 8px;
  grid-template-columns:
    minmax(0, 1fr)
    var(--portal-outline-header-brand-logo-slot)
    minmax(0, 1fr);
  inline-size: 292px;
  justify-content: stretch;
  line-height: 1;
  padding: 0 14px;
}

.portal-outline-header__brand-logo-mount {
  align-self: stretch;
  display: block;
  inline-size: var(--portal-outline-header-brand-logo-slot);
  justify-self: center;
  position: relative;
  z-index: 2;
}

.portal-outline-header__brand-logo {
  background: var(--portal-outline-header-brand-logo-bg);
  border: 1px solid var(--portal-outline-header-brand-logo-border);
  border-radius: 50%;
  box-shadow: var(--portal-outline-header-brand-logo-shadow);
  display: block;
  height: var(--portal-outline-header-brand-logo-size);
  image-rendering: auto;
  inset-block-start: 50%;
  inset-inline-start: 50%;
  object-fit: contain;
  padding: 2px;
  position: absolute;
  transform: translate(-50%, -48%);
  transition:
    border-color 140ms ease,
    box-shadow 140ms ease,
    filter 140ms ease,
    opacity 180ms ease,
    transform 140ms ease;
  width: var(--portal-outline-header-brand-logo-size);
}

.portal-outline-header__connector {
  align-self: center;
  block-size: calc(var(--portal-outline-header-action-height) / 2);
  background: var(--portal-outline-header-connector-fill);
  border-block: 1px solid var(--portal-outline-header-connector-stroke);
  box-shadow:
    inset 0 0 0 1px rgb(5 86 118 / 52%),
    0 0 8px rgb(47 221 255 / 14%);
  display: block;
  height: calc(var(--portal-outline-header-action-height) / 2);
  margin-block-end: 0;
  margin-inline: -1px;
  min-width: 0;
  pointer-events: none;
  position: relative;
  z-index: 0;
}

.portal-outline-header__connector::before {
  content: none;
}

.portal-outline-header__brand-part,
.portal-outline-header__brand-part-muted {
  -webkit-text-stroke: 0.45px var(--portal-outline-header-brand-text-stroke);
  color: var(--portal-outline-header-brand-text);
  font-feature-settings: "kern" 1;
  inline-size: 100%;
  line-height: 1.3;
  min-width: 0;
  overflow: visible;
  paint-order: stroke fill;
  position: relative;
  text-align: center;
  text-overflow: clip;
  text-shadow: var(--portal-outline-header-brand-text-shadow);
  text-transform: uppercase;
  transition:
    color 140ms ease,
    -webkit-text-stroke-color 140ms ease,
    text-shadow 140ms ease;
  white-space: nowrap;
  z-index: 1;
}

.portal-outline-header__brand-part::first-letter,
.portal-outline-header__brand-part-muted::first-letter {
  font-size: 1.5em;
  line-height: 0.9;
}

.portal-outline-header__brand-part,
.portal-outline-header__brand-part-muted {
  justify-self: stretch;
}

[data-ocentra-intervention-state='blocked'] .ocentra-child-outline-header-status .portal-outline-header__action-label {
  color: var(--ocp-danger);
}

[data-ocentra-intervention-state='warning'] .ocentra-child-outline-header-status .portal-outline-header__action-label,
[data-ocentra-intervention-state='limited'] .ocentra-child-outline-header-status .portal-outline-header__action-label {
  color: var(--ocp-warn);
}

.ocentra-child-layout {
  align-items: stretch;
  display: grid;
  gap: clamp(14px, 2.2vw, 24px);
  grid-template-columns: minmax(0, 1.32fr) minmax(280px, 0.68fr);
  padding: clamp(14px, 2vw, 22px);
}

.ocentra-child-copy {
  align-items: stretch;
  display: grid;
  gap: clamp(14px, 2vw, 22px);
  grid-template-columns: minmax(160px, 0.44fr) minmax(0, 1fr);
  min-width: 0;
}

.ocentra-child-rule-mark {
  align-content: center;
  background:
    radial-gradient(circle at 50% 30%, color-mix(in srgb, var(--ocp-brand) 18%, transparent), transparent 44%),
    linear-gradient(180deg, color-mix(in srgb, var(--ocp-panel-strong) 58%, transparent), transparent);
  border: 1px solid color-mix(in srgb, var(--ocp-line) 78%, transparent);
  border-radius: 8px;
  box-shadow:
    inset 0 0 0 1px color-mix(in srgb, var(--ocp-brand) 8%, transparent),
    0 16px 34px rgb(0 0 0 / 12%);
  display: grid;
  gap: 12px;
  justify-items: center;
  min-height: 100%;
  padding: clamp(14px, 2vw, 18px);
  position: relative;
}

.ocentra-child-rule-mark::before,
.ocentra-child-rule-mark::after {
  background: linear-gradient(90deg, transparent, var(--ocp-brand), transparent);
  content: "";
  height: 1px;
  opacity: .76;
  position: absolute;
  width: 72%;
}

.ocentra-child-rule-mark::before {
  top: 12px;
}

.ocentra-child-rule-mark::after {
  bottom: 12px;
}

.ocentra-child-copy-main {
  align-content: center;
  display: grid;
  gap: clamp(10px, 1.4vw, 14px);
  min-width: 0;
}

.ocentra-child-illustration {
  color: var(--ocp-brand);
  height: clamp(98px, 12vw, 136px);
  width: min(100%, 190px);
}

.ocentra-child-illustration svg {
  display: block;
  height: 100%;
  width: 100%;
}

.ocentra-child-screen,
.ocentra-child-shield,
.ocentra-child-base {
  fill: color-mix(in srgb, var(--ocp-brand) 12%, transparent);
  stroke: currentColor;
  stroke-width: 4;
}

.ocentra-child-screen-line,
.ocentra-child-check {
  fill: none;
  stroke: currentColor;
  stroke-linecap: round;
  stroke-linejoin: round;
  stroke-width: 6;
}

.ocentra-child-check {
  color: var(--ocp-accent);
}

[data-ocentra-intervention-state='blocked'] .ocentra-child-check {
  color: var(--ocp-danger);
}

[data-ocentra-intervention-state='warning'] .ocentra-child-check,
[data-ocentra-intervention-state='limited'] .ocentra-child-check {
  color: var(--ocp-warn);
}

.ocentra-child-status,
.ocentra-child-summary,
.ocentra-child-next-step {
  margin: 0;
}

.ocentra-child-status {
  color: var(--ocp-brand-strong);
  font-size: clamp(0.72rem, 1.2vw, 0.82rem);
  font-weight: 850;
  line-height: 1.2;
  max-width: 19ch;
  text-align: center;
  text-transform: uppercase;
}

.ocentra-child-rule-pill {
  background:
    linear-gradient(180deg, color-mix(in srgb, var(--ocp-brand) 18%, transparent), transparent),
    color-mix(in srgb, var(--ocp-panel-strong) 64%, transparent);
  border: 1px solid color-mix(in srgb, var(--ocp-brand) 54%, var(--ocp-line));
  border-radius: 999px;
  color: var(--ocp-brand-strong);
  display: inline-flex;
  font-size: 0.72rem;
  font-weight: 850;
  justify-content: center;
  letter-spacing: 0;
  line-height: 1;
  max-width: 100%;
  min-height: 28px;
  padding: 8px 12px;
  text-align: center;
  text-transform: uppercase;
}

[data-ocentra-intervention-state='blocked'] .ocentra-child-rule-pill {
  border-color: color-mix(in srgb, var(--ocp-danger) 60%, var(--ocp-line));
  color: var(--ocp-danger);
}

[data-ocentra-intervention-state='warning'] .ocentra-child-rule-pill,
[data-ocentra-intervention-state='limited'] .ocentra-child-rule-pill {
  border-color: color-mix(in srgb, var(--ocp-warn) 70%, var(--ocp-line));
  color: var(--ocp-warn);
}

.ocentra-child-copy-main h1 {
  font-size: clamp(1.82rem, 4vw, 3rem);
  font-weight: 860;
  line-height: 1.03;
  margin: 0;
  max-width: 13ch;
}

.ocentra-child-summary {
  color: var(--ocp-muted);
  font-size: clamp(0.96rem, 1.8vw, 1.08rem);
  line-height: 1.42;
  max-width: 38ch;
}

.ocentra-child-actions {
  align-items: stretch;
  display: grid;
  gap: 10px;
  max-width: 500px;
}

.ocentra-child-request {
  display: grid;
  gap: 10px;
}

.ocentra-child-request label {
  color: var(--ocp-muted);
  font-size: 0.82rem;
  font-weight: 780;
}

.ocentra-child-request textarea {
  background: color-mix(in srgb, var(--ocp-panel-strong) 74%, var(--ocp-bg));
  border: 1px solid var(--ocp-line);
  border-radius: 8px;
  color: var(--ocp-text);
  line-height: 1.4;
  min-height: 74px;
  padding: 12px;
  resize: vertical;
}

.ocentra-child-request textarea:focus {
  border-color: var(--ocp-focus);
  outline: 3px solid color-mix(in srgb, var(--ocp-focus) 24%, transparent);
}

.ocentra-child-primary {
  align-items: center;
  border-radius: 8px;
  display: inline-flex;
  font-weight: 830;
  gap: 8px;
  justify-content: center;
  min-height: 44px;
  padding: 11px 14px;
}

.ocentra-child-primary svg {
  height: 20px;
  width: 20px;
}

.ocentra-child-primary {
  background: var(--ocp-brand-strong);
  border: 1px solid var(--ocp-brand-strong);
  color: var(--ocp-panel-strong);
}

.ocentra-child-primary:hover {
  background: color-mix(in srgb, var(--ocp-brand-strong) 82%, var(--ocp-accent));
}

.ocentra-child-request-status {
  color: var(--ocp-accent);
  font-size: 0.84rem;
  font-weight: 760;
  min-height: 1.2em;
}

.ocentra-child-waiting {
  align-items: center;
  background: color-mix(in srgb, var(--ocp-panel-strong) 68%, transparent);
  border: 1px solid var(--ocp-line);
  border-radius: 8px;
  color: var(--ocp-muted);
  display: flex;
  gap: 10px;
  line-height: 1.4;
  min-height: 52px;
  padding: 12px;
}

.ocentra-child-waiting-dot {
  animation: ocentra-child-pulse 1.2s ease-in-out infinite alternate;
  background: var(--ocp-brand);
  border-radius: 50%;
  flex: 0 0 auto;
  height: 10px;
  width: 10px;
}

.ocentra-child-reason {
  align-self: stretch;
  background:
    linear-gradient(180deg, color-mix(in srgb, var(--ocp-panel-strong) 60%, transparent), transparent),
    color-mix(in srgb, var(--ocp-panel-strong) 58%, var(--ocp-bg));
  border: 1px solid var(--ocp-line);
  border-radius: 8px;
  display: grid;
  gap: 10px;
  padding: clamp(13px, 1.8vw, 18px);
}

.ocentra-child-reason h2 {
  font-size: 1rem;
  line-height: 1.15;
  margin: 0;
}

.ocentra-child-reason dl {
  display: grid;
  gap: 10px;
  margin: 0;
}

.ocentra-child-detail-row {
  border-block-end: 1px solid var(--ocp-line);
  display: grid;
  gap: 4px;
  padding-block-end: 10px;
}

.ocentra-child-detail-row:last-child {
  border-block-end: 0;
  padding-block-end: 0;
}

.ocentra-child-detail-row dt {
  color: var(--ocp-muted);
  font-size: 0.68rem;
  font-weight: 830;
  line-height: 1;
  text-transform: uppercase;
}

.ocentra-child-detail-row dd {
  font-size: 0.86rem;
  line-height: 1.32;
  margin: 0;
  overflow-wrap: anywhere;
}

.ocentra-child-next-step {
  align-self: end;
  color: var(--ocp-muted);
  font-size: 0.86rem;
  line-height: 1.42;
}

@keyframes ocentra-child-pulse {
  from {
    opacity: .45;
    transform: scale(.84);
  }
  to {
    opacity: 1;
    transform: scale(1.1);
  }
}
${BrowserChildInterventionResponsiveStyle}
`;
