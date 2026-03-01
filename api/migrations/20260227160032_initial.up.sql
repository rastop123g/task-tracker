-- Add up migration script here
-- CreateEnum
CREATE TYPE status_category AS ENUM ('task_difinition', 'work_waiting', 'work_in_progress', 'blocked', 'test_waiting', 'test_in_progress', 'done', 'canceled');

-- CreateEnum
CREATE TYPE user_filter_executor AS ENUM ('all', 'me');

-- CreateTable
CREATE TABLE app_user (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    avatar TEXT,
    email TEXT NOT NULL,
    confirmed BOOLEAN NOT NULL DEFAULT false,
    password TEXT NOT NULL,
    created_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ(6),

    CONSTRAINT app_user_pkey PRIMARY KEY (id)
);

-- CreateTable
CREATE TABLE app_workspace (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    avatar TEXT,
    admin_id UUID NOT NULL,
    created_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ(6),

    CONSTRAINT app_workspace_pkey PRIMARY KEY (id)
);

-- CreateTable
CREATE TABLE user_workspace_filter (
    user_id UUID NOT NULL,
    workspace_id UUID NOT NULL,
    statuses JSONB NOT NULL DEFAULT '[]',
    users user_filter_executor NOT NULL DEFAULT 'all'
);

-- CreateTable
CREATE TABLE workspace_member (
    user_id UUID NOT NULL,
    workspace_id UUID NOT NULL,
    created_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ(6)
);

-- CreateTable
CREATE TABLE workspace_invitation (
    user_id UUID NOT NULL,
    workspace_id UUID NOT NULL,
    created_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ(6)
);

-- CreateTable
CREATE TABLE task_status (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    category status_category NOT NULL,
    workspace_id UUID NOT NULL,
    created_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ(6),

    CONSTRAINT task_status_pkey PRIMARY KEY (id)
);

-- CreateTable
CREATE TABLE task (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    key SERIAL NOT NULL,
    name TEXT,
    description TEXT NOT NULL,
    workspace_id UUID NOT NULL,
    author_id UUID NOT NULL,
    executor_id UUID,
    status_id UUID NOT NULL,
    created_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT task_pkey PRIMARY KEY (id)
);

-- CreateTable
CREATE TABLE message (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    task_id UUID NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    author_id UUID NOT NULL,

    CONSTRAINT message_pkey PRIMARY KEY (id)
);

-- CreateTable
CREATE TABLE read_mark (
    task_id UUID NOT NULL,
    user_id UUID NOT NULL,
    at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    message_id UUID NOT NULL
);

-- CreateTable
CREATE TABLE message_file (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    message_id UUID NOT NULL,
    storrage_key TEXT NOT NULL,
    name TEXT NOT NULL,
    size INTEGER NOT NULL,
    uploader_id UUID NOT NULL,
    created_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT message_file_pkey PRIMARY KEY (id)
);

-- CreateTable
CREATE TABLE avatar (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    storrage_key TEXT NOT NULL,
    storrage_preview TEXT NOT NULL,
    created_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT avatar_pkey PRIMARY KEY (id)
);

-- CreateTable
CREATE TABLE change_status_event (
    id SERIAL NOT NULL,
    task_id UUID NOT NULL,
    time TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    who_id UUID NOT NULL,
    from_id UUID NOT NULL,
    to_id UUID NOT NULL,

    CONSTRAINT change_status_event_pkey PRIMARY KEY (id)
);

-- CreateTable
CREATE TABLE status_interval (
    id SERIAL NOT NULL,
    start_time TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_time TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    status_id UUID NOT NULL,
    user_id UUID NOT NULL,
    task_id UUID NOT NULL,

    CONSTRAINT status_interval_pkey PRIMARY KEY (id)
);

-- CreateTable
CREATE TABLE statuses_preset (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    user_id UUID NOT NULL,

    CONSTRAINT statuses_preset_pkey PRIMARY KEY (id)
);

-- CreateTable
CREATE TABLE status_preset (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    preset_id UUID NOT NULL,
    category status_category NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ(6),

    CONSTRAINT status_preset_pkey PRIMARY KEY (id)
);

-- CreateTable
CREATE TABLE task_preset (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    workspace_id UUID NOT NULL,
    name TEXT NOT NULL,
    executor_id UUID,
    status_id UUID NOT NULL,
    created_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ(6),

    CONSTRAINT task_preset_pkey PRIMARY KEY (id)
);

