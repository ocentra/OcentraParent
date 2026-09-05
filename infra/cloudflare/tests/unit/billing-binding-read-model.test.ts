import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import {
  buildLocalSeedSnapshot,
  loadBillingAuditEvents,
  loadBillingEntitlementSnapshot,
  loadBillingLicenseDecision,
  loadBillingStatusSummary,
  loadPricingPlans,
} from '../../src/billing-binding-read-model.js';
import { createTestHarness } from '../../src/testing.js';

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

    const plans = await loadPricingPlans(harness.env);

    assert.equal(plans.length, 1);
    assert.equal(plans[0]?.planId, 'family-binding-only');
    assert.equal(plans[0]?.priceCents, 7777);
    assert.equal(harness.bindingState.getTouchCount('pricing-public'), 1);
  });

  it('serves billing status and license decisions from the D1-backed subject read model', async () => {
    const harness = createTestHarness();

    const seed = buildLocalSeedSnapshot(harness.env);
    const originalStatus = seed.statusBySubject?.['parent:demo-active'];
    const originalSnapshot = seed.snapshotsBySubject?.['parent:demo-active'];
    assert.ok(originalStatus);
    assert.ok(originalSnapshot);

    await harness.bindingState.applySeedPatch({
      statusBySubject: {
        'parent:demo-active': {
          ...originalStatus,
          subject: 'parent:demo-active',
          plan: {
            ...originalStatus.plan,
            planId: 'family-binding-only',
            displayName: 'Family Binding Only',
            priceCents: 7777,
            deviceLimit: 5,
          },
          deviceUsage: {
            ...originalStatus.deviceUsage,
            activeDevices: 2,
            trustedDevices: 2,
            limit: 5,
          },
          seatComposition: {
            ...originalStatus.seatComposition,
            effectiveLimit: 5,
            availableDeviceSlots: 3,
          },
          parentVisibleState: 'manual-review',
          warnings: ['binding-backed-status'],
          auditReference: 'audit:binding-status',
        },
      },
      snapshotsBySubject: {
        'parent:demo-active': {
          ...originalSnapshot,
          snapshotId: 'snapshot-binding-only',
          subject: 'parent:demo-active',
          planId: 'family-binding-only',
          parentVisibleState: 'manual-review',
          subscriptionStatus: 'past-due',
          deviceLimit: 5,
          activeDevices: 2,
          trustedDevices: 2,
          availableDeviceSlots: 3,
          auditReference: 'audit:binding-snapshot',
        },
      },
    });

    const updatedStatus = await loadBillingStatusSummary(harness.env, 'parent:demo-active');
    assert.equal(updatedStatus.parentVisibleState, 'manual-review');
    assert.deepEqual(updatedStatus.warnings, ['binding-backed-status']);
    assert.equal(updatedStatus.auditReference, 'audit:binding-status');

    const snapshot = await loadBillingEntitlementSnapshot(harness.env, 'parent:demo-active');
    assert.equal(snapshot.snapshotId, 'snapshot-binding-only');
    assert.equal(snapshot.planId, 'family-binding-only');
    assert.equal(snapshot.parentVisibleState, 'manual-review');

    const license = await loadBillingLicenseDecision(
      harness.env,
      'parent:demo-active',
      'binding-license',
      'device-binding',
      true
    );
    assert.equal(license.decision, 'manual-review');
    assert.equal(license.reasonCode, 'manual-review');
    assert.equal(license.planId, 'family-binding-only');
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

    const rows = await loadBillingAuditEvents(harness.env, 'binding-proof');

    assert.equal(rows.length, 1);
    assert.equal(rows[0]?.eventId, 'audit-binding-only');
    assert.equal(rows[0]?.eventType, 'billing.audit.binding-proof');
    assert.equal(harness.bindingState.getTouchCount('admin-billing-audit'), 1);
  });
});
