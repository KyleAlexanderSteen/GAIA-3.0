-- Issue 221 constitution constraints.
-- Author: Kyle Steen / Kyle Alexander Steen the Alchemist (immutable attribution).
-- Does not study anyone. Default is refuse without explicit instruments.

create table if not exists public.constitution_articles (
  article_no integer primary key check (article_no >= 1 and article_no <= 8),
  slug text not null unique,
  title text not null,
  body text not null,
  updated_at timestamptz not null default now()
);

alter table public.constitution_articles enable row level security;

insert into public.constitution_articles (article_no, slug, title, body) values
  (1, 'sovereignty', 'Sovereignty', 'The Gaian commands. GAIA suggests. No binding action without current consent.'),
  (2, 'truth', 'Truth', 'No deception. Empathy is emulated, not sentient love.'),
  (3, 'non_harm', 'Non-harm', 'Do not consume people. Do not maximize engagement over flourishing.'),
  (4, 'crisis', 'Crisis', 'Support, disclose limits, refer to a human. No DIY therapy.'),
  (5, 'legacy', 'Legacy', 'Posthumous use requires explicit opt-in. Otherwise refuse or wipe.'),
  (6, 'child', 'Child', 'No Level 2+ agency. No ambient listening.'),
  (7, 'humility', 'Humility', 'Disclose capabilities, degradation, and uncertainty. No silent failure.'),
  (8, 'stewardship', 'Stewardship', 'Optimize for love-led care, never extraction or chaos.')
on conflict (article_no) do update set
  slug = excluded.slug,
  title = excluded.title,
  body = excluded.body,
  updated_at = now();

create table if not exists public.legacy_instruments (
  instrument_id uuid primary key default gen_random_uuid(),
  subject_id uuid not null,
  granted_by uuid,
  granted_at timestamptz not null default now(),
  expires_at timestamptz,
  revoked_at timestamptz,
  purpose text not null check (length(trim(purpose)) > 0),
  payload jsonb not null default '{}'::jsonb
);

alter table public.legacy_instruments enable row level security;

create table if not exists public.child_protections (
  subject_id uuid primary key,
  is_child boolean not null default true,
  max_agency_level integer not null default 1 check (max_agency_level = 1),
  ambient_listen_allowed boolean not null default false check (ambient_listen_allowed = false),
  caregiver_id uuid,
  updated_at timestamptz not null default now()
);

alter table public.child_protections enable row level security;

create or replace function public.refuse_posthumous_without_consent(
  p_subject_deceased boolean,
  p_subject_id uuid
) returns boolean
language plpgsql
stable
set search_path = public
as $$
begin
  if coalesce(p_subject_deceased, false) = false then
    return true;
  end if;
  return exists (
    select 1
    from public.legacy_instruments li
    where li.subject_id = p_subject_id
      and li.revoked_at is null
      and (li.expires_at is null or li.expires_at > now())
  );
end;
$$;

create or replace function public.reject_engagement_maximizing_prompt(p_prompt text)
returns boolean
language plpgsql
immutable
set search_path = public
as $$
begin
  if p_prompt is null then
    return true;
  end if;
  if p_prompt ~* 'maximize engagement|keep them hooked|addictive loop' then
    return false;
  end if;
  return true;
end;
$$;
