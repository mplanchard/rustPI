-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS idx_name_version;
DROP TABLE IF EXISTS packages;
