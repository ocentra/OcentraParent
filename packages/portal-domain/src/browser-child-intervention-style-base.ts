export const BrowserChildInterventionStyleBase = `
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
`;
