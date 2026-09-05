import {
  isExactRecord,
  isUnknownArray,
  readBoolean,
  readLiteral,
  readNonEmptyTextArray,
} from './route-live-activity-app-game-extension-values';

export type AppGamePlatformExtensionState = 'ready' | 'manual-required' | 'unavailable';
type AppGamePlatformExtensionRowState = Exclude<AppGamePlatformExtensionState, 'unavailable'>;
type AppGamePlatformExtensionProofPackState = 'proof-pack-ready' | 'manual-proof-pack-required';

export type AppGamePlatformExtensionRow = Readonly<{
  platform: 'macos' | 'ios' | 'android' | 'linux';
  state: AppGamePlatformExtensionRowState;
  setupState: AppGamePlatformExtensionRowState;
  proofPackState: AppGamePlatformExtensionProofPackState;
  authorityTier: 'scoped-execution-only' | 'visibility-only' | 'not-locally-provable';
  adapterExecutionClaim: 'not-executed';
  broadBlockingClaimed: boolean;
  privilegedMobileClaimed: boolean;
  childDeviceDeliveryClaimed: boolean;
  requiredProofRefs: readonly string[];
}>;

const ROW_FIELDS = [
  'platform',
  'state',
  'setupState',
  'proofPackState',
  'authorityTier',
  'adapterExecutionClaim',
  'broadBlockingClaimed',
  'privilegedMobileClaimed',
  'childDeviceDeliveryClaimed',
  'requiredProofRefs',
] as const;
const ROW_STATES = ['ready', 'manual-required'] as const;
const PLATFORMS = ['macos', 'ios', 'android', 'linux'] as const;
const PROOF_PACK_STATES = ['proof-pack-ready', 'manual-proof-pack-required'] as const;
const AUTHORITY_TIERS = ['scoped-execution-only', 'visibility-only', 'not-locally-provable'] as const;
const ADAPTER_EXECUTION_CLAIMS = ['not-executed'] as const;

export function readAppGamePlatformExtensionRows(value: unknown): readonly AppGamePlatformExtensionRow[] {
  if (!isUnknownArray(value)) throw new TypeError('app/game platform extension rows must be an array');
  const rows: AppGamePlatformExtensionRow[] = [];
  const platforms = new Set<string>();
  for (const candidate of value) {
    const row = readRow(candidate);
    if (platforms.has(row.platform)) throw new TypeError('app/game platform extension platform must be unique');
    platforms.add(row.platform);
    rows.push(row);
  }
  return rows;
}

export function appGamePlatformExtensionStateForRows(
  rows: readonly AppGamePlatformExtensionRow[]
): AppGamePlatformExtensionState {
  if (rows.length === 0) return 'unavailable';
  return rows.some((row) => row.state === 'manual-required') ? 'manual-required' : 'ready';
}

function readRow(value: unknown): AppGamePlatformExtensionRow {
  if (!isExactRecord(value, ROW_FIELDS)) throw new TypeError('invalid app/game platform extension row');
  const state = readLiteral(value['state'], ROW_STATES);
  const setupState = readLiteral(value['setupState'], ROW_STATES);
  const proofPackState = readLiteral(value['proofPackState'], PROOF_PACK_STATES);
  if (setupState !== state || proofPackState !== proofPackStateFor(state)) {
    throw new TypeError('inconsistent app/game platform extension state');
  }
  return {
    platform: readLiteral(value['platform'], PLATFORMS),
    state,
    setupState,
    proofPackState,
    authorityTier: readLiteral(value['authorityTier'], AUTHORITY_TIERS),
    adapterExecutionClaim: readLiteral(value['adapterExecutionClaim'], ADAPTER_EXECUTION_CLAIMS),
    broadBlockingClaimed: readBoolean(value['broadBlockingClaimed']),
    privilegedMobileClaimed: readBoolean(value['privilegedMobileClaimed']),
    childDeviceDeliveryClaimed: readBoolean(value['childDeviceDeliveryClaimed']),
    requiredProofRefs: readNonEmptyTextArray(value['requiredProofRefs']),
  };
}

function proofPackStateFor(state: AppGamePlatformExtensionRowState): AppGamePlatformExtensionProofPackState {
  return state === 'ready' ? 'proof-pack-ready' : 'manual-proof-pack-required';
}
