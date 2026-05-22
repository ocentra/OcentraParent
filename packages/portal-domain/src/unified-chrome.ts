export const PortalUnifiedChrome = {
  Tags: {
    Footer: 'footer',
    Image: 'img',
  },
  Classes: {
    Shell: 'portal-unified-shell',
    ShellWork: 'portal-shell-work',
    Header: 'ocentra-game-header',
    HeaderBar: 'ocentra-game-header__bar',
    HeaderHome: 'ocentra-game-header__home',
    HeaderCenter: 'ocentra-game-header__center',
    HeaderLogo: 'ocentra-game-header__logo',
    HeaderTitle: 'ocentra-game-header__title',
    HeaderTitlePart: 'ocentra-game-header__title-part',
    HeaderTitlePartMuted: 'ocentra-game-header__title-part-muted',
    HeaderProfile: 'ocentra-game-header__profile',
    HeaderProfileImage: 'ocentra-game-header__profile-image',
    HeaderNav: 'ocentra-game-header__nav',
    HeaderNavLink: 'ocentra-game-header__nav-link',
    HeaderNavLinkActive: 'ocentra-game-header__nav-link-active',
    HeaderNavLabel: 'ocentra-game-header__nav-label',
    HeaderNavDescription: 'ocentra-game-header__nav-description',
    Footer: 'oc-unified-footer',
    FooterBar: 'oc-unified-footer__bar',
    FooterContent: 'oc-unified-footer__content',
    FooterText: 'oc-unified-footer__text',
    FooterHeart: 'oc-unified-footer__heart',
    FooterLink: 'oc-unified-footer__link',
    FooterVersion: 'oc-unified-footer__version',
  },
  DynamicDataKeys: {
    GameName: 'gameName',
    Tagline: 'tagline',
  },
  HeaderProfile: {
    MainScreen: 'main_screen',
    Contained: 'contained',
  },
  HeaderCenter: {
    ModeA: 'A',
  },
  Version: {
    App: '0.1.1',
  },
} as const;

export const PortalAssets = {
  HeaderLogo: '/ocentra-game-assets/commons/OcentraLogo.svg',
  AnonymousProfile: '/ocentra-game-assets/auth/annon.png',
} as const;

export const PortalExternalLinks = {
  Ocentra: 'https://ocentra.ca',
  BlankTarget: '_blank',
  ExternalRel: 'noopener noreferrer',
} as const;
