export interface BackendVersion {
  status: string;
  service: string;
  version: string;
  env: string;
}


export interface ProfileDiagnostics {
  dataRoot: string;
  configRoot: string;
  stateRoot: string;
  cacheRoot: string;
  profileDatabase: string;
  runtimeActivation: 'available' | 'unavailable';
}

export interface WorkspaceSummary {
  id: string;
  title: string;
  accessEpoch: string;
}

export interface BoardSummary {
  id: string;
  workspaceId: string;
  title: string;
}

export interface VaultStatus {
  mode: 'session-only';
  durable: 'false';
}
