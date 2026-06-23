import { describe, expect, it } from 'vitest';
import {
  AppGamePlatformExtensionRoutingMatrixSchema,
  AppGamePlatformExtensionRoutingRowSchema,
  type AppGamePlatformExtensionProofFile,
  type AppGamePlatformExtensionProofKind,
} from '@ocentra-parent/schema-domain/app-game-platform-extension-routing';
import { AppGamePlatformExtensionRoutingMatrix } from '@ocentra-parent/schema-domain/app-game-platform-extension-routing-data';

const rowFor = (rowId: string) => {
  const row = AppGamePlatformExtensionRoutingMatrix.rows.find((candidate) => candidate.rowId === rowId);

  if (row === undefined) {
    throw new Error(`Missing platform extension row ${rowId}`);
  }

  return row;
};

describe('app/game platform extension routing contracts', () => {
  recordsEveryExtensionChecklistRow();
  linksProofPacksWithoutBareUnsupportedLabels();
  requiresStrongPlatformProofFiles();
  rejectsUnprovedPromotionReadyRows();
  acceptsOnlyFullyProvedHardControlRows();
  rejectsWrongPrefixesDuplicatesAndUnsupportedCopy();
});

function recordsEveryExtensionChecklistRow() {
  it('records every MAC, IOS, ANDROID, and LINUX extension checklist row', () => {
    const matrix = AppGamePlatformExtensionRoutingMatrixSchema.parse(AppGamePlatformExtensionRoutingMatrix);
    const platformCounts = countBy(matrix.rows.map((row) => row.platform));

    expect(matrix.matrixId).toBe('app-game-platform-extension-proof-routing');
    expect(matrix.rows).toHaveLength(52);
    expect(platformCounts).toEqual({
      android: 14,
      ios: 12,
      linux: 14,
      macos: 12,
    });
    expect(matrix.rows.map((row) => row.rowId).slice(0, 3)).toEqual(['MAC-01', 'MAC-02', 'MAC-03']);
    expect(matrix.rows.map((row) => row.rowId).slice(-3)).toEqual(['LINUX-12', 'LINUX-13', 'LINUX-14']);
  });
}

function linksProofPacksWithoutBareUnsupportedLabels() {
  it('links each extension row to app and app-game proof packs without bare unsupported labels', () => {
    const androidDns = rowFor('ANDROID-05');
    const iosFallback = rowFor('IOS-11');
    const labels = AppGamePlatformExtensionRoutingMatrix.rows.map((row) => row.parentVisibleLabel.toLowerCase());

    expect(androidDns.productScope).toBe('platform-handoff');
    expect(androidDns.promotionState).toBe('not-claimed');
    expect(androidDns.crossPlanHandoff).toContain('network-domain-control');
    expect(iosFallback.parentVisibleLabel).toContain('permission, supervision, MDM, or not-claimed labels');
    expect(labels.includes('unsupported')).toBe(false);
    expect(labels.includes('not supported')).toBe(false);
    expect(AppGamePlatformExtensionRoutingMatrix.rows.every((row) => row.appPlanProofPackRef.includes(row.rowId))).toBe(
      true
    );
    expect(AppGamePlatformExtensionRoutingMatrix.rows.every((row) => row.appGameProofPackRef.includes(row.rowId))).toBe(
      true
    );
  });
}

function requiresStrongPlatformProofFiles() {
  it('requires strong platform rows to name authority, setup, and rollback proof files', () => {
    const macosHardBlock = rowFor('MAC-12');
    const androidHide = rowFor('ANDROID-08');
    const linuxScope = rowFor('LINUX-11');

    expect(macosHardBlock.requiredProofFiles).toContain('11-authority-tier-proof.md');
    expect(macosHardBlock.requiredProofFiles).toContain('12-permission-setup-proof.md');
    expect(macosHardBlock.requiredProofFiles).toContain('13-rollback-proof.md');
    expect(androidHide.manualTags).toContain('@requires-device-owner');
    expect(linuxScope.manualTags).toContain('@requires-admin-root');
  });
}

function rejectsUnprovedPromotionReadyRows() {
  it('rejects promotion-ready rows without attached required proof artifacts', () => {
    const androidSuspend = rowFor('ANDROID-09');

    expect(
      AppGamePlatformExtensionRoutingRowSchema.safeParse({
        ...androidSuspend,
        capabilityState: 'supported',
        canPromote: true,
        promotionState: 'promotion-ready',
        proofReferences: [],
      }).success
    ).toBe(false);
  });
}

function acceptsOnlyFullyProvedHardControlRows() {
  it('accepts promoted hard-control rows only when every required proof file is attached', () => {
    const androidSuspend = rowFor('ANDROID-09');
    const promoted = {
      ...androidSuspend,
      capabilityState: 'supported',
      canPromote: true,
      promotionState: 'promotion-ready',
      proofReferences: proofRefsFor(androidSuspend.requiredProofFiles),
    } as const;

    expect(AppGamePlatformExtensionRoutingRowSchema.safeParse(promoted).success).toBe(true);
    expect(
      AppGamePlatformExtensionRoutingRowSchema.safeParse({
        ...promoted,
        proofReferences: promoted.proofReferences.filter(
          (proofReference) => proofReference.proofFile !== '13-rollback-proof.md'
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsWrongPrefixesDuplicatesAndUnsupportedCopy() {
  it('rejects wrong platform prefixes, duplicate rows, and generic unsupported copy', () => {
    const mac = rowFor('MAC-01');

    expect(
      AppGamePlatformExtensionRoutingRowSchema.safeParse({
        ...mac,
        rowId: 'LINUX-99',
      }).success
    ).toBe(false);
    expect(
      AppGamePlatformExtensionRoutingRowSchema.safeParse({
        ...mac,
        parentVisibleLabel: 'Unsupported',
      }).success
    ).toBe(false);
    expect(
      AppGamePlatformExtensionRoutingMatrixSchema.safeParse({
        ...AppGamePlatformExtensionRoutingMatrix,
        rows: [...AppGamePlatformExtensionRoutingMatrix.rows, { ...mac, title: 'duplicate mac row' }],
      }).success
    ).toBe(false);
  });
}

function proofRefsFor(requiredProofFiles: readonly AppGamePlatformExtensionProofFile[]) {
  return requiredProofFiles.map((proofFile) => ({
    proofKind: proofKindFor(proofFile),
    proofFile,
    artifactRef: `output/app-game-plan-proof/platform-extension-promoted/${proofFile}`,
  }));
}

function proofKindFor(proofFile: AppGamePlatformExtensionProofFile): AppGamePlatformExtensionProofKind {
  switch (proofFile) {
    case '00-source-snapshot.md':
      return 'source-snapshot';
    case '03-runtime-evidence.json':
      return 'runtime-evidence-proof';
    case '04-journal-sqlite-proof.json':
      return 'journal-sqlite-proof';
    case '05-policy-action-proof.json':
      return 'policy-action-proof';
    case '07-playwright-ui-proof.log':
      return 'ui-proof';
    case '08-security-negative-proof.log':
      return 'security-negative-proof';
    case '09-manual-platform-proof.md':
      return 'manual-platform-proof';
    case '11-authority-tier-proof.md':
      return 'authority-tier-proof';
    case '12-permission-setup-proof.md':
      return 'permission-setup-proof';
    case '13-rollback-proof.md':
      return 'rollback-proof';
    default:
      return 'contract-proof';
  }
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
