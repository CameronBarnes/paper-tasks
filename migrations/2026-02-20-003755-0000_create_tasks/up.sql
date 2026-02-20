-- Your SQL goes here
CREATE TABLE tasks (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	parent_id INTEGER,
	title TEXT NOT NULL,
	description TEXT,
	scheduled_at TEXT,
	recurrence_rule TEXT,
	archived INTEGER NOT NULL DEFAULT 0,
	created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

	FOREIGN KEY (parent_id) REFERENCES tasks(id) ON DELETE CASCADE
);

CREATE TRIGGER set_tasks_updated_at
BEFORE UPDATE ON tasks
FOR EACH ROW
WHEN NEW.updated_at IS OLD.updated_at
BEGIN
    SELECT NEW.updated_at = CURRENT_TIMESTAMP;
END;

CREATE TABLE task_categories (
	task_id INTEGER NOT NULL,
	category_id INTEGER NOT NULL,
	PRIMARY KEY (task_id, category_id),
	FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
	FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);
