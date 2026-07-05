import { describe, expect, it } from 'vitest';
import { PortalAgentEvent, type PortalRouteEventRecord } from '../../src/portal-contract-adapter';
import { GeneratedPortalSocialReadModelPayloadField } from '../../src/generated-portal-contracts';
import {
  parseAgentSocialAlertReportParentSurfaceReadModelEvent,
  parseAgentSocialAlertReportReadModelEvent,
  parseAgentSocialDashboardReadModelEvent,
  parseAgentSocialParentNotificationDeliveryReadModelEvent,
} from '../../src/social-read-model-events';

describe('social read model event adapter', () => {
  it('decodes Rust-owned social read model payloads through generated event and field names', () => {
    const snapshot = {
      schemaVersion: 1,
      generatedAt: '2026-06-30T14:00:00.000Z',
      rows: [{ rowId: 'social-row-1' }],
    };
    const event = routeEvent(PortalAgentEvent.BrowserSocialAlertReportReadModelReported, {
      [GeneratedPortalSocialReadModelPayloadField.AlertReport]: JSON.stringify(snapshot),
    });

    expect(parseAgentSocialAlertReportReadModelEvent(event)).toEqual({
      ok: true,
      value: snapshot,
    });
  });

  it('uses the generated social field instead of earlier unrelated string payloads', () => {
    const snapshot = {
      schemaVersion: 1,
      generatedAt: '2026-06-30T14:10:00.000Z',
      rows: [{ rowId: 'social-row-with-prefix' }],
    };
    const event = routeEvent(PortalAgentEvent.BrowserSocialAlertReportReadModelReported, {
      status: 'ready',
      readModel: JSON.stringify({ wrongField: true }),
      [GeneratedPortalSocialReadModelPayloadField.AlertReport]: JSON.stringify(snapshot),
    });

    expect(parseAgentSocialAlertReportReadModelEvent(event)).toEqual({
      ok: true,
      value: snapshot,
    });
  });
});

describe('social read model event adapter contracts', () => {
  it('uses the generated social event and payload-field contract for each read model seam', () => {
    const snapshot = { schemaVersion: 1, rows: [] };

    expect(
      parseAgentSocialAlertReportParentSurfaceReadModelEvent(
        routeEvent(PortalAgentEvent.BrowserSocialAlertReportParentSurfaceReadModelReported, {
          [GeneratedPortalSocialReadModelPayloadField.AlertReportParentSurface]: JSON.stringify(snapshot),
        })
      )
    ).toEqual({ ok: true, value: snapshot });
    expect(
      parseAgentSocialParentNotificationDeliveryReadModelEvent(
        routeEvent(PortalAgentEvent.BrowserSocialParentNotificationDeliveryReadModelReported, {
          [GeneratedPortalSocialReadModelPayloadField.ParentNotificationDelivery]: JSON.stringify(snapshot),
        })
      )
    ).toEqual({ ok: true, value: snapshot });
    expect(
      parseAgentSocialDashboardReadModelEvent(
        routeEvent(PortalAgentEvent.BrowserSocialDashboardReadModelReported, {
          [GeneratedPortalSocialReadModelPayloadField.Dashboard]: JSON.stringify(snapshot),
        })
      )
    ).toEqual({ ok: true, value: snapshot });
  });

  it('rejects event mismatches before reading generated payload fields', () => {
    expect(
      parseAgentSocialDashboardReadModelEvent(
        routeEvent(PortalAgentEvent.BrowserSocialAlertReportReadModelReported, {
          [GeneratedPortalSocialReadModelPayloadField.Dashboard]: JSON.stringify({ schemaVersion: 1 }),
        })
      )
    ).toEqual({ ok: false, reason: 'wrong-event' });
  });

  it('keeps malformed generated social payload failures explicit', () => {
    expect(
      parseAgentSocialDashboardReadModelEvent(
        routeEvent(PortalAgentEvent.BrowserSocialDashboardReadModelReported, {
          [GeneratedPortalSocialReadModelPayloadField.Dashboard]: 'not-json',
        })
      )
    ).toEqual({ ok: false, reason: 'invalid-json' });
    expect(
      parseAgentSocialDashboardReadModelEvent(
        routeEvent(PortalAgentEvent.BrowserSocialDashboardReadModelReported, {
          [GeneratedPortalSocialReadModelPayloadField.Dashboard]: JSON.stringify(['not-record']),
        })
      )
    ).toEqual({ ok: false, reason: 'invalid-payload' });
    expect(
      parseAgentSocialDashboardReadModelEvent(
        routeEvent(PortalAgentEvent.BrowserSocialDashboardReadModelReported, {
          [GeneratedPortalSocialReadModelPayloadField.Dashboard]: 1,
        })
      )
    ).toEqual({ ok: false, reason: 'missing-json-field' });
    expect(
      parseAgentSocialDashboardReadModelEvent(
        routeEvent(PortalAgentEvent.BrowserSocialDashboardReadModelReported, {
          readModel: JSON.stringify({ schemaVersion: 1 }),
        })
      )
    ).toEqual({ ok: false, reason: 'missing-json-field' });
  });
});

function routeEvent(
  event: PortalRouteEventRecord['event'],
  payload: PortalRouteEventRecord['payload']
): PortalRouteEventRecord {
  return {
    event,
    payload,
  };
}
