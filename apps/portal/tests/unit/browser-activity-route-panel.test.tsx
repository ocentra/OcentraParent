import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { ParentRoute } from '../../generated/parent-ui-bridge';
import { BrowserActivityRoutePanel, shouldRenderBrowserActivityRoute } from '../../src/BrowserActivityRoutePanel';
import type { PortalRenderActions } from '../../src/portal-actions';
import { resolveSnapshotLiveActivityState } from '../../src/route-live-activity-state';

const actions: PortalRenderActions = {
  reconnect() {},
  selectCommandResult() {},
  async sendCommand() {
    return null;
  },
};

describe('browser activity route panel', () => {
  it('renders only on the Rust-backed Browser product route', () => {
    expect([
      shouldRenderBrowserActivityRoute(ParentRoute.Browser),
      shouldRenderBrowserActivityRoute(ParentRoute.BrowserSettings),
    ]).toEqual([true, false]);
  });

  it('renders an active reconnect path without inventing browser state', () => {
    const markup = renderToStaticMarkup(
      createElement(BrowserActivityRoutePanel, {
        actions,
        commandEnabled: false,
        liveActivity: resolveSnapshotLiveActivityState(null),
      })
    );

    expect(markup).toContain('data-ocentra-browser-route-state="unavailable"');
    expect(markup).toContain('<h2>Browser status unavailable</h2>');
    expect(markup).toContain('<h2>Managed session</h2>');
    expect(markup).toContain('<h2>Evidence status</h2>');
    expect(markup).toContain('<h2>Activity rows</h2>');
    expect(markup.match(/<article class="summary product-status-card">/gu)).toHaveLength(3);
    expect(markup.match(/<p>Not reported<\/p>/gu)).toHaveLength(3);
    expect(markup).toContain('Retry status');
    expect(markup).not.toContain('Refresh browser status');
  });

  it('renders service-owned status, activity, and evidence without exposing exact URL or endpoint refs', () => {
    const markup = renderToStaticMarkup(
      createElement(BrowserActivityRoutePanel, {
        actions: {
          ...actions,
          async refreshRouteSnapshot() {
            return null;
          },
        },
        commandEnabled: true,
        liveActivity: browserLiveActivity(),
      })
    );

    expect(markup).toContain('data-ocentra-browser-route-state="reported"');
    expect(markup).toContain('Browser activity and managed-session status');
    expect(markup).toContain('bridge-connected');
    expect(markup).toContain('1 reported');
    expect(markup).toContain('Refresh browser status');
    expect(markup).not.toContain('https://private.example/path');
    expect(markup).not.toContain('bridge-endpoint-ref-1');
  });
});

function browserLiveActivity() {
  return resolveSnapshotLiveActivityState({
    activityBrowserReadModel: browserActivityReadModel(),
    browserManagedStatus: browserManagedStatus(),
    browserEvidenceReadModel: browserEvidenceReadModel(),
  });
}

function browserActivityReadModel() {
  return {
    ok: true,
    state: 'ready',
    value: {
      schemaVersion: 1,
      request: {
        schemaVersion: 1,
        scope: { scopeKind: 'family', familyId: 'family-1', deviceId: null },
        requestedAt: '2026-09-03T10:00:00Z',
        rangeStart: '2026-09-02T10:00:00Z',
        rangeEnd: '2026-09-03T10:00:00Z',
      },
      state: 'ready',
      generatedAt: '2026-09-03T10:00:01Z',
      summary: 'One bounded browser activity row was reported.',
      rows: [
        {
          rowId: 'browser-row-1',
          domainLabel: 'private.example',
          deviceId: 'child-device-1',
          state: 'ready',
          visitCount: 1,
          totalMs: 5000,
          evidenceDigest: 'browser-evidence-digest-1',
        },
      ],
    },
  };
}

function browserManagedStatus() {
  return {
    schemaVersion: 1,
    checkedAt: '2026-09-03T10:00:01Z',
    managedBrowserSessionId: 'managed-session-1',
    browserFamily: 'chrome',
    browserChannel: 'stable',
    browserVersion: '128.0.0',
    profileId: 'profile-1',
    profilePathRef: 'profile-path-ref-1',
    profileRootRef: 'profile-root-ref-1',
    profileScopeId: 'profile-scope-1',
    profileLifecycleState: 'ready',
    policyRevision: 'policy-revision-1',
    processId: 4242,
    bridgeKind: 'chromium-devtools-protocol',
    bridgeEndpointRef: 'bridge-endpoint-ref-1',
    unmanagedProcessName: null,
    unmanagedExecutablePathRef: null,
    unmanagedSignatureRef: null,
    unmanagedProcessHashRef: null,
    unmanagedProcessKind: null,
    unmanagedDetectionConfidence: null,
    unmanagedDetectionReason: null,
    managedState: 'bridge-connected',
    capabilityStatus: 'available',
    degradedReason: null,
    startedAt: '2026-09-03T10:00:00Z',
    custodyLabel: 'child-device-local',
    queryVisibility: 'live-local',
  };
}

function browserEvidenceReadModel() {
  return {
    schemaVersion: 1,
    generatedAt: '2026-09-03T10:00:01Z',
    limit: 10,
    returned: 1,
    latestEventId: 'browser-event-1',
    latestObservedAt: '2026-09-03T10:00:00Z',
    capabilityStatus: 'available',
    custodyLabel: 'child-device-local',
    queryVisibility: 'live-local',
    rows: [
      {
        schemaVersion: 1,
        browserEvidenceId: 'browser-evidence-1',
        observedAt: '2026-09-03T10:00:00Z',
        freshUntil: '2026-09-03T10:01:00Z',
        sourceId: 'browser-source-1',
        adapterId: 'browser-adapter-1',
        deviceId: 'child-device-1',
        browserFamily: 'chrome',
        browserChannel: 'stable',
        browserVersion: '128.0.0',
        managedBrowserSessionId: 'managed-session-1',
        profileId: 'profile-1',
        processId: 4242,
        windowId: 'window-1',
        tabId: 'tab-1',
        targetId: 'target-1',
        activeState: 'known-active',
        activeProofSource: 'cdp-focus-activation',
        url: 'https://private.example/path',
        origin: 'https://private.example',
        domain: 'private.example',
        title: 'Private page',
        capabilityStatus: 'available',
        degradedReason: null,
        staleAt: '2026-09-03T10:01:00Z',
        custodyLabel: 'child-device-local',
        queryVisibility: 'live-local',
      },
    ],
  };
}
