-- Add down migration script here
-- DropForeignKey
ALTER TABLE public.app_workspace DROP CONSTRAINT app_workspace_admin_id_fkey;

-- DropForeignKey
ALTER TABLE public.user_workspace_filter DROP CONSTRAINT user_workspace_filter_user_id_fkey;

-- DropForeignKey
ALTER TABLE public.user_workspace_filter DROP CONSTRAINT user_workspace_filter_workspace_id_fkey;

-- DropForeignKey
ALTER TABLE public.workspace_member DROP CONSTRAINT workspace_member_user_id_fkey;

-- DropForeignKey
ALTER TABLE public.workspace_member DROP CONSTRAINT workspace_member_workspace_id_fkey;

-- DropForeignKey
ALTER TABLE public.workspace_invitation DROP CONSTRAINT workspace_invitation_user_id_fkey;

-- DropForeignKey
ALTER TABLE public.workspace_invitation DROP CONSTRAINT workspace_invitation_workspace_id_fkey;

-- DropForeignKey
ALTER TABLE public.task_status DROP CONSTRAINT task_status_workspace_id_fkey;

-- DropForeignKey
ALTER TABLE public.task DROP CONSTRAINT task_workspace_id_fkey;

-- DropForeignKey
ALTER TABLE public.task DROP CONSTRAINT task_author_id_fkey;

-- DropForeignKey
ALTER TABLE public.task DROP CONSTRAINT task_executor_id_fkey;

-- DropForeignKey
ALTER TABLE public.task DROP CONSTRAINT task_status_id_fkey;

-- DropForeignKey
ALTER TABLE public.message DROP CONSTRAINT message_task_id_fkey;

-- DropForeignKey
ALTER TABLE public.message DROP CONSTRAINT message_author_id_fkey;

-- DropForeignKey
ALTER TABLE public.read_mark DROP CONSTRAINT read_mark_task_id_fkey;

-- DropForeignKey
ALTER TABLE public.read_mark DROP CONSTRAINT read_mark_user_id_fkey;

-- DropForeignKey
ALTER TABLE public.read_mark DROP CONSTRAINT read_mark_message_id_fkey;

-- DropForeignKey
ALTER TABLE public.message_file DROP CONSTRAINT message_file_message_id_fkey;

-- DropForeignKey
ALTER TABLE public.message_file DROP CONSTRAINT message_file_uploader_id_fkey;

-- DropForeignKey
ALTER TABLE public.change_status_event DROP CONSTRAINT change_status_event_task_id_fkey;

-- DropForeignKey
ALTER TABLE public.change_status_event DROP CONSTRAINT change_status_event_who_id_fkey;

-- DropForeignKey
ALTER TABLE public.change_status_event DROP CONSTRAINT change_status_event_from_id_fkey;

-- DropForeignKey
ALTER TABLE public.change_status_event DROP CONSTRAINT change_status_event_to_id_fkey;

-- DropForeignKey
ALTER TABLE public.status_interval DROP CONSTRAINT status_interval_status_id_fkey;

-- DropForeignKey
ALTER TABLE public.status_interval DROP CONSTRAINT status_interval_user_id_fkey;

-- DropForeignKey
ALTER TABLE public.status_interval DROP CONSTRAINT status_interval_task_id_fkey;

-- DropForeignKey
ALTER TABLE public.statuses_preset DROP CONSTRAINT statuses_preset_user_id_fkey;

-- DropForeignKey
ALTER TABLE public.status_preset DROP CONSTRAINT status_preset_preset_id_fkey;

-- DropForeignKey
ALTER TABLE public.task_preset DROP CONSTRAINT task_preset_workspace_id_fkey;

-- DropForeignKey
ALTER TABLE public.task_preset DROP CONSTRAINT task_preset_executor_id_fkey;

-- DropForeignKey
ALTER TABLE public.task_preset DROP CONSTRAINT task_preset_status_id_fkey;

-- DropForeignKey
ALTER TABLE public.workspace_reaction DROP CONSTRAINT workspace_reaction_user_id_fkey;

-- DropForeignKey
ALTER TABLE public.workspace_reaction DROP CONSTRAINT workspace_reaction_workspace_id_fkey;

-- DropForeignKey
ALTER TABLE public.task_status_reaction DROP CONSTRAINT task_status_reaction_user_id_fkey;

-- DropForeignKey
ALTER TABLE public.task_status_reaction DROP CONSTRAINT task_status_reaction_task_id_fkey;

-- DropTable
DROP TABLE public.app_user;

-- DropTable
DROP TABLE public.app_workspace;

-- DropTable
DROP TABLE public.user_workspace_filter;

-- DropTable
DROP TABLE public.workspace_member;

-- DropTable
DROP TABLE public.workspace_invitation;

-- DropTable
DROP TABLE public.task_status;

-- DropTable
DROP TABLE public.task;

-- DropTable
DROP TABLE public.message;

-- DropTable
DROP TABLE public.read_mark;

-- DropTable
DROP TABLE public.message_file;

-- DropTable
DROP TABLE public.avatar;

-- DropTable
DROP TABLE public.change_status_event;

-- DropTable
DROP TABLE public.status_interval;

-- DropTable
DROP TABLE public.statuses_preset;

-- DropTable
DROP TABLE public.status_preset;

-- DropTable
DROP TABLE public.task_preset;

-- DropTable
DROP TABLE public.workspace_reaction;

-- DropTable
DROP TABLE public.task_status_reaction;

-- DropEnum
DROP TYPE public.status_category;

-- DropEnum
DROP TYPE public.user_filter_executor;
