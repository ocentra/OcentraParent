import type { AuthPageSvgControls } from './CyberAuthSurface';
import authPageLayoutSource from './AuthPageLayout.asset?raw';

type AuthPageLayoutAsset = {
  readonly data?: {
    readonly authControls?: Partial<AuthPageSvgControls>;
  };
};

const authPageLayout = JSON.parse(authPageLayoutSource) as AuthPageLayoutAsset;

export const AUTH_PAGE_LAYOUT_CONTROLS = authPageLayout.data?.authControls ?? null;
