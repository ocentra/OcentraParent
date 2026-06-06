import { describe, expect, it } from 'vitest';
import {
  ProductionSupportRuntimeGapProofSchema,
  summarizeProductionSupportRuntimeGapRows,
} from '../src/production-support-runtime-gap-proof';
import { ProductionSupportRuntimeGapReadModel } from '../src/production-support-runtime-gap-read-model';
import { ForbiddenRuntimeGapDataClasses } from '../src/production-support-runtime-gap-values';

describe('production support runtime gap proof', () => {
  it('accepts the deterministic runtime gap read model', () => {
    const proof = ProductionSupportRuntimeGapProofSchema.parse(ProductionSupportRuntimeGapReadModel);

    expect(summarizeProductionSupportRuntimeGapRows(proof.rows)).toEqual({
      'public-website-runtime-gap': 1,
      'support-publication-execution-gap': 1,
      'support-backend-upload-execution-gap': 1,
      'account-billing-provider-runtime-gap': 1,
      'legal-export-delete-runtime-gap': 1,
      'remote-support-sla-runtime-gap': 1,
    });
    expect(proof.publicRuntimeClaim).toBe('not-implemented');
    expect(proof.supportPublicationExecutionClaim).toBe('manual-required');
    expect(proof.supportBackendUploadExecutionClaim).toBe('manual-required');
    expect(proof.accountBackendRuntimeClaim).toBe('backend-required');
    expect(proof.billingProviderRuntimeClaim).toBe('provider-required');
    expect(proof.legalExportDeleteRuntimeClaim).toBe('manual-required');
    expect(proof.remoteSupportSessionClaim).toBe('not-implemented');
    expect(proof.productionSlaClaim).toBe('not-implemented');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
  });

  it('rejects runtime execution overclaims', () => {
    expect(() =>
      ProductionSupportRuntimeGapProofSchema.parse({
        ...ProductionSupportRuntimeGapReadModel,
        rows: [
          {
            ...ProductionSupportRuntimeGapReadModel.rows[0],
            runtimeExecutionState: 'executed',
          },
          ...ProductionSupportRuntimeGapReadModel.rows.slice(1),
        ],
      })
    ).toThrow(/runtime claims/);
  });

  it('rejects forbidden support data classes', () => {
    expect(() =>
      ProductionSupportRuntimeGapProofSchema.parse({
        ...ProductionSupportRuntimeGapReadModel,
        rows: [
          {
            ...ProductionSupportRuntimeGapReadModel.rows[0],
            supportSafeDataClasses: [
              ...ProductionSupportRuntimeGapReadModel.rows[0].supportSafeDataClasses,
              ForbiddenRuntimeGapDataClasses[0],
            ],
          },
          ...ProductionSupportRuntimeGapReadModel.rows.slice(1),
        ],
      })
    ).toThrow(/exclude child activity/);
  });

  it('rejects missing coverage and required nonclaims', () => {
    expect(() =>
      ProductionSupportRuntimeGapProofSchema.parse({
        ...ProductionSupportRuntimeGapReadModel,
        rows: ProductionSupportRuntimeGapReadModel.rows.slice(1),
      })
    ).toThrow(/cover all required rows/);

    expect(() =>
      ProductionSupportRuntimeGapProofSchema.parse({
        ...ProductionSupportRuntimeGapReadModel,
        nonClaims: ProductionSupportRuntimeGapReadModel.nonClaims.slice(1),
      })
    ).toThrow(/cover all required rows/);
  });
});
