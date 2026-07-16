import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import { createTestHarness, executeRequest, readJson } from '../../src/testing.js';

describe('binding-backed payment read model', () => {
  it('serves pricing from the config binding instead of the static fixture array', async () => {
    const harness = createTestHarness();
    await harness.bindingState.applySeedPatch({
      pricingPlans: [
        {
          planId: 'family-binding-only',
          displayName: 'Family Binding Only',
          interval: 'monthly',
          priceCents: 7777,
          currency: 'USD',
          deviceLimit: 12,
          activeState: 'active',
          featureSummary: [
            {
              code: 'binding-proof',
              label: 'Binding-backed pricing proof',
              included: true,
              safetyCritical: false,
            },
          ],
        },
      ],
    });

    const { response } = await executeRequest({
      path: '/public/pricing',
      harness,
    });
    const body = await readJson<{
      plans: Array<{
        planId: string;
        priceCents: number;
      }>;
    }>(response);

    assert.equal(response.status, 200);
    assert.equal(body.plans.length, 1);
    assert.equal(body.plans[0]?.planId, 'family-binding-only');
    assert.equal(body.plans[0]?.priceCents, 7777);
    assert.equal(harness.bindingState.getTouchCount('pricing-public'), 1);
  });

  it('serves billing status and license decisions from the D1-backed subject read model', async () => {
    const harness = createTestHarness();

    const originalStatus = await executeRequest({
      path: '/auth/billing/status',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const originalStatusBody = await readJson<Record<string, unknown>>(originalStatus.response);

    const originalSnapshot = await executeRequest({
      path: '/auth/billing/entitlement-snapshot',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
        'x-ocentra-trusted-device': 'true',
      },
    });
    const originalSnapshotBody = await readJson<{
      snapshot: Record<string, unknown>;
    }>(originalSnapshot.response);

    await harness.bindingState.applySeedPatch({
      statusBySubject: {
        'parent:demo-active': {
          ...(originalStatusBody as Record<string, unknown>),
          subject: 'parent:demo-active',
          parentVisibleState: 'manual-review',
          warnings: ['binding-backed-status'],
          auditReference: 'audit:binding-status',
        } as never,
      },
      snapshotsBySubject: {
        'parent:demo-active': {
          ...(originalSnapshotBody.snapshot as Record<string, unknown>),
          snapshotId: 'snapshot-binding-only',
          subject: 'parent:demo-active',
          planId: 'family-binding-only',
          parentVisibleState: 'manual-review',
          subscriptionStatus: 'past-due',
          deviceLimit: 2,
          activeDevices: 2,
          availableDeviceSlots: 0,
          auditReference: 'audit:binding-snapshot',
        } as never,
      },
    });

    const updatedStatus = await executeRequest({
      path: '/auth/billing/status',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
      },
    });
    const updatedStatusBody = await readJson<{
      parentVisibleState: string;
      warnings: ReadonlyArray<string>;
      auditReference: string;
    }>(updatedStatus.response);

    assert.equal(updatedStatus.response.status, 200);
    assert.equal(updatedStatusBody.parentVisibleState, 'manual-review');
    assert.deepEqual(updatedStatusBody.warnings, ['binding-backed-status']);
    assert.equal(updatedStatusBody.auditReference, 'audit:binding-status');

    const snapshot = await executeRequest({
      path: '/auth/billing/entitlement-snapshot',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
        'x-ocentra-trusted-device': 'true',
      },
    });
    const snapshotBody = await readJson<{
      snapshot: {
        snapshotId: string;
        planId: string;
        parentVisibleState: string;
      };
    }>(snapshot.response);

    assert.equal(snapshot.response.status, 200);
    assert.equal(snapshotBody.snapshot.snapshotId, 'snapshot-binding-only');
    assert.equal(snapshotBody.snapshot.planId, 'family-binding-only');
    assert.equal(snapshotBody.snapshot.parentVisibleState, 'manual-review');

    const license = await executeRequest({
      path: '/auth/billing/license-check',
      method: 'POST',
      harness,
      headers: {
        authorization: 'Bearer parent:demo-active',
        'x-ocentra-trusted-device': 'true',
      },
      body: {
        requestId: 'binding-license',
        deviceId: 'device-binding',
        requestedNewDevice: true,
      },
    });
    const licenseBody = await readJson<{
      decision: string;
      reasonCode: string;
      planId: string;
    }>(license.response);

    assert.equal(license.response.status, 200);
    assert.equal(licenseBody.decision, 'manual-review');
    assert.equal(licenseBody.reasonCode, 'manual-review');
    assert.equal(licenseBody.planId, 'family-binding-only');
  });

  it('serves audit rows from the R2-backed audit object', async () => {
    const harness = createTestHarness();
    await harness.bindingState.applySeedPatch({
      auditEvents: [
        {
          eventId: 'audit-binding-only',
          eventType: 'billing.audit.binding-proof',
          actorRole: 'admin',
          parentAccountRef: 'parent-account:binding',
          familyRef: 'family:binding',
          auditReference: 'audit:binding-proof',
          createdAt: '2026-06-14T00:00:00.000Z',
        } as any,
      ],
    });

    const { response } = await executeRequest({
      path: '/admin/billing/audit?q=binding-proof',
      harness,
      headers: {
        authorization: 'Bearer parent:admin-agent',
        'x-ocentra-role': 'admin',
      },
    });
    const body = await readJson<{
      resultCount: number;
      results: Array<{
        eventId: string;
        eventType: string;
      }>;
    }>(response);

    assert.equal(response.status, 200);
    assert.equal(body.resultCount, 1);
    assert.equal(body.results[0]?.eventId, 'audit-binding-only');
    assert.equal(body.results[0]?.eventType, 'billing.audit.binding-proof');
    assert.equal(harness.bindingState.getTouchCount('admin-billing-audit'), 1);
  });
});
