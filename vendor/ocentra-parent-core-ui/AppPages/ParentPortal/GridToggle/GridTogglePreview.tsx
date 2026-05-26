import type { ReactElement } from 'react';
import { GridToggle } from './GridToggle';
import { defaultGridToggleConfig } from './GridToggleConfig';

export function GridTogglePreview(): ReactElement {
  return (
    <div
      style={{
        minHeight: defaultGridToggleConfig.preview.minHeight,
        display: 'grid',
        placeItems: 'center',
        background: defaultGridToggleConfig.preview.background,
        padding: defaultGridToggleConfig.preview.padding,
        boxSizing: 'border-box',
      }}
    >
      <GridToggle rows={3} columns={5} />
    </div>
  );
}

export default GridTogglePreview;
