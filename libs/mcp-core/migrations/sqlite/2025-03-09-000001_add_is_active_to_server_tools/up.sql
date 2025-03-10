-- Add is_active column to server_tools table with default value of true
ALTER TABLE server_tools ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT true;
