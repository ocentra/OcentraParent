import type { ReactElement } from 'react';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { LoginDialog, type LoginDialogActionResult } from '../../../vendor/ocentra-parent-core-ui/Auth/LoginDialog';
import { AUTH_PAGE_LAYOUT_CONTROLS } from '../../../vendor/ocentra-parent-core-ui/Auth/AuthPageLayoutControls';

type PortalAuthDialogProps = {
  readonly onClose: () => void;
};

function unavailableResult(): Promise<LoginDialogActionResult> {
  return Promise.resolve({
    success: false,
    error: resolvePortalDevText(PortalDevTextToken.AuthUnavailable),
  });
}

function unavailableSignUp(): Promise<LoginDialogActionResult> {
  return unavailableResult();
}

export function PortalAuthDialog({ onClose }: PortalAuthDialogProps): ReactElement {
  return (
    <LoginDialog
      brandTitle={resolvePortalDevText(PortalDevTextToken.AppTitle)}
      closeAriaLabel={resolvePortalDevText(PortalDevTextToken.AuthClose)}
      contextDescription={resolvePortalDevText(PortalDevTextToken.AuthBody)}
      contextEyebrow={resolvePortalDevText(PortalDevTextToken.AuthEyebrow)}
      contextTitle={resolvePortalDevText(PortalDevTextToken.AuthTitle)}
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
