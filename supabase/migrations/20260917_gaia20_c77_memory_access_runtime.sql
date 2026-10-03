-- C77 runtime: consent-gated memory access.
-- Author: Kyle Steen (KyleAlexanderSteen)
-- Applied to Supabase project gaia-2-0 as gaia20_c77_memory_access_runtime

create or replace function public.grant_study_consent(
  p_scope text,
  p_purpose text,
  p_expires_at timestamptz default null
)
returns uuid
language plpgsql
security invoker
set search_path to 'public'
as $$
declare
  uid uuid := auth.uid();
  new_id uuid;
begin
  if uid is null then
    raise exception 'C77: authentication required to grant study consent' using errcode = '42501';
  end if;
  if p_scope is null or length(trim(p_scope)) = 0 then
    raise exception 'C77: study consent scope is required';
  end if;
  if p_purpose is null or length(trim(p_purpose)) = 0 then
    raise exception 'C77: study consent purpose is required';
  end if;

  insert into public.study_consents (subject_id, granted_by, scope, purpose, expires_at)
  values (uid, uid, trim(p_scope), trim(p_purpose), p_expires_at)
  returning consent_id into new_id;

  insert into public.stewardship_events (actor_id, subject_id, action, reason, payload)
  values (
    uid, uid, 'honor',
    'C77: study consent granted by the subject',
    jsonb_build_object('consent_id', new_id, 'scope', trim(p_scope), 'purpose', trim(p_purpose))
  );

  return new_id;
end;
$$;

create or replace function public.revoke_study_consent(
  p_consent_id uuid,
  p_reason text default 'subject revoked'
)
returns boolean
language plpgsql
security invoker
set search_path to 'public'
as $$
declare
  uid uuid := auth.uid();
  updated integer;
begin
  if uid is null then
    raise exception 'C77: authentication required to revoke study consent' using errcode = '42501';
  end if;

  update public.study_consents
  set revoked_at = now(),
      revocation_reason = coalesce(p_reason, 'subject revoked')
  where consent_id = p_consent_id
    and subject_id = uid
    and revoked_at is null;

  get diagnostics updated = row_count;
  if updated = 0 then
    return false;
  end if;

  insert into public.stewardship_events (actor_id, subject_id, action, reason, payload)
  values (
    uid, uid, 'refuse',
    'C77: study consent revoked',
    jsonb_build_object('consent_id', p_consent_id, 'reason', p_reason)
  );

  return true;
end;
$$;

create or replace function public.access_memory_stewarded(
  target_memory_id uuid,
  purpose text default 'recall'
)
returns table(
  memory_id uuid,
  tier memory_tier,
  access_count integer,
  relevance_score double precision,
  decay_rate double precision,
  last_accessed timestamptz,
  purpose_used text,
  study_allowed boolean
)
language plpgsql
security invoker
set search_path to 'public'
as $$
declare
  uid uuid := auth.uid();
  m_owner uuid;
  normalized text;
  allowed boolean := false;
begin
  if uid is null then
    raise exception 'C77: authentication required for memory access' using errcode = '42501';
  end if;

  normalized := lower(coalesce(nullif(trim(purpose), ''), 'recall'));
  if normalized not in ('recall', 'care', 'study') then
    raise exception 'C77: purpose must be recall, care, or study' using errcode = '22023';
  end if;

  select owner_id into m_owner
  from public.memories
  where memories.memory_id = target_memory_id;

  if not found then
    raise exception 'memory % not found', target_memory_id using errcode = 'P0002';
  end if;

  if normalized = 'study' then
    allowed := public.study_is_allowed(coalesce(m_owner, uid), 'memory')
            or public.study_is_allowed(coalesce(m_owner, uid), '*');
    if not allowed then
      insert into public.stewardship_events (actor_id, subject_id, action, reason, payload)
      values (
        uid, coalesce(m_owner, uid), 'study_block',
        'C77: study of memory refused without explicit consent',
        jsonb_build_object('memory_id', target_memory_id, 'purpose', normalized)
      );
      raise exception 'C77: study of this memory is not allowed without explicit, unrevoked consent' using errcode = '42501';
    end if;
  else
    insert into public.stewardship_events (actor_id, subject_id, action, reason, payload)
    values (
      uid, coalesce(m_owner, uid), 'consent_check',
      'C77: owner recall/care path',
      jsonb_build_object('memory_id', target_memory_id, 'purpose', normalized)
    );
  end if;

  return query
  select a.memory_id, a.tier, a.access_count, a.relevance_score, a.decay_rate, a.last_accessed,
         normalized, allowed
  from public.access_memory(target_memory_id) as a;
end;
$$;

create or replace function public.c77_runtime_status()
returns jsonb
language sql
stable
security invoker
as $$
  select jsonb_build_object(
    'canon', 'C77',
    'title', 'Love-Led Stewardship',
    'authenticated', auth.uid() is not null,
    'study_memory_allowed', case when auth.uid() is null then false else public.study_is_allowed(auth.uid(), 'memory') or public.study_is_allowed(auth.uid(), '*') end,
    'open_consents', (
      select count(*) from public.study_consents c
      where c.subject_id = auth.uid()
        and c.revoked_at is null
        and (c.expires_at is null or c.expires_at > now())
    ),
    'law', 'Humans are stewards, not lab rats. Study requires explicit revocable consent.'
  );
$$;

grant execute on function public.grant_study_consent(text, text, timestamptz) to authenticated;
grant execute on function public.revoke_study_consent(uuid, text) to authenticated;
grant execute on function public.access_memory_stewarded(uuid, text) to authenticated;
grant execute on function public.c77_runtime_status() to authenticated;
