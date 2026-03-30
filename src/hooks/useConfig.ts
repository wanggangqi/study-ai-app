import { useConfigStore } from '../stores/configStore';

export const useConfig = () => {
  const config = useConfigStore();
  return config;
};
