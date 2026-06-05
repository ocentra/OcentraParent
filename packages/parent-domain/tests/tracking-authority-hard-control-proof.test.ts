import { describe, expect, it } from 'vitest';
import {
  TrackingAuthorityHardControlReadModelSchema,
  TrackingAuthorityHardControlSurfaceProofSchema,
  buildTrackingAuthorityHardControlReadModel,
} from '../src/tracking-authority-hard-control-proof';

describe('tracking authority hard-control proof', () => {
  it('keeps all enrolled-device authority surfaces authority-required without product claims', () => {
    const readModel = buildTrackingAuthorityHardControlReadModel();

    expect(readModel.summary).toEqual({
      surfaceCount: 5,
      authorityRequiredRows: 5,
      authorityEnrolledRows: 0,
      hardControlClaimedRows: 0,
      childDeviceRuntimeClaimedRows: 0,
      physicalDeviceClaimedRows: 0,
      productClaimReadyRows: 0,
    });
    expect(readModel.surfaces.map((surface) => surface.surface).sort()).toEqual([
      'android-device-owner-location-control',
      'android-managed-profile-location-control',
      'ios-supervised-mdm-location-control',
      'macos-mdm-location-control',
      'windows-applocker-app-control-location-control',
    ]);
    expect(readModel.surfaces.every((surface) => surface.authorityRequirement === 'authority_required')).toBe(true);
  });

  it('rejects hard-control and product-ready overclaims', () => {
    const readModel = buildTrackingAuthorityHardControlReadModel();
    const overclaim = TrackingAuthorityHardControlReadModelSchema.safeParse({
      ...readModel,
      surfaces: readModel.surfaces.map((surface) =>
        surface.surface === 'android-device-owner-location-control'
          ? {
              ...surface,
              hardControlClaimed: true,
              productClaimReady: true,
            }
          : surface
      ),
      summary: {
        ...readModel.summary,
        hardControlClaimedRows: 1,
        productClaimReadyRows: 1,
      },
    });

    expect(overclaim.success).toBe(false);
  });

  it('rejects enrolled-device claims before matching authority artifacts exist', () => {
    const readModel = buildTrackingAuthorityHardControlReadModel();
    const enrolledSurface = TrackingAuthorityHardControlSurfaceProofSchema.safeParse({
      ...readModel.surfaces[0],
      authorityEnrolled: true,
      physicalDeviceClaimed: true,
    });

    expect(enrolledSurface.success).toBe(false);
  });
});
