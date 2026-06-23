import { describe, expect, it } from 'vitest';
import {
  AppGameLinuxActiveWindowToolProofSchema,
  createAppGameLinuxActiveWindowToolProof,
  summarizeAppGameLinuxActiveWindowToolProof,
} from '@ocentra-parent/schema-domain/app-game-linux-active-window-tool-proof';

describe('app-game Linux active-window tool proof', () => {
  recordsAvailableToolWithoutActiveWindowCaptureClaim();
  recordsObservedOpaqueActiveWindowRefWithoutRawTitleCustody();
  rejectsRawTitleAndForegroundCaptureClaims();
});

function recordsAvailableToolWithoutActiveWindowCaptureClaim() {
  it('records xprop availability while no active-window ref is observed', () => {
    const proof = createAppGameLinuxActiveWindowToolProof({
      toolState: 'xprop-available',
      activeWindowRefState: 'no-active-window-ref',
      displaySourceObserved: true,
      checkedAt: '2026-06-08T21:55:00.000Z',
    });
    const summary = summarizeAppGameLinuxActiveWindowToolProof(proof);

    expect(summary.toolState).toBe('xprop-available');
    expect(summary.toolAvailable).toBe(true);
    expect(summary.activeWindowRefObserved).toBe(false);
    expect(proof.proofRefs).toEqual(
      expect.arrayContaining(['linux-wsl-runtime-proof-ref', 'linux-wslg-display-ref', 'linux-active-window-tool-ref'])
    );
    expect(proof.openGaps).toEqual(
      expect.arrayContaining([
        'linux-active-window-ref-not-observed',
        'linux-active-window-title-not-captured',
        'linux-foreground-capture-not-proved',
        'linux-platform-enforcement-not-proved',
        'linux-child-device-delivery-not-proved',
      ])
    );
  });
}

function recordsObservedOpaqueActiveWindowRefWithoutRawTitleCustody() {
  it('allows an opaque active-window ref while keeping title/process custody unclaimed', () => {
    const proof = createAppGameLinuxActiveWindowToolProof({
      toolState: 'xdotool-available',
      activeWindowRefState: 'active-window-ref-observed',
      displaySourceObserved: true,
      checkedAt: '2026-06-08T21:55:00.000Z',
    });

    expect(proof.proofRefs).toContain('linux-active-window-ref-proof');
    expect(proof.openGaps).not.toContain('linux-active-window-ref-not-observed');
    expect(proof.rawWindowTitleStored).toBe(false);
    expect(proof.rawProcessNameStored).toBe(false);
    expect(proof.foregroundCaptureClaimed).toBe(false);
  });
}

function rejectsRawTitleAndForegroundCaptureClaims() {
  it('rejects raw title custody, foreground capture, adapter dispatch, and enforcement claims', () => {
    const proof = createAppGameLinuxActiveWindowToolProof({
      toolState: 'xprop-available',
      activeWindowRefState: 'no-active-window-ref',
      displaySourceObserved: true,
      checkedAt: '2026-06-08T21:55:00.000Z',
    });

    expect(
      AppGameLinuxActiveWindowToolProofSchema.safeParse({
        ...proof,
        rawWindowTitleStored: true,
      }).success
    ).toBe(false);
    expect(
      AppGameLinuxActiveWindowToolProofSchema.safeParse({
        ...proof,
        foregroundCaptureClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameLinuxActiveWindowToolProofSchema.safeParse({
        ...proof,
        platformEnforcementClaimed: true,
      }).success
    ).toBe(false);
  });
}
