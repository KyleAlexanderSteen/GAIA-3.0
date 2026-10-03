-- SPDX-FileCopyrightText: Copyright 2026 Kyle Steen
-- Author: Kyle Steen
-- Public identity: Kyle Alexander Steen the Alchemist
-- GitHub: https://github.com/KyleAlexanderSteen
-- SPDX-License-Identifier: Apache-2.0
-- Canon: C77 Love-Led Stewardship Doctrine
-- Assisted-by: Perplexity

-- Registers C77 provenance. Does not weaken existing RLS.
-- Additive only. No drops. No grants to anon.

create table if not exists public.gaia_canon_documents (
  id uuid primary key default gen_random_uuid(),
  canon_id text not null unique,
  title text not null,
  version integer not null default 1,
  status text not null default 'draft',
  author_legal_name text not null default 'Kyle Steen',
  author_public_name text not null default 'Kyle Alexander Steen the Alchemist',
  github_login text not null default 'KyleAlexanderSteen',
  copyright_year integer not null default 2026,
  license text not null default 'CC-BY-4.0',
  path text,
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  constraint gaia_canon_documents_status_chk
    check (status in ('draft', 'proposed', 'active', 'deprecated')),
  constraint gaia_canon_documents_author_chk
    check (author_legal_name = 'Kyle Steen')
);

comment on table public.gaia_canon_documents is
  'C77 canon registry. Author is Kyle Steen. Credit stripping is a canon violation.';

alter table public.gaia_canon_documents enable row level security;

insert into public.gaia_canon_documents (
  canon_id,
  title,
  version,
  status,
  author_legal_name,
  author_public_name,
  github_login,
  copyright_year,
  license,
  path
)
values (
  'C77',
  'Love-Led Stewardship Doctrine',
  1,
  'proposed',
  'Kyle Steen',
  'Kyle Alexander Steen the Alchemist',
  'KyleAlexanderSteen',
  2026,
  'CC-BY-4.0',
  'docs/canon/C77_LOVE_LED_STEWARDSHIP_DOCTRINE.md'
)
on conflict (canon_id) do update
set
  title = excluded.title,
  version = excluded.version,
  status = excluded.status,
  author_legal_name = 'Kyle Steen',
  author_public_name = excluded.author_public_name,
  github_login = excluded.github_login,
  copyright_year = excluded.copyright_year,
  license = excluded.license,
  path = excluded.path,
  updated_at = now();
