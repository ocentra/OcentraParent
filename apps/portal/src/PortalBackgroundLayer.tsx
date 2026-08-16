import { useMemo, type ReactElement } from 'react';
import { type PortalThemeValue } from '@ocentra-parent/portal-domain/contracts';
import { PortalBackgroundSvg } from './PortalBackgroundSvg';
import { portalBackgroundAppRenderConfig } from './portal-background-config';
import { usePortalBackgroundConfig } from './use-portal-background-config';

type PortalBackgroundLayerProps = {
  readonly theme: PortalThemeValue;
};

export function PortalBackgroundLayer({ theme }: PortalBackgroundLayerProps): ReactElement {
  const [config] = usePortalBackgroundConfig();
  const renderConfig = useMemo(() => portalBackgroundAppRenderConfig(config, theme), [config, theme]);

  return (
    <PortalBackgroundSvg
      {...renderConfig}
      ariaHidden={true}
      preserveAspectRatio="xMidYMid slice"
      style={{
        height: '100%',
        inset: 0,
        position: 'absolute',
        width: '100%',
      }}
    />
  );
}
