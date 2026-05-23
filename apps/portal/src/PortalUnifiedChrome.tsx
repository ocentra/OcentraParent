import type { ReactElement, ReactNode } from 'react';
import {
  PortalAssets,
  PortalDom,
  PortalRoute,
  PortalText,
  PortalTextToken,
  PortalUnifiedChrome,
} from '@ocentra-parent/portal-domain/contracts';
import { GameFooter } from '../../../vendor/ocentra-games-core-ui/Footer/GameFooter';
import { createOcentraHeaderLogoConfig } from '../../../vendor/ocentra-games-core-ui/Header/createOcentraHeaderConfig';
import { UnifiedHeader } from '../../../vendor/ocentra-games-core-ui/Header/UnifiedHeader';
import type { UnifiedHeaderConfigInput } from '../../../vendor/ocentra-games-core-ui/Header/UnifiedHeader.config';
import { UnifiedPageShell } from '../../../vendor/ocentra-games-core-ui/Shell/UnifiedPageShell';

type PortalUnifiedShellProps = {
  readonly children: ReactNode;
  readonly onAuthOpen: () => void;
};

const headerLogoConfig = createOcentraHeaderLogoConfig(PortalAssets.HeaderLogo, 45);

function goHome(): void {
  window.location.hash = `${PortalDom.HashPrefix}${PortalRoute.Overview}`;
}

function createHeaderConfig(onAuthOpen: () => void) {
  return {
    left: {
      text: PortalText.Resolve(PortalTextToken.HeaderHome),
      ariaLabel: PortalText.Resolve(PortalTextToken.HeaderHome),
      isButton: true,
      onClick: goHome,
    },
    right: {
      text: PortalText.Resolve(PortalTextToken.HeaderLogin),
      ariaLabel: PortalText.Resolve(PortalTextToken.HeaderLogin),
      isButton: true,
      onClick: onAuthOpen,
    },
    center: {
      mode: PortalUnifiedChrome.HeaderCenter.ModeA,
      contentGap: 14,
      modeA: {
        leftText: PortalText.Resolve(PortalTextToken.HeaderBrandLeft),
        rightText: PortalText.Resolve(PortalTextToken.HeaderBrandRight),
        ...(headerLogoConfig.center?.modeA?.logo === undefined
          ? {}
          : {
              logo: headerLogoConfig.center.modeA.logo,
            }),
      },
    },
    navigation: {
      enabled: false,
    },
  } satisfies UnifiedHeaderConfigInput;
}

export function PortalUnifiedShell({ children, onAuthOpen }: PortalUnifiedShellProps): ReactElement {
  return (
    <UnifiedPageShell
      className={PortalUnifiedChrome.Classes.Shell}
      footer={<GameFooter appVersion={PortalUnifiedChrome.Version.App} />}
      header={
        <UnifiedHeader
          config={createHeaderConfig(onAuthOpen)}
          placement={PortalUnifiedChrome.HeaderProfile.Contained}
          profileName={PortalUnifiedChrome.HeaderProfile.MainScreen}
          showPrimaryNavigation={false}
        />
      }
      viewportLocked={true}
      workClassName={PortalUnifiedChrome.Classes.ShellWork}
    >
      {children}
    </UnifiedPageShell>
  );
}
