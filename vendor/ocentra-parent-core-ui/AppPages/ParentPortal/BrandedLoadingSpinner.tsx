import { parentPortalLogoCometImageUrl, parentPortalLogoImageUrl } from '../../shims/parent-portal-assets-common';
import './BrandedLoadingSpinner.css';

export type BrandedLoadingSpinnerSize = 'small' | 'medium' | 'large';

export interface BrandedLoadingSpinnerProps {
  size?: BrandedLoadingSpinnerSize;
}

export function BrandedLoadingSpinner({ size = 'medium' }: BrandedLoadingSpinnerProps) {
  return (
    <div className={`branded-loading-spinner branded-loading-spinner--${size}`} aria-hidden="true">
      <div className="branded-loading-spinner__orbit">
        <div className="branded-loading-spinner__comet-layer">
          <img className="branded-loading-spinner__comet" src={parentPortalLogoCometImageUrl} alt="" />
          <span className="branded-loading-spinner__comet-head" aria-hidden="true" />
        </div>
      </div>
      <img className="branded-loading-spinner__logo" src={parentPortalLogoImageUrl} alt="" />
    </div>
  );
}
