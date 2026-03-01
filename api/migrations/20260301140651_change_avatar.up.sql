-- Add up migration script here
-- AlterTable
ALTER TABLE app_user ADD COLUMN     avatar_preview TEXT;

-- DropTable
DROP TABLE avatar;

