import {
  V08EnforcementIntegrityRuntimeAuditReadModelSchema,
  type V08EnforcementIntegrityRuntimeAuditReadModel,
  type V08IntegrityAlertStatusBridgeReadModel,
  type V08NotificationProviderStatusBoundaryReadModel,
  V08SupportedAdapterRuntimeProofReadModelSchema,
  type V08SupportedAdapterRuntimeProofReadModel,
} from '@ocentra-parent/parent-domain/v0-8-supported-adapter-runtime-proof';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from './contracts';

export type EnforcementSupportedAdapterRuntimeProofParseResult =
  | {
      readonly status: 'accepted';
      readonly readModel: V08SupportedAdapterRuntimeProofReadModel;
      readonly integrityAuditReadModel: V08EnforcementIntegrityRuntimeAuditReadModel;
      readonly integrityAlertStatusBridge: V08IntegrityAlertStatusBridgeReadModel;
      readonly notificationProviderStatusBoundary: V08NotificationProviderStatusBoundaryReadModel;
    }
  | {
      readonly status: 'rejected';
      readonly reason:
        | 'unexpected-event'
        | 'missing-read-model'
        | 'invalid-read-model-json'
        | 'invalid-read-model'
        | 'missing-integrity-audit-read-model'
        | 'invalid-integrity-audit-read-model-json'
        | 'invalid-integrity-audit-read-model';
    };

export function parseEnforcementSupportedAdapterRuntimeProofEvent(
  event: AgentEventEnvelope
): EnforcementSupportedAdapterRuntimeProofParseResult {
  if (event.event !== AgentEvent.EnforcementSupportedAdapterRuntimeProofReported) {
    return { status: 'rejected', reason: 'unexpected-event' };
  }

  const rawReadModel = event.payload[AgentProtocolDefaults.Field.EnforcementSupportedAdapterRuntimeProofReadModel];
  if (typeof rawReadModel !== 'string' || rawReadModel.trim().length === 0) {
    return { status: 'rejected', reason: 'missing-read-model' };
  }

  const decoded = parseJson(rawReadModel);
  if (decoded.status === 'rejected') {
    return decoded;
  }

  const parsed = V08SupportedAdapterRuntimeProofReadModelSchema.safeParse(decoded.value);
  if (!parsed.success) {
    return { status: 'rejected', reason: 'invalid-read-model' };
  }

  const rawAuditReadModel = event.payload[AgentProtocolDefaults.Field.EnforcementIntegrityRuntimeAuditReadModel];
  if (typeof rawAuditReadModel !== 'string' || rawAuditReadModel.trim().length === 0) {
    return { status: 'rejected', reason: 'missing-integrity-audit-read-model' };
  }

  const decodedAudit = parseIntegrityAuditJson(rawAuditReadModel);
  if (decodedAudit.status === 'rejected') {
    return decodedAudit;
  }

  const parsedAudit = V08EnforcementIntegrityRuntimeAuditReadModelSchema.safeParse(decodedAudit.value);
  if (!parsedAudit.success) {
    return { status: 'rejected', reason: 'invalid-integrity-audit-read-model' };
  }

  return {
    status: 'accepted',
    readModel: parsed.data,
    integrityAuditReadModel: parsedAudit.data,
    integrityAlertStatusBridge: parsedAudit.data.integrityAlertStatusBridge,
    notificationProviderStatusBoundary: parsedAudit.data.notificationProviderStatusBoundary,
  };
}

function parseJson(value: string):
  | {
      readonly status: 'accepted';
      readonly value: unknown;
    }
  | {
      readonly status: 'rejected';
      readonly reason: 'invalid-read-model-json';
    } {
  try {
    return { status: 'accepted', value: JSON.parse(value) as unknown };
  } catch {
    return { status: 'rejected', reason: 'invalid-read-model-json' };
  }
}

function parseIntegrityAuditJson(value: string):
  | {
      readonly status: 'accepted';
      readonly value: unknown;
    }
  | {
      readonly status: 'rejected';
      readonly reason: 'invalid-integrity-audit-read-model-json';
    } {
  try {
    return { status: 'accepted', value: JSON.parse(value) as unknown };
  } catch {
    return { status: 'rejected', reason: 'invalid-integrity-audit-read-model-json' };
  }
}
