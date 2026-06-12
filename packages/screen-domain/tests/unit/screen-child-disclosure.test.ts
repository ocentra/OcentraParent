import { describe, expect, it } from 'vitest';
import {
  ScreenChildDisclosureSnapshotSchema,
  ScreenChildDisclosureTextToken,
  screenChildDisclosureProofSnapshots,
} from '../../src/screen-evidence';

describe('screen child disclosure', () => {
  specifyVisibleStates();
  specifyDisabledState();
  specifyActiveCaptureState();
  specifyNoSurveillanceClaims();
  specifyDeletedSummaryCustody();
});

function specifyVisibleStates() {
  it('defines child-visible disabled, paused, active, protected, and deleted-summary states', () => {
    const snapshots = screenChildDisclosureProofSnapshots();

    expect(snapshots.map((snapshot) => snapshot.state)).toEqual([
      'disabledByParent',
      'pausedByParent',
      'captureActive',
      'protectedSurface',
      'deletedSummaryReady',
    ]);
    expect(snapshots.map((snapshot) => ScreenChildDisclosureSnapshotSchema.safeParse(snapshot).success)).toEqual([
      true,
      true,
      true,
      true,
      true,
    ]);
    expect(snapshots.map((snapshot) => snapshot.visibleToChildRequired)).toEqual([true, true, true, true, true]);
    expect(snapshots.map((snapshot) => snapshot.tone)).toEqual(['calm', 'calm', 'calm', 'calm', 'calm']);
  });
}

function specifyDisabledState() {
  it('keeps disabled screen analysis unable to capture or hide from the child', () => {
    const disabled = screenChildDisclosureProofSnapshots()[0];

    expect(disabled?.screenAnalysisEnabled).toBe(false);
    expect(disabled?.cadenceCaptureEnabled).toBe(false);
    expect(disabled?.triggerCaptureEnabled).toBe(false);
    expect(disabled?.captureActive).toBe(false);
    expect(disabled?.capabilityStatus).toBe('disabledByParent');
    expect(disabled?.primaryTextToken).toBe(ScreenChildDisclosureTextToken.Disabled);
    expect(disabled?.hiddenCaptureClaimed).toBe(false);
    expect(disabled?.rawScreenshotShownToChild).toBe(false);
  });
}

function specifyActiveCaptureState() {
  it('requires active capture to be visible, ready, and scoped before it can be represented', () => {
    const active = screenChildDisclosureProofSnapshots()[2];
    const invalidUnsupportedCapture = ScreenChildDisclosureSnapshotSchema.safeParse({
      ...active,
      captureScope: 'unsupported',
    });
    const invalidModeledCapture = ScreenChildDisclosureSnapshotSchema.safeParse({
      ...active,
      surface: 'modeled-only',
    });

    expect(active?.state).toBe('captureActive');
    expect(active?.captureActive).toBe(true);
    expect(active?.surface).toBe('child-agent-capture-banner');
    expect(active?.capabilityStatus).toBe('ready');
    expect(active?.captureScope).toBe('activeWindow');
    expect(invalidUnsupportedCapture.success).toBe(false);
    expect(invalidModeledCapture.success).toBe(false);
  });
}

function specifyNoSurveillanceClaims() {
  it('rejects hidden capture, raw screenshot display, remote viewer, and policy-authority claims', () => {
    const active = screenChildDisclosureProofSnapshots()[2];

    expect(ScreenChildDisclosureSnapshotSchema.safeParse({ ...active, hiddenCaptureClaimed: true }).success).toBe(
      false
    );
    expect(ScreenChildDisclosureSnapshotSchema.safeParse({ ...active, rawScreenshotShownToChild: true }).success).toBe(
      false
    );
    expect(ScreenChildDisclosureSnapshotSchema.safeParse({ ...active, remoteViewerClaimed: true }).success).toBe(false);
    expect(ScreenChildDisclosureSnapshotSchema.safeParse({ ...active, policyAuthorityClaimed: true }).success).toBe(
      false
    );
  });
}

function specifyDeletedSummaryCustody() {
  it('requires deleted summaries to cite deleted local custody before showing summary-ready state', () => {
    const summaryReady = screenChildDisclosureProofSnapshots()[4];
    const invalidMissingDeletion = ScreenChildDisclosureSnapshotSchema.safeParse({
      ...summaryReady,
      deletionState: null,
    });
    const invalidLiveCustody = ScreenChildDisclosureSnapshotSchema.safeParse({
      ...summaryReady,
      custodyState: 'live-local-child-agent',
    });

    expect(summaryReady?.state).toBe('deletedSummaryReady');
    expect(summaryReady?.deletionState).toBe('deleted');
    expect(summaryReady?.custodyState).toBe('child-device-query-store');
    expect(summaryReady?.captureActive).toBe(false);
    expect(invalidMissingDeletion.success).toBe(false);
    expect(invalidLiveCustody.success).toBe(false);
  });
}
