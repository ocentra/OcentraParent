import type { ReactElement } from 'react';
import { ScopeToggle } from './ScopeToggle';
import { defaultScopeToggleConfig } from './ScopeToggleConfig';

export function ScopeTogglePreview(): ReactElement {
  return (
    <div
      style={{
        minHeight: defaultScopeToggleConfig.preview.minHeight,
        display: 'grid',
        placeItems: 'center',
        background: defaultScopeToggleConfig.preview.background,
        padding: defaultScopeToggleConfig.preview.padding,
        boxSizing: 'border-box',
      }}
    >
      <ScopeToggle
        options={[
          { value: 'family', label: 'Family' },
          { value: 'device', label: 'Per Device' },
          { value: 'lan', label: 'LAN' },
          { value: 'remote', label: 'Remote' },
        ]}
      />
    </div>
  );
}

export default ScopeTogglePreview;
