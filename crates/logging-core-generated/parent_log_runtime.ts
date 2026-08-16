/* generated from crates/logging-core/src/parent_log_runtime.rs */

import { buildGeneratedParentLogConfig as buildGeneratedParentLogConfigImpl } from './parent-log-runtime-config';
import {
  buildGeneratedRunStartedPayload as buildGeneratedRunStartedPayloadImpl,
  generatedHasRunInfoConflict as generatedHasRunInfoConflictImpl,
  generatedStaleRunInfoWarning as generatedStaleRunInfoWarningImpl,
  normalizeGeneratedBridgeEndpoint as normalizeGeneratedBridgeEndpointImpl,
  resolveGeneratedBridgeRoute as resolveGeneratedBridgeRouteImpl,
} from './parent-log-runtime-bridge';
import {
  isGeneratedDevOrTestEnvironment as isGeneratedDevOrTestEnvironmentImpl,
  isGeneratedLevelAtOrAbove as isGeneratedLevelAtOrAboveImpl,
} from './parent-log-runtime-level';
import { matchesGeneratedDebugSelection as matchesGeneratedDebugSelectionImpl } from './parent-log-runtime-selection';
import {
  shouldGeneratedLogToConsole as shouldGeneratedLogToConsoleImpl,
  shouldGeneratedStoreLog as shouldGeneratedStoreLogImpl,
} from './parent-log-runtime-policy';
import {
  normalizeGeneratedDebugPath as normalizeGeneratedDebugPathImpl,
  parseGeneratedBoolean as parseGeneratedBooleanImpl,
  parseGeneratedBridgeMode as parseGeneratedBridgeModeImpl,
  parseGeneratedLevel as parseGeneratedLevelImpl,
  parseGeneratedList as parseGeneratedListImpl,
} from './parent-log-runtime-parsing';

export const parseGeneratedBoolean = parseGeneratedBooleanImpl;
export const parseGeneratedList = parseGeneratedListImpl;
export const parseGeneratedLevel = parseGeneratedLevelImpl;
export const parseGeneratedBridgeMode = parseGeneratedBridgeModeImpl;
export const normalizeGeneratedDebugPath = normalizeGeneratedDebugPathImpl;
export const isGeneratedLevelAtOrAbove = isGeneratedLevelAtOrAboveImpl;
export const isGeneratedDevOrTestEnvironment = isGeneratedDevOrTestEnvironmentImpl;
export const matchesGeneratedDebugSelection = matchesGeneratedDebugSelectionImpl;
export const shouldGeneratedLogToConsole = shouldGeneratedLogToConsoleImpl;
export const shouldGeneratedStoreLog = shouldGeneratedStoreLogImpl;
export const buildGeneratedParentLogConfig = buildGeneratedParentLogConfigImpl;
export const normalizeGeneratedBridgeEndpoint = normalizeGeneratedBridgeEndpointImpl;
export const resolveGeneratedBridgeRoute = resolveGeneratedBridgeRouteImpl;
export const generatedStaleRunInfoWarning = generatedStaleRunInfoWarningImpl;
export const generatedHasRunInfoConflict = generatedHasRunInfoConflictImpl;
export const buildGeneratedRunStartedPayload = buildGeneratedRunStartedPayloadImpl;
