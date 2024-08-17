PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS todos
(
    id          TEXT PRIMARY KEY NOT NULL,
    title       TEXT             NOT NULL,
    description TEXT,
    due_date    TEXT,
    completed   BOOLEAN          NOT NULL DEFAULT 0,
    priority    INTEGER                   DEFAULT 0,
    created_at  TEXT             NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT             NOT NULL DEFAULT (datetime('now'))
);


CREATE TRIGGER IF NOT EXISTS update_todo_updated_at
    AFTER UPDATE
    ON todos
    FOR EACH ROW
BEGIN
    UPDATE todos SET updated_at = datetime('now') WHERE id = OLD.id;
END;

CREATE INDEX IF NOT EXISTS idx_todos_completed ON todos (completed);

CREATE UNIQUE INDEX IF NOT EXISTS idx_todos_title_due_date
    ON todos (title, due_date);

CREATE TRIGGER IF NOT EXISTS check_priority_range
    BEFORE INSERT
    ON todos
    FOR EACH ROW
    WHEN NEW.priority < 0 OR NEW.priority > 5
BEGIN
    SELECT RAISE(FAIL, 'Priority must be between 0 and 5');
END;
