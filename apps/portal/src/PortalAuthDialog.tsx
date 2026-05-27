import type { ReactElement } from 'react';
import { PortalText, PortalTextToken } from '@ocentra-parent/portal-domain/contracts';
import { LoginDialog, type LoginDialogActionResult } from '../../../vendor/ocentra-parent-core-ui/Auth/LoginDialog';
import { AUTH_PAGE_LAYOUT_CONTROLS } from '../../../vendor/ocentra-parent-core-ui/Auth/AuthPageLayoutControls';

type PortalAuthDialogProps = {
  readonly onClose: () => void;
};

function unavailableResult(): Promise<LoginDialogActionResult> {
  return Promise.resolve({
    success: false,
    error: PortalText.Resolve(PortalTextToken.AuthUnavailable),
  });
}

function unavailableSignUp(): Promise<LoginDialogActionResult> {
  return unavailableResult();
}

export function PortalAuthDialog({ onClose }: PortalAuthDialogProps): ReactElement {
  return (
    <LoginDialog
      brandTitle={PortalText.Resolve(PortalTextToken.AppTitle)}
      closeAriaLabel={PortalText.Resolve(PortalTextToken.AuthClose)}
      contextDescription={PortalText.Resolve(PortalTextToken.AuthBody)}
      contextEyebrow={PortalText.Resolve(PortalTextToken.AuthEyebrow)}
      contextTitle={PortalText.Resolve(PortalTextToken.AuthTitle)}
      layoutControls={AUTH_PAGE_LAYOUT_CONTROLS}
      onClose={onClose}
      onFacebookLogin={unavailableResult}
      onGoogleLogin={unavailableResult}
      onGuestLogin={unavailableResult}
      onLogin={unavailableResult}
      onSendPasswordReset={unavailableResult}
      onSignUp={unavailableSignUp}
    />
  );
}