-- CreateTable
CREATE TABLE workspace_reaction (
    user_id UUID NOT NULL,
    workspace_id UUID NOT NULL,
    at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- CreateTable
CREATE TABLE task_status_reaction (
    user_id UUID NOT NULL,
    task_id UUID NOT NULL,
    at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- CreateIndex
CREATE UNIQUE INDEX user_workspace_filter_user_id_workspace_id_key ON user_workspace_filter(user_id, workspace_id);

-- CreateIndex
CREATE UNIQUE INDEX workspace_member_user_id_workspace_id_key ON workspace_member(user_id, workspace_id);

-- CreateIndex
CREATE UNIQUE INDEX workspace_invitation_user_id_workspace_id_key ON workspace_invitation(user_id, workspace_id);

-- CreateIndex
CREATE UNIQUE INDEX read_mark_task_id_user_id_key ON read_mark(task_id, user_id);

-- CreateIndex
CREATE UNIQUE INDEX workspace_reaction_user_id_workspace_id_key ON workspace_reaction(user_id, workspace_id);

-- CreateIndex
CREATE UNIQUE INDEX task_status_reaction_user_id_task_id_key ON task_status_reaction(user_id, task_id);

-- AddForeignKey
ALTER TABLE app_workspace ADD CONSTRAINT app_workspace_admin_id_fkey FOREIGN KEY (admin_id) REFERENCES app_user(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE user_workspace_filter ADD CONSTRAINT user_workspace_filter_user_id_fkey FOREIGN KEY (user_id) REFERENCES app_user(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE user_workspace_filter ADD CONSTRAINT user_workspace_filter_workspace_id_fkey FOREIGN KEY (workspace_id) REFERENCES app_workspace(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE workspace_member ADD CONSTRAINT workspace_member_user_id_fkey FOREIGN KEY (user_id) REFERENCES app_user(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE workspace_member ADD CONSTRAINT workspace_member_workspace_id_fkey FOREIGN KEY (workspace_id) REFERENCES app_workspace(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE workspace_invitation ADD CONSTRAINT workspace_invitation_user_id_fkey FOREIGN KEY (user_id) REFERENCES app_user(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE workspace_invitation ADD CONSTRAINT workspace_invitation_workspace_id_fkey FOREIGN KEY (workspace_id) REFERENCES app_workspace(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE task_status ADD CONSTRAINT task_status_workspace_id_fkey FOREIGN KEY (workspace_id) REFERENCES app_workspace(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE task ADD CONSTRAINT task_workspace_id_fkey FOREIGN KEY (workspace_id) REFERENCES app_workspace(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE task ADD CONSTRAINT task_author_id_fkey FOREIGN KEY (author_id) REFERENCES app_user(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE task ADD CONSTRAINT task_executor_id_fkey FOREIGN KEY (executor_id) REFERENCES app_user(id) ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE task ADD CONSTRAINT task_status_id_fkey FOREIGN KEY (status_id) REFERENCES task_status(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE message ADD CONSTRAINT message_task_id_fkey FOREIGN KEY (task_id) REFERENCES task(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE message ADD CONSTRAINT message_author_id_fkey FOREIGN KEY (author_id) REFERENCES app_user(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE read_mark ADD CONSTRAINT read_mark_task_id_fkey FOREIGN KEY (task_id) REFERENCES task(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE read_mark ADD CONSTRAINT read_mark_user_id_fkey FOREIGN KEY (user_id) REFERENCES app_user(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE read_mark ADD CONSTRAINT read_mark_message_id_fkey FOREIGN KEY (message_id) REFERENCES message(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE message_file ADD CONSTRAINT message_file_message_id_fkey FOREIGN KEY (message_id) REFERENCES message(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE message_file ADD CONSTRAINT message_file_uploader_id_fkey FOREIGN KEY (uploader_id) REFERENCES app_user(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE change_status_event ADD CONSTRAINT change_status_event_task_id_fkey FOREIGN KEY (task_id) REFERENCES task(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE change_status_event ADD CONSTRAINT change_status_event_who_id_fkey FOREIGN KEY (who_id) REFERENCES app_user(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE change_status_event ADD CONSTRAINT change_status_event_from_id_fkey FOREIGN KEY (from_id) REFERENCES task_status(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE change_status_event ADD CONSTRAINT change_status_event_to_id_fkey FOREIGN KEY (to_id) REFERENCES task_status(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE status_interval ADD CONSTRAINT status_interval_status_id_fkey FOREIGN KEY (status_id) REFERENCES task_status(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE status_interval ADD CONSTRAINT status_interval_user_id_fkey FOREIGN KEY (user_id) REFERENCES app_user(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE status_interval ADD CONSTRAINT status_interval_task_id_fkey FOREIGN KEY (task_id) REFERENCES task(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE statuses_preset ADD CONSTRAINT statuses_preset_user_id_fkey FOREIGN KEY (user_id) REFERENCES app_user(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE status_preset ADD CONSTRAINT status_preset_preset_id_fkey FOREIGN KEY (preset_id) REFERENCES statuses_preset(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE task_preset ADD CONSTRAINT task_preset_workspace_id_fkey FOREIGN KEY (workspace_id) REFERENCES app_workspace(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE task_preset ADD CONSTRAINT task_preset_executor_id_fkey FOREIGN KEY (executor_id) REFERENCES app_user(id) ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE task_preset ADD CONSTRAINT task_preset_status_id_fkey FOREIGN KEY (status_id) REFERENCES task_status(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE workspace_reaction ADD CONSTRAINT workspace_reaction_user_id_fkey FOREIGN KEY (user_id) REFERENCES app_user(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE workspace_reaction ADD CONSTRAINT workspace_reaction_workspace_id_fkey FOREIGN KEY (workspace_id) REFERENCES app_workspace(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE task_status_reaction ADD CONSTRAINT task_status_reaction_user_id_fkey FOREIGN KEY (user_id) REFERENCES app_user(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE task_status_reaction ADD CONSTRAINT task_status_reaction_task_id_fkey FOREIGN KEY (task_id) REFERENCES task(id) ON DELETE RESTRICT ON UPDATE CASCADE;

