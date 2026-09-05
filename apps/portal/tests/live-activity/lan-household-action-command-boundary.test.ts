import { describe, expect, it } from 'vitest';
import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';
import { PortalAgentLanHouseholdActionKind } from '@ocentra-parent/portal-domain/contracts';
import { createParentPortalLanPairingUiSlots } from '../../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent';
import {
  lanPairingHouseholdActionCommandPayload,
  lanPairingRouteIntentCommandPayload,
} from '../../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface';
import { lanAddDeviceReadModel } from '../fixtures/activity-ui-lan-pairing-fixtures';

const log = Logger.instance;
log.register(import.meta.url);

describe('portal LAN household action command boundary', () => {
  it('keeps every household mutation unavailable for the real authority-less projection', () => {
    const slots = createParentPortalLanPairingUiSlots([], lanAddDeviceReadModel());
    const selectedSlot = slots.find((slot) => slot.value === 'child-android-1');

    expect(selectedSlot).toMatchObject({
      status: 'connected',
      device: {
        id: 'child-android-1',
        pairingId: 'pairing-child-android-1',
        routeId: 'lan-route-local-1',
      },
    });

    const actions = [
      PortalAgentLanHouseholdActionKind.Trust,
      PortalAgentLanHouseholdActionKind.Ignore,
      PortalAgentLanHouseholdActionKind.Restore,
      PortalAgentLanHouseholdActionKind.Rename,
    ] as const;
    for (const actionKind of actions) {
      const payload = lanPairingHouseholdActionCommandPayload(selectedSlot ?? null, actionKind, {
        displayName: 'Kitchen laptop',
        deviceKind: 'laptop',
        requiresRoute: false,
      });
      expect(payload).toBeNull();
    }

    expect(lanPairingHouseholdActionCommandPayload(null, PortalAgentLanHouseholdActionKind.Ignore)).toBeNull();
    const routeSelectPayload = lanPairingRouteIntentCommandPayload(selectedSlot ?? null);
    const routeRevokePayload = lanPairingRouteIntentCommandPayload(selectedSlot ?? null);
    expect(routeSelectPayload).toBeNull();
    expect(routeRevokePayload).toBeNull();
    expect(lanPairingRouteIntentCommandPayload(null)).toBeNull();
    log.logInfo(
      'portal LAN household actions remain unavailable without owner-issued authority',
      getStackTrace(),
      { projectedSlots: slots.length, checkedActions: actions.length, checkedRouteCommands: 2 },
      false
    );
  });
});
