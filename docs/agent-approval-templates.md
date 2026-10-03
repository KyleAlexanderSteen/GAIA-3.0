# Agent Approval Templates

> These templates standardize the approval requests an AI assistant
> must use when requesting human authorization for repository mutations
> inside GAIA 2.0. Copy-paste the relevant template exactly.
> Conversational text alone — e.g. "go ahead", "looks good", "yes" —
> is **not** a valid approval for a write operation.

---

## What makes an approval valid

An approval is valid only when:

- The human explicitly approves the **exact** request shown below.
- The `Expected SHA` matches the current branch head at approval time.
- The `Target files` list has not been expanded after the request.
- The approval has not previously been used (single-use).
- The approval has not expired (treat any approval older than the
  current conversation turn as expired).

## What invalidates an existing approval

- The branch HEAD advances (another commit lands).
- The proposed patch is modified after the request is shown.
- The target file set expands beyond what was listed.
- The operation class changes (e.g. from `PUSH_PATCH` to `UPDATE_PR`).
- The human revokes or modifies the approval after giving it.

If any of the above occurs, **discard the approval** and present a new
request.

---

## Template: CREATE_BRANCH

```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
APPROVAL REQUEST — CREATE_BRANCH
┃ Operation:    CREATE_BRANCH
┃ Repository:   KyleAlexanderSteen/GAIA-2.0
┃ New branch:   <branch-name>
┃ From:         <base-branch> @ <base-SHA>
┃ Purpose:      <one-sentence description>
┃ Attempt:      N/A
┃ Expires:      Single-use — valid for this request only
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

---

## Template: PUSH_PATCH (initial)

```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
APPROVAL REQUEST — PUSH_PATCH
┃ Operation:       PUSH_PATCH
┃ Repository:      KyleAlexanderSteen/GAIA-2.0
┃ Branch:          <branch-name>
┃ Expected SHA:    <exact head SHA at time of this request>
┃ Target files:    <explicit list, one per line>
┃ Diff summary:    <concise description of what changes and why>
┃ Risk class:      low | medium | high
┃ Validation:      <command that will run after the patch>
┃ Attempt:         1 of <max>
┃ Expires:         Single-use — valid for this request only
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

---

## Template: PUSH_CORRECTIVE_PATCH (retry)

```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
APPROVAL REQUEST — PUSH_CORRECTIVE_PATCH
┃ Operation:         PUSH_CORRECTIVE_PATCH
┃ Repository:        KyleAlexanderSteen/GAIA-2.0
┃ Branch:            <branch-name>
┃ Expected SHA:      <exact head SHA at time of this request>
┃ Target files:      <explicit list — must not exceed previous approval>
┃ Previous attempt:  <n-1> — fingerprint: <prev-fingerprint>
┃ Progress:          <what changed vs. previous attempt>
┃ Diff summary:      <concise description of what changes and why>
┃ Risk class:        low | medium | high
┃ Validation:        <command that will run after the patch>
┃ Attempt:           <n> of <max>
┃ Expires:           Single-use — valid for this request only
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

---

## Template: UPDATE_PR

```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
APPROVAL REQUEST — UPDATE_PR
┃ Operation:     UPDATE_PR
┃ Repository:    KyleAlexanderSteen/GAIA-2.0
┃ PR number:     #<number>
┃ Branch:        <head-branch> → <base-branch>
┃ Expected SHA:  <exact head SHA at time of this request>
┃ Changes:       <title update | body update | reviewer change | label>
┃ Risk class:    low
┃ Attempt:       N/A
┃ Expires:       Single-use — valid for this request only
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

---

## Template: MERGE

> ⚠️ **MERGE approval is always separate from patch approval.**
> Approving a `PUSH_PATCH` or `PUSH_CORRECTIVE_PATCH` does **not** imply
> approval to merge. The assistant must always present a distinct MERGE
> request and wait for explicit approval before merging.

```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
APPROVAL REQUEST — MERGE
┃ Operation:       MERGE
┃ Repository:      KyleAlexanderSteen/GAIA-2.0
┃ PR number:       #<number>
┃ Branch:          <head-branch> → <base-branch>
┃ Expected SHA:    <exact head SHA at time of this request>
┃ CI status:       All required checks passed ✓
┃ Merge method:    squash | merge | rebase
┃ Risk class:      <low | medium | high>
┃ Note:            Deploy/publish/release require separate approval.
┃ Expires:         Single-use — valid for this request only
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

---

*Part of Phase 3 of the human-gated correction loop.
See [`docs/agent-correction-loop.md`](agent-correction-loop.md),
issue [#986](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/986),
and parent issue [#983](https://github.com/KyleAlexanderSteen/GAIA-2.0/issues/983).*
