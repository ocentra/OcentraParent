import { describe, expect, it } from 'vitest';
import {
  ScreenChildDisclosureStatusSchema,
  ScreenChildDisclosureUxProofSchema,
  screenChildDisclosureUxProof,
} from '../src/screen-child-disclosure-ux';

describe('screen child disclosure UX contracts', () => {
  it('exposes disabled, paused, ready, capture-active, and protected-surface child-visible states', () => {
    const proof = screenChildDisclosureUxProof();
    const states = proof.statuses.map((status) => status.state);

    expect(ScreenChildDisclosureUxProofSchema.parse(proof).proofTier).toBe('P2_CONTRACT_WITH_PORTAL_PROOF');
    expect(states).toEqual(['disabled', 'paused', 'ready', 'captureActive', 'protectedSurface']);
    expect(proof.hiddenCaptureAllowed).toBe(false);
    expect(proof.productionChildAppClaimed).toBe(false);
    expect(proof.servicePersistenceClaimed).toBe(false);
    expect(proof.osNotificationClaimed).toBe(false);
    for (const status of proof.statuses) {
      expect(status.childVisible).toBe(true);
      expect(status.hiddenCaptureAllowed).toBe(false);
      expect(status.rawScreenshotPathVisible).toBe(false);
      expect(status.rawScreenshotRemoteUploadEnabled).toBe(false);
      expect(status.auditRef).not.toBeNull();
    }
  });

  it('requires capture-active disclosure to be visible, local-only, audited, and capability-ready', () => {
    const proof = screenChildDisclosureUxProof();
    const active = proof.statuses.find((status) => status.state === 'captureActive');

    if (active === undefined) {
      throw new Error('expected capture-active child disclosure status');
    }

    expect(active.indicator).toBe('active');
    expect(active.captureActive).toBe(true);
    expect(active.capabilityStatus).toBe('ready');
    expect(active.delivery).toBe('childDeviceLocal');
    expect(active.custodyState).toBe('live-local-child-agent');
  });

  it('rejects hidden capture, invisible active status, remote upload, raw path exposure, and missing audit refs', () => {
    const proof = screenChildDisclosureUxProof();
    const active = proof.statuses.find((status) => status.state === 'captureActive');
    const disabled = proof.statuses.find((status) => status.state === 'disabled');

    if (active === undefined || disabled === undefined) {
      throw new Error('expected disclosure fixtures');
    }

    expect(ScreenChildDisclosureStatusSchema.safeParse({ ...active, hiddenCaptureAllowed: true }).success).toBe(false);
    expect(ScreenChildDisclosureStatusSchema.safeParse({ ...active, childVisible: false }).success).toBe(false);
    expect(ScreenChildDisclosureStatusSchema.safeParse({ ...active, auditRef: null }).success).toBe(false);
    expect(
      ScreenChildDisclosureStatusSchema.safeParse({ ...active, rawScreenshotRemoteUploadEnabled: true }).success
    ).toBe(false);
    expect(ScreenChildDisclosureStatusSchema.safeParse({ ...active, rawScreenshotPathVisible: true }).success).toBe(
      false
    );
    expect(ScreenChildDisclosureStatusSchema.safeParse({ ...disabled, captureActive: true }).success).toBe(false);
  });

  it('rejects proof packs that omit a required disclosure state', () => {
    const proof = screenChildDisclosureUxProof();
    const missingProtectedSurface = {
      ...proof,
      statuses: proof.statuses.filter((status) => status.state !== 'protectedSurface'),
    };

    expect(ScreenChildDisclosureUxProofSchema.safeParse(missingProtectedSurface).success).toBe(false);
  });
});
