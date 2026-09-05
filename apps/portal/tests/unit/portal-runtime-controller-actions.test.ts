import { describe, expect, it } from 'vitest';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { ParentRoute, ParentUiActionKind, type ParentUiAction } from '../../generated/parent-ui-bridge';
import { createPortalRuntimeActions } from '../../src/portal-runtime-controller-actions';
import { createPortalRuntimeState } from '../../src/portal-state';

describe('portal runtime retry action', () => {
  it('labels the fail-closed action as a status retry and requests only a route refresh', () => {
    const dispatched: ParentUiAction[] = [];
    const actions = createPortalRuntimeActions(
      {
        state: createPortalRuntimeState(),
        refresh() {},
        getRoute: () => ParentRoute.Browser,
      },
      async (action) => {
        dispatched.push(action);
        return null;
      }
    );

    actions.reconnect();

    expect(resolvePortalDevText(PortalDevTextToken.RetryStatus)).toBe('Retry status');
    expect(dispatched).toEqual([
      {
        action: ParentUiActionKind.RefreshRoute,
        route: ParentRoute.Browser,
        payload: {},
      },
    ]);
  });
});
