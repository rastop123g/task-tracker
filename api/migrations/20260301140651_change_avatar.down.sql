-- Add down migration script here
-- AlterTable
ALTER TABLE public.app_user DROP COLUMN avatar_preview;

-- CreateTable
CREATE TABLE public.avatar (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    storrage_key TEXT NOT NULL,
    storrage_preview TEXT NOT NULL,
    created_at TIMESTAMPTZ(6) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT avatar_pkey PRIMARY KEY (id)
);

