# RP-NO-LEASE-WITHOUT-FENCING-V1

**Status:** Active. Promoted 2026-06-15 with D5 v1 ship.
**Originating finding:** D5 ships an admission-time lease only; v1 has no write-side fencing.
**Source review:** `REVIEW_quorum-sense_2026-06-15.md` HELMS F5; RR Round-2 D5 acceptance with documented stale-writer gap.
**Follow-up ticket:** D5.1 (RR-owned, opened when --max-instances > 1 is needed in production).

## What this standard says (verbatim)

> Admission-time lease (D5) serializes new mutating requests across healthy instances. It does not prevent stale-writer writes after TTL steal. Write-side fencing is D5.1.

## What this means in practice

D5's `SessionOwnershipLayer` rejects a second instance's mutating request with 409 ownership_held while the first instance holds the lease. That is its complete safety guarantee.

D5 does NOT prevent the classic stuck-process scenario:
1. Instance A acquires lease at t=0.
2. A pauses (GC, network, scheduling) longer than TTL.
3. Instance B's request arrives at t=TTL+1; B steals the lease (Acquired).
4. B completes its writes.
5. A wakes at t=TTL+10; A's already-in-flight handler completes its writes through `DocumentStore`/`EventLog` — bypassing the lease entirely because the storage layer doesn't check holder identity on writes.

In step 5, A's write is "stale" — A no longer owns the lease, but the storage backend accepts the write anyway.

## When this standard matters

Any of these claims must be rejected:
- "We have D5 now, we can run multi-writer."
- "D5 prevents data corruption under concurrent writes."
- "Lifting `--max-instances=1` is safe because D5 shipped."

Correct claim: "D5 prevents concurrent admission of mutating requests. It does not prevent stale-writer writes after a steal. `--max-instances > 1` is only safe when D5.1 (write-side fencing) ALSO ships."

## Cross-references

- Marquee App Contract rule 6 (`BOUNDARY_REGISTRY.md`).
- `marquee-apps/quorum-sense/deploy/cloud-run-provision.sh` `--max-instances=1` pin comment cites D5 + QF-CR-03 + QF-CR-08 as the three release gates. When the pin is lifted, the comment must reference D5.1 as the safety-completion ticket.
- D5 spec section 12 (deferred to D5.1).

## How to check (drift)

PR review rejects any change that:
- Lifts `--max-instances=1` in any marquee-app deploy without D5.1 also having shipped.
- Documents D5 alone as multi-writer-safe.
- Removes this standard.
