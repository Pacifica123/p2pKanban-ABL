import { useEffect, useState } from 'react';
import type { FormEvent } from 'react';
import {
  createBoard,
  createWorkspace,
  getVaultStatus,
  listBoards,
  listWorkspaces,
  openBoard,
} from './features/planner/api/workspace';
import { getBackendVersion } from './features/system/api/version';
import { getApiTransportKind } from './shared/api/client';
import type {
  BackendVersion,
  BoardSummary,
  VaultStatus,
  WorkspaceSummary,
} from './shared/api/types';

function message(error: unknown): string {
  return error instanceof Error ? error.message : 'Native operation failed';
}

export default function App() {
  const [health, setHealth] = useState<BackendVersion | null>(null);
  const [vault, setVault] = useState<VaultStatus | null>(null);
  const [workspaces, setWorkspaces] = useState<WorkspaceSummary[]>([]);
  const [selectedWorkspace, setSelectedWorkspace] = useState<WorkspaceSummary | null>(null);
  const [boards, setBoards] = useState<BoardSummary[]>([]);
  const [openedBoard, setOpenedBoard] = useState<BoardSummary | null>(null);
  const [workspaceTitle, setWorkspaceTitle] = useState('');
  const [boardTitle, setBoardTitle] = useState('');
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    void Promise.all([getBackendVersion(), getVaultStatus(), listWorkspaces()])
      .then(([healthValue, vaultValue, workspaceValues]) => {
        setHealth(healthValue);
        setVault(vaultValue);
        setWorkspaces(workspaceValues);
      })
      .catch((reason: unknown) => setError(message(reason)));
  }, []);

  async function chooseWorkspace(workspace: WorkspaceSummary): Promise<void> {
    setError(null);
    setOpenedBoard(null);
    setSelectedWorkspace(workspace);
    try {
      setBoards(await listBoards(workspace.id));
    } catch (reason) {
      setError(message(reason));
    }
  }

  async function submitWorkspace(event: FormEvent): Promise<void> {
    event.preventDefault();
    setError(null);
    try {
      const workspace = await createWorkspace(workspaceTitle);
      setWorkspaceTitle('');
      setWorkspaces((current) => [...current, workspace].sort((a, b) => a.title.localeCompare(b.title)));
      await chooseWorkspace(workspace);
    } catch (reason) {
      setError(message(reason));
    }
  }

  async function submitBoard(event: FormEvent): Promise<void> {
    event.preventDefault();
    if (!selectedWorkspace) return;
    setError(null);
    try {
      const board = await createBoard(selectedWorkspace.id, boardTitle);
      setBoardTitle('');
      setBoards((current) => [...current, board].sort((a, b) => a.title.localeCompare(b.title)));
      setOpenedBoard(await openBoard(selectedWorkspace.id, board.id));
    } catch (reason) {
      setError(message(reason));
    }
  }

  async function chooseBoard(board: BoardSummary): Promise<void> {
    if (!selectedWorkspace) return;
    setError(null);
    try {
      setOpenedBoard(await openBoard(selectedWorkspace.id, board.id));
    } catch (reason) {
      setError(message(reason));
    }
  }

  return (
    <main className="app-shell" aria-labelledby="app-title">
      <header className="topbar">
        <div>
          <p className="eyebrow">ARCH NATIVE · A07 DURABLE LOCAL SLICE</p>
          <h1 id="app-title">p2pKanban</h1>
        </div>
        <div className="status-stack" aria-live="polite">
          <span>mode: <strong>{getApiTransportKind()}</strong></span>
          <span>{health ? `${health.service} ${health.version} · ${health.status}` : 'native IPC…'}</span>
          <span>
            vault: <strong>{vault?.mode ?? 'checking'}</strong>
            {vault ? ` · durable secrets ${vault.durable === 'true' ? 'enabled' : 'disabled'}` : ''}
          </span>
        </div>
      </header>

      {error ? <div className="error-banner" role="alert">{error}</div> : null}

      <section className="planner-grid" aria-label="Local durable planner">
        <aside className="panel">
          <div className="panel-heading">
            <div>
              <p className="kicker">Local profile</p>
              <h2>Workspaces</h2>
            </div>
            <span className="count">{workspaces.length}</span>
          </div>

          <form className="create-row" onSubmit={(event) => void submitWorkspace(event)}>
            <input
              aria-label="Workspace title"
              maxLength={120}
              placeholder="New workspace"
              value={workspaceTitle}
              onChange={(event) => setWorkspaceTitle(event.target.value)}
            />
            <button type="submit">Create</button>
          </form>

          <div className="item-list">
            {workspaces.length === 0 ? <p className="empty">No workspaces yet.</p> : null}
            {workspaces.map((workspace) => (
              <button
                className={selectedWorkspace?.id === workspace.id ? 'item active' : 'item'}
                key={workspace.id}
                type="button"
                onClick={() => void chooseWorkspace(workspace)}
              >
                <span>{workspace.title || 'Untitled migrated workspace'}</span>
                <small>epoch {workspace.accessEpoch}</small>
              </button>
            ))}
          </div>
        </aside>

        <section className="panel">
          <div className="panel-heading">
            <div>
              <p className="kicker">{selectedWorkspace ? selectedWorkspace.title : 'Select a workspace'}</p>
              <h2>Boards</h2>
            </div>
            <span className="count">{boards.length}</span>
          </div>

          <form className="create-row" onSubmit={(event) => void submitBoard(event)}>
            <input
              aria-label="Board title"
              disabled={!selectedWorkspace}
              maxLength={120}
              placeholder={selectedWorkspace ? 'New board' : 'Choose workspace first'}
              value={boardTitle}
              onChange={(event) => setBoardTitle(event.target.value)}
            />
            <button disabled={!selectedWorkspace} type="submit">Create</button>
          </form>

          <div className="item-list">
            {!selectedWorkspace ? <p className="empty">Choose a workspace to load durable boards.</p> : null}
            {selectedWorkspace && boards.length === 0 ? <p className="empty">No boards yet.</p> : null}
            {boards.map((board) => (
              <button
                className={openedBoard?.id === board.id ? 'item active' : 'item'}
                key={board.id}
                type="button"
                onClick={() => void chooseBoard(board)}
              >
                <span>{board.title || 'Untitled migrated board'}</span>
                <small>open</small>
              </button>
            ))}
          </div>
        </section>

        <section className="panel board-stage">
          <p className="kicker">Opened board</p>
          {openedBoard ? (
            <>
              <h2>{openedBoard.title}</h2>
              <p className="lede">
                This board is loaded through typed Tauri IPC → Rust application service → SQLite.
                Close and reopen the native app: the workspace and board remain in the XDG profile.
              </p>
              <div className="durability-badge">Durable local board shell</div>
              <p className="empty">Cards, ordering and checklists arrive in A08.</p>
            </>
          ) : (
            <>
              <h2>Nothing open</h2>
              <p className="empty">Create or choose a board. No localhost backend is involved.</p>
            </>
          )}
        </section>
      </section>

      <footer>
        <span>A07 local workspace/board persistence</span>
        <span>Secrets: session-only boundary · production providers A09</span>
      </footer>
    </main>
  );
}
