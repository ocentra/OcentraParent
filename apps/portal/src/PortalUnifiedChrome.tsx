import type { ReactElement } from 'react';
import {
  PortalAssets,
  PortalDom,
  PortalExternalLinks,
  PortalRoute,
  PortalText,
  PortalTextToken,
  PortalUnifiedChrome,
} from '@ocentra-parent/portal-domain/contracts';

export function UnifiedHeaderChrome({ onAuthOpen }: { readonly onAuthOpen: () => void }): ReactElement {
  return (
    <header className={PortalUnifiedChrome.Classes.Header}>
      <div className={PortalUnifiedChrome.Classes.HeaderBar}>
        <a className={PortalUnifiedChrome.Classes.HeaderHome} href={`${PortalDom.HashPrefix}${PortalRoute.Overview}`}>
          {PortalText.Resolve(PortalTextToken.HeaderHome)}
        </a>
        <HeaderBrand />
        <button
          aria-label={PortalText.Resolve(PortalTextToken.HeaderLogin)}
          className={PortalUnifiedChrome.Classes.HeaderProfile}
          onClick={onAuthOpen}
          type={PortalDom.ButtonType.Button}
        >
          <img
            alt={PortalText.Resolve(PortalTextToken.HeaderLogin)}
            aria-hidden={true}
            className={PortalUnifiedChrome.Classes.HeaderProfileImage}
            src={PortalAssets.AnonymousProfile}
          />
          <span>{PortalText.Resolve(PortalTextToken.HeaderLogin)}</span>
        </button>
      </div>
    </header>
  );
}

export function UnifiedFooterChrome(): ReactElement {
  return (
    <footer className={PortalUnifiedChrome.Classes.Footer}>
      <div className={PortalUnifiedChrome.Classes.FooterBar}>
        <div className={PortalUnifiedChrome.Classes.FooterContent}>
          <span className={PortalUnifiedChrome.Classes.FooterText}>
            <span>{PortalText.Resolve(PortalTextToken.FooterMadeWith)}</span>
            <span className={PortalUnifiedChrome.Classes.FooterHeart}>
              {PortalText.Resolve(PortalTextToken.FooterHeart)}
            </span>
            <span>{PortalText.Resolve(PortalTextToken.FooterBy)}</span>
            <a
              className={PortalUnifiedChrome.Classes.FooterLink}
              href={PortalExternalLinks.Ocentra}
              rel={PortalExternalLinks.ExternalRel}
              target={PortalExternalLinks.BlankTarget}
            >
              {PortalText.Resolve(PortalTextToken.FooterLink)}
            </a>
            <span className={PortalUnifiedChrome.Classes.FooterVersion}>
              {PortalText.Resolve(PortalTextToken.FooterVersion)}
            </span>
          </span>
        </div>
      </div>
    </footer>
  );
}

function HeaderBrand(): ReactElement {
  return (
    <div className={PortalUnifiedChrome.Classes.HeaderCenter}>
      <div className={PortalUnifiedChrome.Classes.HeaderTitle}>
        <span className={PortalUnifiedChrome.Classes.HeaderTitlePart}>
          {PortalText.Resolve(PortalTextToken.HeaderBrandLeft)}
        </span>
        <img
          alt={PortalText.Resolve(PortalTextToken.AppTitle)}
          className={PortalUnifiedChrome.Classes.HeaderLogo}
          src={PortalAssets.HeaderLogo}
        />
        <span className={PortalUnifiedChrome.Classes.HeaderTitlePartMuted}>
          {PortalText.Resolve(PortalTextToken.HeaderBrandRight)}
        </span>
      </div>
    </div>
  );
}
