import { invoke } from '@tauri-apps/api/core';
import { ApiError } from '../api/errors';
import type { BackendVersion } from '../api/types';
import type { ApiTransport } from './types';
import { requestMethod } from './types';

const DESKTOP_HEALTH_COMMAND = 'desktop_api_health' as const;

function unsupported(path: string, method: string): never {
  throw new ApiError(`Desktop route is not implemented yet: ${method} ${path}`, {
    status: 0,
    code: 'DESKTOP_ROUTE_UNSUPPORTED',
  });
}

export const desktopTransport: ApiTransport = {
  kind: 'desktop',
  async request<T>(path: string, init?: RequestInit) {
    const method = requestMethod(init);
    if (method === 'GET' && path === '/health') {
      return (await invoke<BackendVersion>(DESKTOP_HEALTH_COMMAND)) as T;
    }
    return unsupported(path, method);
  },
};
