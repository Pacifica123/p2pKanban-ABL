import { invoke } from '@tauri-apps/api/core';
import { ApiError } from '../api/errors';
import type {
  BackendVersion,
  BoardSummary,
  ProfileDiagnostics,
  VaultStatus,
  WorkspaceSummary,
} from '../api/types';
import type { ApiTransport } from './types';
import { requestMethod } from './types';

const DESKTOP_HEALTH_COMMAND = 'desktop_api_health' as const;
const DESKTOP_PROFILE_DIAGNOSTICS_COMMAND = 'desktop_api_profile_diagnostics' as const;
const DESKTOP_VAULT_STATUS_COMMAND = 'desktop_api_vault_status' as const;
const DESKTOP_LIST_WORKSPACES_COMMAND = 'desktop_api_list_workspaces' as const;
const DESKTOP_CREATE_WORKSPACE_COMMAND = 'desktop_api_create_workspace' as const;
const DESKTOP_LIST_BOARDS_COMMAND = 'desktop_api_list_boards' as const;
const DESKTOP_CREATE_BOARD_COMMAND = 'desktop_api_create_board' as const;
const DESKTOP_OPEN_BOARD_COMMAND = 'desktop_api_open_board' as const;

function unsupported(path: string, method: string): never {
  throw new ApiError(`Desktop route is not implemented yet: ${method} ${path}`, {
    status: 0,
    code: 'DESKTOP_ROUTE_UNSUPPORTED',
  });
}

function jsonBody<T extends Record<string, unknown>>(init?: RequestInit): T {
  if (typeof init?.body !== 'string') {
    throw new ApiError('Desktop JSON request body is required.', {
      status: 0,
      code: 'DESKTOP_BODY_REQUIRED',
    });
  }
  try {
    const parsed = JSON.parse(init.body) as unknown;
    if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) {
      throw new Error('body is not an object');
    }
    return parsed as T;
  } catch {
    throw new ApiError('Desktop JSON request body is invalid.', {
      status: 0,
      code: 'DESKTOP_BODY_INVALID',
    });
  }
}

export const desktopTransport: ApiTransport = {
  kind: 'desktop',
  async request<T>(path: string, init?: RequestInit) {
    const method = requestMethod(init);
    if (method === 'GET' && path === '/health') {
      return (await invoke<BackendVersion>(DESKTOP_HEALTH_COMMAND)) as T;
    }
    if (method === 'GET' && path === '/system/profile-diagnostics') {
      return (await invoke<ProfileDiagnostics>(DESKTOP_PROFILE_DIAGNOSTICS_COMMAND)) as T;
    }
    if (method === 'GET' && path === '/system/vault-status') {
      return (await invoke<VaultStatus>(DESKTOP_VAULT_STATUS_COMMAND)) as T;
    }
    if (method === 'GET' && path === '/planner/workspaces') {
      return (await invoke<WorkspaceSummary[]>(DESKTOP_LIST_WORKSPACES_COMMAND)) as T;
    }
    if (method === 'POST' && path === '/planner/workspaces') {
      const body = jsonBody<{ title: string }>(init);
      return (await invoke<WorkspaceSummary>(DESKTOP_CREATE_WORKSPACE_COMMAND, {
        title: body.title,
      })) as T;
    }
    if (method === 'POST' && path === '/planner/boards/list') {
      const body = jsonBody<{ workspaceId: string }>(init);
      return (await invoke<BoardSummary[]>(DESKTOP_LIST_BOARDS_COMMAND, {
        workspaceId: body.workspaceId,
      })) as T;
    }
    if (method === 'POST' && path === '/planner/boards') {
      const body = jsonBody<{ workspaceId: string; title: string }>(init);
      return (await invoke<BoardSummary>(DESKTOP_CREATE_BOARD_COMMAND, {
        workspaceId: body.workspaceId,
        title: body.title,
      })) as T;
    }
    if (method === 'POST' && path === '/planner/boards/open') {
      const body = jsonBody<{ workspaceId: string; boardId: string }>(init);
      return (await invoke<BoardSummary>(DESKTOP_OPEN_BOARD_COMMAND, {
        workspaceId: body.workspaceId,
        boardId: body.boardId,
      })) as T;
    }
    return unsupported(path, method);
  },
};
