-- This file should undo anything in `up.sql`
DROP TABLE task_categories;
DROP TRIGGER IF EXISTS update_tasks_updated_at;
DROP TABLE tasks;
