import { afterEach, describe } from 'vitest';
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

  registerPortalProofTracePipelineSuite(tempDirs);
});
