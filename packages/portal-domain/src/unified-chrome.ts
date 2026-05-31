export const PortalUnifiedChrome = {
  Tags: {
    Footer: 'footer',
    Image: 'img',
  },
  Classes: {
    Shell: 'portal-unified-shell',
    ShellWork: 'portal-shell-work',
    OutlineHeader: 'portal-outline-header',
    OutlineHeaderAction: 'portal-outline-header__action',
    OutlineHeaderActionIcon: 'portal-outline-header__action-icon',
    OutlineHeaderActionIconImage: 'portal-outline-header__action-icon-image',
    OutlineHeaderActionLabel: 'portal-outline-header__action-label',
    OutlineHeaderBrand: 'portal-outline-header__brand',
    OutlineHeaderBrandLogo: 'portal-outline-header__brand-logo',
    OutlineHeaderBrandLogoMount: 'portal-outline-header__brand-logo-mount',
    OutlineHeaderBrandLogoSpinner: 'portal-outline-header__brand-logo-spinner',
    OutlineHeaderConnector: 'portal-outline-header__connector',
    OutlineHeaderConnectorBox: 'portal-outline-header__connector-box',
    OutlineHeaderConnectorSvg: 'portal-outline-header__connector-svg',
    OutlineHeaderFrame: 'portal-outline-header__frame',
    OutlineHeaderFrameLine: 'portal-outline-header__frame-line',
    OutlineHeaderFrameOuter: 'portal-outline-header__frame-outer',
    OutlineHeaderFrameSegmentGroup: 'portal-outline-header__frame-segment-group',
    OutlineHeaderFrameSvg: 'portal-outline-header__frame-svg',
    OutlineHeaderBrandPart: 'portal-outline-header__brand-part',
    OutlineHeaderBrandPartMuted: 'portal-outline-header__brand-part-muted',
    Footer: 'oc-unified-footer',
    FooterBar: 'oc-unified-footer__bar',
    FooterContent: 'oc-unified-footer__content',
    FooterText: 'oc-unified-footer__text',
    FooterHeart: 'oc-unified-footer__heart',
    FooterLink: 'oc-unified-footer__link',
    FooterVersion: 'oc-unified-footer__version',
  },
  Attributes: {
    HeaderLogoLoading: 'data-oc-header-logo-loading',
    ShellHeaderExtension: 'data-oc-shell-header-extension',
  },
  Alt: {
    DecorativeImage: '',
  },
  Svg: {
    AnchorKeyInner: 'innerAnchor',
    AnchorKeyOuter: 'outerAnchor',
    BottomTabDown: 'down',
    FillNone: 'none',
    FrameColorCyan: '#2fddff',
    FrameKeyOuter: 'outerFrame',
    FrameLineVariant: {
      Line: 'line',
      Outline: 'outline',
    },
    FrameOrientationLandscape: 'landscape',
    HeaderConnectorViewBox: '0 0 100 44',
    PointerEventsNone: 'none',
    PreserveAspectRatioNone: 'none',
    StrokeLinejoinRound: 'round',
    VectorEffectNonScalingStroke: 'non-scaling-stroke',
  },
  Version: {
    App: '0.1.1',
  },
} as const;

export const PortalAssets = {
  AnonymousProfile: '/header-login.svg',
  HeaderHomeIcon: '/nav-overview.svg',
  HeaderLoginIcon: '/header-login.svg',
  HeaderLogo: '/ocentra-logo.svg',
} as const;

export const PortalExternalLinks = {
  Ocentra: 'https://ocentra.ca',
  BlankTarget: '_blank',
  ExternalRel: 'noopener noreferrer',
} as const;
