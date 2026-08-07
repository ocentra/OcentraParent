import assert from 'node:assert/strict';

import { createLoopbackOnlyTestEnvironment } from './agent-service-process.mjs';

const inheritedEnvironment = {
  OCENTRA_PARENT_AGENT_LOCAL_NETWORK_ENABLED: 'true',
  OCENTRA_PARENT_DEV_NETWORK: 'lan',
  PRESERVED_TEST_VALUE: 'kept',
};
const loopbackEnvironment = createLoopbackOnlyTestEnvironment(inheritedEnvironment);

assert.equal(loopbackEnvironment.OCENTRA_PARENT_DEV_NETWORK, undefined);
assert.equal(loopbackEnvironment.OCENTRA_PARENT_AGENT_LOCAL_NETWORK_ENABLED, undefined);
assert.equal(loopbackEnvironment.PRESERVED_TEST_VALUE, 'kept');
assert.equal(inheritedEnvironment.OCENTRA_PARENT_DEV_NETWORK, 'lan');
assert.equal(inheritedEnvironment.OCENTRA_PARENT_AGENT_LOCAL_NETWORK_ENABLED, 'true');

process.stdout.write('loopback-test-environment-proof: passed\n');
