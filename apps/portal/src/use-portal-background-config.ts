import { useEffect, useState } from 'react';
import {
  loadPortalBackgroundConfig,
  readDefaultPortalBackgroundConfig,
  subscribePortalBackgroundConfig,
  type PortalBackgroundConfig,
} from './portal-background-config';

export function usePortalBackgroundConfig(): readonly [
  PortalBackgroundConfig,
  (config: PortalBackgroundConfig) => void,
] {
  const [config, setConfig] = useState<PortalBackgroundConfig>(() => readDefaultPortalBackgroundConfig());

  useEffect(() => {
    let active = true;
    void loadPortalBackgroundConfig().then((loadedConfig) => {
      if (active) {
        setConfig(loadedConfig);
      }
    });
    const unsubscribe = subscribePortalBackgroundConfig(setConfig);
    return () => {
      active = false;
      unsubscribe();
    };
  }, []);

  const updateConfig = (nextConfig: PortalBackgroundConfig): void => {
    setConfig(nextConfig);
  };

  return [config, updateConfig];
}
