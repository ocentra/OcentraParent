import type { ReactElement } from 'react';
import { DeviceChoiceGrid } from './DeviceChoiceGrid';
import { defaultDeviceChoiceGridConfig } from './DeviceChoiceGridConfig';

export function DeviceChoiceGridPreview(): ReactElement {
  return (
    <div style={defaultDeviceChoiceGridConfig.preview}>
      <DeviceChoiceGrid />
    </div>
  );
}

export default DeviceChoiceGridPreview;
