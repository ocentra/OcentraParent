import { afterEach, describe, expect, it } from 'vitest';
import { localArtifactDirectoryDurability } from '../../../../packages/logging-domain/src/local-artifact-path';
import { registerPortalProofTracePipelineSuite } from './portal-proof-trace-pipeline.test.helpers';

describe('portal proof trace pipeline', () => {
  const tempDirs: string[] = [];
  const originalStructuredRoot = process.env['OCENTRA_PARENT_LOG_DIR'];

  afterEach(() => {
    if (originalStructuredRoot == null) {
      delete process.env['OCENTRA_PARENT_LOG_DIR'];
    } else {
      process.env['OCENTRA_PARENT_LOG_DIR'] = originalStructuredRoot;
    }
  });

  if (localArtifactDirectoryDurability() === 'mutation-unsupported') {
    it('reports unavailable local artifact mutation instead of fabricating pipeline proof', () => {
      expect(localArtifactDirectoryDurability()).toBe('mutation-unsupported');
    });
  } else {
    registerPortalProofTracePipelineSuite(tempDirs);
  }
});
