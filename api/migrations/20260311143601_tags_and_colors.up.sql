-- Add up migration script here
-- CreateEnum
CREATE TYPE color AS ENUM ('red', 'green', 'blue', 'yellow', 'pink', 'purple', 'orange', 'brown', 'gray');

-- AlterTable
ALTER TABLE status_preset ADD COLUMN     color color NOT NULL DEFAULT 'gray';

-- AlterTable
ALTER TABLE task_status ADD COLUMN     color color NOT NULL DEFAULT 'gray';

-- CreateTable
CREATE TABLE workspace_tag (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    workspace_id UUID NOT NULL,
    name TEXT NOT NULL,
    color color NOT NULL DEFAULT 'gray',
    created_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ(6),

    CONSTRAINT workspace_tag_pkey PRIMARY KEY (id)
);

-- CreateTable
CREATE TABLE task_tag (
    task_id UUID NOT NULL,
    tag_id UUID NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX task_tag_task_id_tag_id_key ON task_tag(task_id, tag_id);

-- AddForeignKey
ALTER TABLE workspace_tag ADD CONSTRAINT workspace_tag_workspace_id_fkey FOREIGN KEY (workspace_id) REFERENCES app_workspace(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE task_tag ADD CONSTRAINT task_tag_task_id_fkey FOREIGN KEY (task_id) REFERENCES task(id) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE task_tag ADD CONSTRAINT task_tag_tag_id_fkey FOREIGN KEY (tag_id) REFERENCES workspace_tag(id) ON DELETE RESTRICT ON UPDATE CASCADE;

