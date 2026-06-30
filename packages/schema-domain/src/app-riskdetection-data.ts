/* thin adapter over Rust-generated app risk detection proof data */

import { AppRiskDetectionMatrixSchema } from './app-riskdetection';
import { GeneratedAppRiskDetectionMatrix } from './generated/app-riskdetection-contracts';

export const AppRiskDetectionMatrix = AppRiskDetectionMatrixSchema.parse(GeneratedAppRiskDetectionMatrix);
