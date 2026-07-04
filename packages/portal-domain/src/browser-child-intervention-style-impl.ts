import { BrowserChildInterventionResponsiveStyle } from './browser-child-intervention-responsive-style';
import { BrowserChildInterventionStyleBase } from './browser-child-intervention-style-base';

export const BrowserChildInterventionPageStyle = `${BrowserChildInterventionStyleBase}
${BrowserChildInterventionResponsiveStyle}

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
  font-size: 0.93rem;
  font-weight: 780;
  justify-content: center;
  letter-spacing: .02em;
  line-height: 1.05;
  min-width: 0;
  text-align: center;
  text-shadow: var(--portal-outline-header-label-shadow);
  text-transform: uppercase;
  word-break: normal;
}

.portal-outline-header__brand {
  align-content: center;
  box-shadow:
    inset 0 0 0 1px rgb(255 255 255 / 6%),
    0 0 0 1px rgb(47 221 255 / 8%),
    0 0 14px rgb(47 221 255 / 16%);
  display: grid;
  gap: 0;
  grid-template-columns: auto auto auto;
  justify-content: center;
  justify-items: center;
  padding-inline: 10px;
}

.portal-outline-header__brand-logo {
  display: block;
  height: 42px;
  width: 42px;
}

.portal-outline-header__brand-logo-mount {
  align-items: center;
  background: var(--portal-outline-header-brand-logo-bg);
  border: 1px solid var(--portal-outline-header-brand-logo-border);
  border-radius: 3px;
  box-shadow: var(--portal-outline-header-brand-logo-shadow);
  display: inline-grid;
  height: 42px;
  justify-items: center;
  width: 42px;
}

.portal-outline-header__brand-part {
  color: var(--portal-outline-header-brand-text);
  font-size: 1.18rem;
  font-weight: 800;
  letter-spacing: .08em;
  line-height: 1;
  text-shadow: var(--portal-outline-header-brand-text-shadow);
  text-transform: uppercase;
}

.portal-outline-header__brand-part--muted {
  color: var(--portal-outline-header-brand-text);
  opacity: .86;
  text-shadow: var(--portal-outline-header-brand-text-shadow);
}
`;
