/* generated from crates/logging-core/src/parent_log_runtime.rs */

import {
  normalizeGeneratedBridgeEndpoint as normalizeGeneratedBridgeEndpointImpl,
  resolveGeneratedBridgeRoute as resolveGeneratedBridgeRouteImpl,
} from './parent-log-runtime-route';
import {
  buildGeneratedRunStartedPayload as buildGeneratedRunStartedPayloadImpl,
  generatedHasRunInfoConflict as generatedHasRunInfoConflictImpl,
  generatedStaleRunInfoWarning as generatedStaleRunInfoWarningImpl,
} from './parent-log-runtime-state';

export const buildGeneratedRunStartedPayload = buildGeneratedRunStartedPayloadImpl;
export const generatedHasRunInfoConflict = generatedHasRunInfoConflictImpl;
export const generatedStaleRunInfoWarning = generatedStaleRunInfoWarningImpl;
export const normalizeGeneratedBridgeEndpoint = normalizeGeneratedBridgeEndpointImpl;
export const resolveGeneratedBridgeRoute = resolveGeneratedBridgeRouteImpl;
