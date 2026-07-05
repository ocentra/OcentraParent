import { describe, expect, it } from 'vitest';

import {
  ChildSigningStoreDeviceOwnerMatrixProofReadModel,
  ChildSigningStoreDeviceOwnerMatrixSchema,
  RequiredChildArtifactMatrixPlatforms,
} from '../../src/child-signing-store-device-owner-matrix';
import { GeneratedChildSigningStoreDeviceOwnerMatrixProof } from '../../src/generated-child-signing-store-device-owner-matrix-contracts';

describe('childSigningStoreDeviceOwnerMatrix', () => {
  it('parses the Rust-owned generated proof through the thin adapter', () => {
    expect(ChildSigningStoreDeviceOwnerMatrixProofReadModel.rows).toHaveLength(
      RequiredChildArtifactMatrixPlatforms.length
    );
    expect(ChildSigningStoreDeviceOwnerMatrixProofReadModel.rows[0]?.platform).toBe('windows');
    expect(ChildSigningStoreDeviceOwnerMatrixProofReadModel.rows[0]?.signingState).toBe('unsigned');
    expect(ChildSigningStoreDeviceOwnerMatrixProofReadModel.rows[3]?.deviceOwnerState).toBe(
      'manual-required'
    );
    expect(ChildSigningStoreDeviceOwnerMatrixProofReadModel.rows[4]?.supervisionState).toBe(
      'device-proof-required'
    );
  });

  it('rejects missing platform coverage', () => {
    const invalid = structuredClone(GeneratedChildSigningStoreDeviceOwnerMatrixProof);
    invalid.rows = invalid.rows.filter((row) => row.platform !== 'ios');

    expect(() => ChildSigningStoreDeviceOwnerMatrixSchema.parse(invalid)).toThrow(
      /Expected exactly one generated matrix row/
    );
  });

  it('rejects duplicate platform coverage', () => {
    const invalid = structuredClone(GeneratedChildSigningStoreDeviceOwnerMatrixProof);
    invalid.rows = [
      ...invalid.rows.slice(0, 4),
      {
        ...invalid.rows[4],
        platform: 'windows',
      },
    ];

    expect(() => ChildSigningStoreDeviceOwnerMatrixSchema.parse(invalid)).toThrow(
      /Expected exactly one generated matrix row/
    );
  });

  it('rejects rows that lose proof references', () => {
    const invalid = structuredClone(GeneratedChildSigningStoreDeviceOwnerMatrixProof);
    invalid.rows[0] = {
      ...invalid.rows[0],
      proofRefs: [],
    };

    expect(() => ChildSigningStoreDeviceOwnerMatrixSchema.parse(invalid)).toThrow(
      /Expected every artifact row to keep at least one proof reference/
    );
  });
});
