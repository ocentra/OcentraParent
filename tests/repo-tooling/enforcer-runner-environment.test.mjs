import assert from 'node:assert/strict';
import test from 'node:test';

import { cleanGitEnvironment } from '../../scripts/enforcer/run-ocentra-enforcer.mjs';

test('external Enforcer runner excludes Git hook environment variables', () => {
  const environment = cleanGitEnvironment({
    GIT_DIR: 'E:/OcentraParent/.git/worktrees/OcentraParent',
    GIT_INDEX_FILE: 'E:/OcentraParent/.git/worktrees/OcentraParent/index',
    OCENTRA_ENFORCER_HOME: 'E:/ocentra-enforcer',
    PATH: 'C:/Program Files/nodejs',
  });

  assert.deepEqual(environment, {
    OCENTRA_ENFORCER_HOME: 'E:/ocentra-enforcer',
    PATH: 'C:/Program Files/nodejs',
  });
});
