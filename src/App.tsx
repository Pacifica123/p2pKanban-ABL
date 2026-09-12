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
import {
  createCard,
  createChecklist,
  createChecklistItem,
  createColumn,
  deleteCard,
  deleteChecklist,
  deleteChecklistItem,
  getPendingChangeCount,
  listCards,
  listChecklistItems,
  listChecklists,
  listColumns,
  moveCard,
  setCardArchived,
  swapCardOrder,
  setChecklistItemDone,
} from './features/planner/api/planner';
import { getBackendVersion } from './features/system/api/version';
import { getApiTransportKind } from './shared/api/client';
import type {
  BackendVersion,
  BoardSummary,
  CardSummary,
  ChecklistItemSummary,
  ChecklistSummary,
  ColumnSummary,
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
  const [columns, setColumns] = useState<ColumnSummary[]>([]);
  const [cards, setCards] = useState<CardSummary[]>([]);
  const [selectedCard, setSelectedCard] = useState<CardSummary | null>(null);
  const [checklists, setChecklists] = useState<ChecklistSummary[]>([]);
  const [checklistItems, setChecklistItems] = useState<Record<string, ChecklistItemSummary[]>>({});
  const [pendingCount, setPendingCount] = useState('0');
  const [showArchived, setShowArchived] = useState(false);
  const [workspaceTitle, setWorkspaceTitle] = useState('');
  const [boardTitle, setBoardTitle] = useState('');
  const [columnTitle, setColumnTitle] = useState('');
  const [cardTitle, setCardTitle] = useState('');
  const [cardColumnId, setCardColumnId] = useState('');
  const [checklistTitle, setChecklistTitle] = useState('');
  const [itemDrafts, setItemDrafts] = useState<Record<string, string>>({});
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
    setColumns([]);
    setCards([]);
    setSelectedCard(null);
    setChecklists([]);
    setChecklistItems({});
    setSelectedWorkspace(workspace);
    try {
      setBoards(await listBoards(workspace.id));
    } catch (reason) {
      setError(message(reason));
    }
  }

  async function loadCardDetails(workspaceId: string, card: CardSummary): Promise<void> {
    const lists = await listChecklists(workspaceId, card.id);
    const itemPairs = await Promise.all(
      lists.map(async (checklist) => [checklist.id, await listChecklistItems(workspaceId, checklist.id)] as const),
    );
    setSelectedCard(card);
    setChecklists(lists);
    setChecklistItems(Object.fromEntries(itemPairs));
  }

  async function loadBoard(
    workspace: WorkspaceSummary,
    board: BoardSummary,
    includeArchived = showArchived,
  ): Promise<void> {
    const [columnValues, cardValues, pending] = await Promise.all([
      listColumns(workspace.id, board.id),
      listCards(workspace.id, board.id, includeArchived),
      getPendingChangeCount(workspace.id, board.id),
    ]);
    setColumns(columnValues);
    setCards(cardValues);
    setPendingCount(pending.count);
    setCardColumnId((current) => current || columnValues[0]?.id || '');
    if (selectedCard) {
      const fresh = cardValues.find((card) => card.id === selectedCard.id);
      if (fresh) await loadCardDetails(workspace.id, fresh);
      else {
        setSelectedCard(null);
        setChecklists([]);
        setChecklistItems({});
      }
    }
  }

  async function chooseBoard(board: BoardSummary): Promise<void> {
    if (!selectedWorkspace) return;
    setError(null);
    try {
      const opened = await openBoard(selectedWorkspace.id, board.id);
      setOpenedBoard(opened);
      setSelectedCard(null);
      setChecklists([]);
      setChecklistItems({});
      await loadBoard(selectedWorkspace, opened);
    } catch (reason) {
      setError(message(reason));
    }
  }

  async function refreshBoard(includeArchived = showArchived): Promise<void> {
    if (!selectedWorkspace || !openedBoard) return;
    await loadBoard(selectedWorkspace, openedBoard, includeArchived);
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
      await chooseBoard(board);
    } catch (reason) {
      setError(message(reason));
    }
  }

  async function submitColumn(event: FormEvent): Promise<void> {
    event.preventDefault();
    if (!selectedWorkspace || !openedBoard) return;
    setError(null);
    try {
      const column = await createColumn(selectedWorkspace.id, openedBoard.id, columnTitle);
      setColumnTitle('');
      if (!cardColumnId) setCardColumnId(column.id);
      await refreshBoard();
    } catch (reason) {
      setError(message(reason));
    }
  }

  async function submitCard(event: FormEvent): Promise<void> {
    event.preventDefault();
    if (!selectedWorkspace || !openedBoard || !cardColumnId) return;
    setError(null);
    try {
      await createCard(selectedWorkspace.id, openedBoard.id, cardColumnId, cardTitle);
      setCardTitle('');
      await refreshBoard();
    } catch (reason) {
      setError(message(reason));
    }
  }

  async function mutateCard(operation: () => Promise<unknown>): Promise<void> {
    setError(null);
    try {
      await operation();
      await refreshBoard();
    } catch (reason) {
      setError(message(reason));
    }
  }

  async function submitChecklist(event: FormEvent): Promise<void> {
    event.preventDefault();
    if (!selectedWorkspace || !selectedCard) return;
    setError(null);
    try {
      await createChecklist(selectedWorkspace.id, selectedCard.id, checklistTitle);
      setChecklistTitle('');
      await loadCardDetails(selectedWorkspace.id, selectedCard);
      await refreshBoard();
    } catch (reason) {
      setError(message(reason));
    }
  }

  async function submitChecklistItem(event: FormEvent, checklist: ChecklistSummary): Promise<void> {
    event.preventDefault();
    if (!selectedWorkspace || !selectedCard) return;
    const title = itemDrafts[checklist.id] ?? '';
    setError(null);
    try {
      await createChecklistItem(selectedWorkspace.id, checklist.id, title);
      setItemDrafts((current) => ({ ...current, [checklist.id]: '' }));
      await loadCardDetails(selectedWorkspace.id, selectedCard);
      await refreshBoard();
    } catch (reason) {
      setError(message(reason));
    }
  }

  async function mutateChecklist(operation: () => Promise<unknown>): Promise<void> {
    if (!selectedWorkspace || !selectedCard) return;
    setError(null);
    try {
      await operation();
      await loadCardDetails(selectedWorkspace.id, selectedCard);
      await refreshBoard();
    } catch (reason) {
      setError(message(reason));
    }
  }

  return (
    <main className="app-shell" aria-labelledby="app-title">
      <header className="topbar">
        <div>
          <p className="eyebrow">ARCH NATIVE · A09 SECURE LOCAL-FIRST</p>
          <h1 id="app-title">p2pKanban</h1>
        </div>
        <div className="status-stack" aria-live="polite">
          <span>mode: <strong>{getApiTransportKind()}</strong></span>
          <span>{health ? `${health.service} ${health.version} · ${health.status}` : 'native IPC…'}</span>
          <span>vault: <strong>{vault?.mode ?? 'checking'}</strong>{vault ? ` · ${vault.state} · durable secrets ${vault.durable === 'true' ? 'enabled' : 'disabled'}` : ''}</span>
          <span>local pending changes: <strong>{pendingCount}</strong></span>
        </div>
      </header>

      {error ? <div className="error-banner" role="alert">{error}</div> : null}

      <section className="planner-grid" aria-label="Local durable planner">
        <aside className="panel">
          <div className="panel-heading"><div><p className="kicker">Local profile</p><h2>Workspaces</h2></div><span className="count">{workspaces.length}</span></div>
          <form className="create-row" onSubmit={(event) => void submitWorkspace(event)}>
            <input aria-label="Workspace title" maxLength={120} placeholder="New workspace" value={workspaceTitle} onChange={(event) => setWorkspaceTitle(event.target.value)} />
            <button type="submit">Create</button>
          </form>
          <div className="item-list">
            {workspaces.length === 0 ? <p className="empty">No workspaces yet.</p> : null}
            {workspaces.map((workspace) => (
              <button className={selectedWorkspace?.id === workspace.id ? 'item active' : 'item'} key={workspace.id} type="button" onClick={() => void chooseWorkspace(workspace)}>
                <span>{workspace.title || 'Untitled migrated workspace'}</span><small>epoch {workspace.accessEpoch}</small>
              </button>
            ))}
          </div>
        </aside>

        <section className="panel">
          <div className="panel-heading"><div><p className="kicker">{selectedWorkspace ? selectedWorkspace.title : 'Select a workspace'}</p><h2>Boards</h2></div><span className="count">{boards.length}</span></div>
          <form className="create-row" onSubmit={(event) => void submitBoard(event)}>
            <input aria-label="Board title" disabled={!selectedWorkspace} maxLength={120} placeholder={selectedWorkspace ? 'New board' : 'Choose workspace first'} value={boardTitle} onChange={(event) => setBoardTitle(event.target.value)} />
            <button disabled={!selectedWorkspace} type="submit">Create</button>
          </form>
          <div className="item-list">
            {!selectedWorkspace ? <p className="empty">Choose a workspace to load durable boards.</p> : null}
            {selectedWorkspace && boards.length === 0 ? <p className="empty">No boards yet.</p> : null}
            {boards.map((board) => (
              <button className={openedBoard?.id === board.id ? 'item active' : 'item'} key={board.id} type="button" onClick={() => void chooseBoard(board)}>
                <span>{board.title || 'Untitled migrated board'}</span><small>open</small>
              </button>
            ))}
          </div>
        </section>

        <section className="panel board-stage">
          <p className="kicker">Opened board</p>
          {!openedBoard || !selectedWorkspace ? (
            <><h2>Nothing open</h2><p className="empty">Create or choose a board. No localhost backend is involved.</p></>
          ) : (
            <>
              <div className="board-heading">
                <div><h2>{openedBoard.title}</h2><p className="lede">Cards and checklists commit to SQLite locally. Pending markers are durable but are not claimed as remotely converged until A10.</p></div>
                <label className="toggle"><input type="checkbox" checked={showArchived} onChange={(event) => { const includeArchived = event.target.checked; setShowArchived(includeArchived); void refreshBoard(includeArchived); }} /> show archived</label>
              </div>

              <div className="planner-actions">
                <form className="create-row" onSubmit={(event) => void submitColumn(event)}>
                  <input aria-label="Column title" maxLength={120} placeholder="New column" value={columnTitle} onChange={(event) => setColumnTitle(event.target.value)} />
                  <button type="submit">Add column</button>
                </form>
                <form className="create-row card-create" onSubmit={(event) => void submitCard(event)}>
                  <input aria-label="Card title" disabled={columns.length === 0} maxLength={120} placeholder={columns.length ? 'New card' : 'Create a column first'} value={cardTitle} onChange={(event) => setCardTitle(event.target.value)} />
                  <select aria-label="Card column" disabled={columns.length === 0} value={cardColumnId} onChange={(event) => setCardColumnId(event.target.value)}>
                    {columns.map((column) => <option key={column.id} value={column.id}>{column.title || 'Untitled column'}</option>)}
                  </select>
                  <button disabled={!cardColumnId} type="submit">Add card</button>
                </form>
              </div>

              {columns.length === 0 ? <p className="empty">Create the first column to start the offline planner.</p> : null}
              <div className="kanban-columns">
                {columns.map((column) => (
                  <section className="kanban-column" key={column.id}>
                    <div className="column-heading"><h3>{column.title || 'Untitled column'}</h3><span className="count">{cards.filter((card) => card.columnId === column.id).length}</span></div>
                    <div className="card-stack">
                      {cards.filter((card) => card.columnId === column.id).map((card, cardIndex, columnCards) => (
                        <article className={selectedCard?.id === card.id ? 'kanban-card selected' : 'kanban-card'} key={card.id}>
                          <button className="card-title" type="button" onClick={() => selectedWorkspace && void loadCardDetails(selectedWorkspace.id, card)}>{card.title}</button>
                          {card.archived === 'true' ? <span className="archived-badge">archived</span> : null}
                          <select aria-label={`Move ${card.title}`} value={card.columnId} onChange={(event) => selectedWorkspace && void mutateCard(() => moveCard(selectedWorkspace.id, card.id, event.target.value))}>
                            {columns.map((target) => <option key={target.id} value={target.id}>{target.title || 'Untitled column'}</option>)}
                          </select>
                          <div className="card-actions">
                            <button disabled={cardIndex === 0} type="button" onClick={() => selectedWorkspace && cardIndex > 0 && void mutateCard(() => swapCardOrder(selectedWorkspace.id, card.id, columnCards[cardIndex - 1].id))}>↑</button>
                            <button disabled={cardIndex >= columnCards.length - 1} type="button" onClick={() => selectedWorkspace && cardIndex < columnCards.length - 1 && void mutateCard(() => swapCardOrder(selectedWorkspace.id, card.id, columnCards[cardIndex + 1].id))}>↓</button>
                            <button type="button" onClick={() => selectedWorkspace && void mutateCard(() => setCardArchived(selectedWorkspace.id, card.id, card.archived !== 'true'))}>{card.archived === 'true' ? 'Restore' : 'Archive'}</button>
                            <button className="danger" type="button" onClick={() => selectedWorkspace && void mutateCard(() => deleteCard(selectedWorkspace.id, card.id))}>Delete</button>
                          </div>
                        </article>
                      ))}
                    </div>
                  </section>
                ))}
              </div>

              <section className="card-details">
                {selectedCard ? (
                  <>
                    <div className="panel-heading"><div><p className="kicker">Card details</p><h3>{selectedCard.title}</h3></div><span className="count">{checklists.length} lists</span></div>
                    <form className="create-row" onSubmit={(event) => void submitChecklist(event)}>
                      <input aria-label="Checklist title" maxLength={120} placeholder="New checklist" value={checklistTitle} onChange={(event) => setChecklistTitle(event.target.value)} />
                      <button type="submit">Add checklist</button>
                    </form>
                    <div className="checklist-grid">
                      {checklists.map((checklist) => (
                        <section className="checklist" key={checklist.id}>
                          <div className="checklist-heading"><strong>{checklist.title}</strong><button className="link danger" type="button" onClick={() => selectedWorkspace && void mutateChecklist(() => deleteChecklist(selectedWorkspace.id, checklist.id))}>Delete</button></div>
                          <div className="checklist-items">
                            {(checklistItems[checklist.id] ?? []).map((item) => (
                              <label className="checklist-item" key={item.id}>
                                <input type="checkbox" checked={item.isDone === 'true'} onChange={(event) => selectedWorkspace && void mutateChecklist(() => setChecklistItemDone(selectedWorkspace.id, item.id, event.target.checked))} />
                                <span>{item.title}</span>
                                <button className="link danger" type="button" onClick={() => selectedWorkspace && void mutateChecklist(() => deleteChecklistItem(selectedWorkspace.id, item.id))}>×</button>
                              </label>
                            ))}
                          </div>
                          <form className="create-row compact" onSubmit={(event) => void submitChecklistItem(event, checklist)}>
                            <input aria-label={`New item for ${checklist.title}`} maxLength={120} placeholder="Checklist item" value={itemDrafts[checklist.id] ?? ''} onChange={(event) => setItemDrafts((current) => ({ ...current, [checklist.id]: event.target.value }))} />
                            <button type="submit">Add</button>
                          </form>
                        </section>
                      ))}
                    </div>
                  </>
                ) : <p className="empty">Select a card to manage durable checklists.</p>}
              </section>
            </>
          )}
        </section>
      </section>

      <footer><span>A08 local planner persistence · pending is local-only until A10</span><span>Secrets: session-only boundary · production providers A09</span></footer>
    </main>
  );
}
