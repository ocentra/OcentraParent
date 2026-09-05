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

export function PortalAuthDialog({ onClose }: PortalAuthDialogProps): ReactElement {
  return (
    <LoginDialog
      brandTitle={resolvePortalDevText(PortalDevTextToken.AppTitle)}
      closeAriaLabel={resolvePortalDevText(PortalDevTextToken.AuthClose)}
      contextDescription={resolvePortalDevText(PortalDevTextToken.AuthBody)}
      contextEyebrow={resolvePortalDevText(PortalDevTextToken.AuthEyebrow)}
      contextTitle={resolvePortalDevText(PortalDevTextToken.AuthTitle)}
      disableCredentials
      layoutControls={AUTH_PAGE_LAYOUT_CONTROLS}
      onClose={onClose}
      onLogin={unavailableResult}
      statusMessage={{
        kind: 'info',
        text: resolvePortalDevText(PortalDevTextToken.AuthUnavailable),
      }}
    />
  );
}
