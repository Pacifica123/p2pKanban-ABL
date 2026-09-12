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
  mode: 'session-only' | 'secret-service' | 'passphrase';
  state: 'ready' | 'provider-unavailable' | 'provider-locked' | 'provider-corrupt' | 'passphrase-required';
  durable: 'true' | 'false';
  passphraseFallbackAvailable: 'true' | 'false';
}

export interface ColumnSummary {
  id: string;
  boardId: string;
  title: string;
  position: string;
}

export interface CardSummary {
  id: string;
  workspaceId: string;
  boardId: string;
  columnId: string;
  title: string;
  position: string;
  archived: 'true' | 'false';
}

export interface ChecklistSummary {
  id: string;
  cardId: string;
  title: string;
  position: string;
}

export interface ChecklistItemSummary {
  id: string;
  checklistId: string;
  title: string;
  position: string;
  isDone: 'true' | 'false';
}

export interface PendingChangeCount {
  count: string;
}
