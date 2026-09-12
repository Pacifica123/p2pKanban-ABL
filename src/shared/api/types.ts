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
