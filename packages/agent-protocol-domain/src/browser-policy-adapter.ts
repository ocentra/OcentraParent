import {
  BrowserControlUpdateRequestSchema,
  BrowserControlUpdateResponseSchema,
  type BrowserControlUpdateRequest,
  type BrowserControlUpdateResponse,
} from '@ocentra-parent/browser-domain/browser-control-policy';
import type { BrowserControlUpdateKind } from '@ocentra-parent/browser-domain/browser-control-values';
import {
  AgentCommand,
  AgentCommandEnvelopeSchema,
  AgentEvent,
  AgentProtocolDefaults,
  type AgentCommandEnvelope,
  type AgentEventEnvelope,
  type AgentEventName,
} from './contracts';
import { AgentProtocolSchemaVersion, type AgentRoute } from './primitives';

export type BrowserPolicyAdapterFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-request'
  | 'invalid-payload';

export type BrowserPolicyAdapterResult =
  | {
      readonly ok: true;
      readonly value: BrowserControlUpdateResponse;
    }
  | {
      readonly ok: false;
      readonly reason: BrowserPolicyAdapterFailureReason;
    };

export type BrowserPolicyCommandPeerInput = {
  readonly peerId: string;
  readonly role: 'portal' | 'agent-service' | 'cloud-relay';
};

export type BrowserPolicyCommandTargetInput = {
  readonly deviceId: string;
  readonly platform: string;
  readonly route: AgentRoute;
};

export type CreateBrowserPolicyCommandInput = {
  readonly messageId: string;
  readonly sentAt: string;
  readonly source: BrowserPolicyCommandPeerInput;
  readonly target: BrowserPolicyCommandTargetInput;
  readonly request: BrowserControlUpdateRequest;
};

export function createBrowserPolicyCommand(input: CreateBrowserPolicyCommandInput): AgentCommandEnvelope {
  const parsedRequest = BrowserControlUpdateRequestSchema.safeParse(input.request);
  if (!parsedRequest.success) {
    throw new Error('invalid browser policy request');
  }
  return AgentCommandEnvelopeSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    messageId: input.messageId,
    sentAt: input.sentAt,
    source: input.source,
    target: input.target,
    command: commandForKind(parsedRequest.data.kind),
    payload: {
      [AgentProtocolDefaults.Field.BrowserPolicyRequest]: JSON.stringify(parsedRequest.data),
      [AgentProtocolDefaults.Field.BrowserPolicyUpdateKind]: parsedRequest.data.kind,
    },
  });
}

export function parseBrowserPolicyUpdateEvent(event: AgentEventEnvelope): BrowserPolicyAdapterResult {
  if (!browserPolicyEventNames().includes(event.event)) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.BrowserPolicyResponse];
  if (typeof raw !== 'string') {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = BrowserControlUpdateResponseSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function commandForKind(kind: BrowserControlUpdateKind): AgentCommandEnvelope['command'] {
  if (kind === 'preview') return AgentCommand.BrowserPolicyPreview;
  if (kind === 'patch') return AgentCommand.BrowserPolicyPatch;
  if (kind === 'replace') return AgentCommand.BrowserPolicyReplace;
  if (kind === 'rollback') return AgentCommand.BrowserPolicyRollback;
  return AgentCommand.BrowserPolicyGet;
}

function browserPolicyEventNames(): AgentEventName[] {
  return [
    AgentEvent.BrowserPolicyReported,
    AgentEvent.BrowserPolicyPreviewed,
    AgentEvent.BrowserPolicyPatchAccepted,
    AgentEvent.BrowserPolicyPatchRejected,
    AgentEvent.BrowserPolicyReplaceAccepted,
    AgentEvent.BrowserPolicyReplaceRejected,
    AgentEvent.BrowserPolicyRollbackAccepted,
    AgentEvent.BrowserPolicyRollbackRejected,
  ];
}

function adapterFailure(reason: BrowserPolicyAdapterFailureReason): BrowserPolicyAdapterResult {
  return {
    ok: false,
    reason,
  };
}
