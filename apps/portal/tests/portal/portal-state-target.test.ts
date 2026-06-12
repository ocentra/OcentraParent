import { expect, it } from 'vitest';
import { decodeAgentWebSocketUrl } from '@ocentra-parent/agent-protocol-domain/contracts';
import { createPortalRuntimeState } from '../../src/portal-state';

it('createPortalRuntimeState: keeps LAN-hosted portal commands on the local service route by default', () => {
  const state = createPortalRuntimeState(decodeAgentWebSocketUrl('ws://192.168.2.10:4677/api/dev/ws'));

  expect(state.target).toMatchObject({
    deviceId: 'local-dev-agent',
    platform: 'windows',
    route: 'localhost',
  });
});
