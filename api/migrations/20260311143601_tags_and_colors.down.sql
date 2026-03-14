-- Add down migration script here
-- DropForeignKey
ALTER TABLE public.workspace_tag DROP CONSTRAINT workspace_tag_workspace_id_fkey;

-- DropForeignKey
ALTER TABLE public.task_tag DROP CONSTRAINT task_tag_task_id_fkey;

-- DropForeignKey
ALTER TABLE public.task_tag DROP CONSTRAINT task_tag_tag_id_fkey;

-- AlterTable
ALTER TABLE public.task_status DROP COLUMN color;

-- AlterTable
ALTER TABLE public.status_preset DROP COLUMN color;

-- DropTable
DROP TABLE public.workspace_tag;

-- DropTable
DROP TABLE public.task_tag;

-- DropEnum
DROP TYPE public.color;
