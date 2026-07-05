import assert from 'node:assert/strict';
import test from 'node:test';

import { classifyArchitectureFiles } from '../../scripts/check-architecture-policy.mjs';

test('architecture wrapper routes generated outputs and generator producers away from generic source checks', () => {
  const files = [
    'packages/schema-domain/src/generated/report-query-custody-contracts.ts',
    'apps/portal/generated/parent-ui-bridge.ts',
    'packages/portal-domain/src/generated/portal-contracts.ts',
    'crates/schema/src/parent_agent_protocol_bridge_ts.rs',
    'crates/schema/src/bin/export_agent_protocol_domain_contract_types.rs',
    'crates/schema/tests/contract/parent_agent_protocol_bridge_defaults.rs',
    'crates/agent-protocol/src/lib.rs',
  ];

  assert.deepEqual(classifyArchitectureFiles(files), {
    generatedFiles: [
      'packages/schema-domain/src/generated/report-query-custody-contracts.ts',
      'apps/portal/generated/parent-ui-bridge.ts',
      'packages/portal-domain/src/generated/portal-contracts.ts',
    ],
    generatorFiles: [
      'crates/schema/src/parent_agent_protocol_bridge_ts.rs',
      'crates/schema/src/bin/export_agent_protocol_domain_contract_types.rs',
    ],
    sourceFiles: [
      'crates/schema/tests/contract/parent_agent_protocol_bridge_defaults.rs',
      'crates/agent-protocol/src/lib.rs',
    ],
  });
});
