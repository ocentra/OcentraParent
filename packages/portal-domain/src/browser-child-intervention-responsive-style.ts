export const BrowserChildInterventionResponsiveStyle = `
@media (max-width: 820px) {
  .ocentra-child-page {
    align-items: stretch;
    padding: 0;
  }

  .ocentra-child-panel {
    border-inline: 0;
    border-radius: 0;
    min-height: 100svh;
  }

  .ocentra-child-layout {
    grid-template-columns: 1fr;
  }

  .ocentra-child-copy-main h1 {
    max-width: 100%;
  }
}

@media (max-width: 760px) {
  .portal-outline-header {
    --portal-outline-header-action-height: 44px;
    --portal-outline-header-action-icon-box: 32px;
    --portal-outline-header-action-icon-image: var(--portal-outline-header-action-icon-box);
    --portal-outline-header-action-width: 46px;

    grid-template-columns:
      var(--portal-outline-header-action-width)
      minmax(10px, 1fr)
      224px
      minmax(10px, 1fr)
      var(--portal-outline-header-action-width);
    min-height: 56px;
  }

  .ocentra-child-outline-header {
    padding-inline: 8px;
  }

  .portal-outline-header__action {
    font-size: 12px;
    grid-template-columns: 1fr;
    inline-size: var(--portal-outline-header-action-width);
    justify-items: center;
    padding: 0;
  }

  .portal-outline-header__action:last-child {
    inline-size: var(--portal-outline-header-action-width);
  }

  .portal-outline-header__action-content {
    column-gap: 0;
    grid-template-columns: var(--portal-outline-header-action-icon-box);
    inline-size: var(--portal-outline-header-action-icon-box);
    justify-content: center;
    justify-items: center;
  }

  .portal-outline-header__action-label {
    display: none;
  }

  .portal-outline-header__action-icon {
    justify-self: center;
  }

  .portal-outline-header__brand {
    --portal-outline-header-brand-logo-size: 54px;
    --portal-outline-header-brand-logo-slot: 50px;

    font-size: 11.5px;
    gap: 6px;
    inline-size: 224px;
    padding: 0 10px;
  }
}

@media (max-width: 640px) {
  .portal-outline-header {
    --portal-outline-header-action-height: 42px;
    --portal-outline-header-action-icon-box: 30px;
    --portal-outline-header-action-icon-image: var(--portal-outline-header-action-icon-box);
    --portal-outline-header-action-width: 44px;

    grid-template-columns:
      var(--portal-outline-header-action-width)
      minmax(8px, 1fr)
      210px
      minmax(8px, 1fr)
      var(--portal-outline-header-action-width);
    min-height: 54px;
  }

  .portal-outline-header__action {
    font-size: 11px;
    inline-size: var(--portal-outline-header-action-width);
  }

  .portal-outline-header__action:last-child {
    inline-size: var(--portal-outline-header-action-width);
  }

  .portal-outline-header__brand {
    --portal-outline-header-brand-logo-size: 52px;
    --portal-outline-header-brand-logo-slot: 48px;

    font-size: 10.5px;
    gap: 6px;
    inline-size: 210px;
    padding: 0 10px;
  }

  .ocentra-child-layout {
    padding: 12px;
  }

  .ocentra-child-copy {
    grid-template-columns: 1fr;
  }

  .ocentra-child-rule-mark {
    min-height: 0;
  }

  .ocentra-child-illustration {
    height: 98px;
  }

  .ocentra-child-copy-main {
    align-content: start;
  }
}

@media (max-width: 360px) {
  .portal-outline-header {
    --portal-outline-header-action-height: 40px;
    --portal-outline-header-action-icon-box: 28px;
    --portal-outline-header-action-icon-image: var(--portal-outline-header-action-icon-box);
    --portal-outline-header-action-width: 40px;

    grid-template-columns:
      var(--portal-outline-header-action-width)
      minmax(6px, 1fr)
      188px
      minmax(6px, 1fr)
      var(--portal-outline-header-action-width);
    min-height: 50px;
  }

  .portal-outline-header__action {
    inline-size: var(--portal-outline-header-action-width);
  }

  .portal-outline-header__action:last-child {
    inline-size: var(--portal-outline-header-action-width);
  }

  .portal-outline-header__brand {
    --portal-outline-header-brand-logo-size: 48px;
    --portal-outline-header-brand-logo-slot: 44px;

    font-size: 9.5px;
    gap: 4px;
    inline-size: 188px;
    padding: 0 8px;
  }
}

@media (min-width: 821px) and (max-height: 760px) {
  .ocentra-child-next-step,
  .ocentra-child-detail-row:last-child {
    display: none;
  }
}
`;
