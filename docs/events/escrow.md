# `escrow` — Event & Call Schema Reference

`escrow/src/lib.rs` (via `escrow/src/events.rs`) emits a dedicated event for
every state-changing call, keyed off the `Escrow` struct's lifecycle
(`Created` -> `Funded` -> `Released`/`Refunded`, or `Funded` -> `Disputed` ->
`Released`/`Refunded`, or `Created` -> `Cancelled`). This is what feeds the
**Top Contracts ranking**: each event below corresponds 1:1 with a
state-changing contract call, so counting/weighting events per contract ID
is equivalent to counting/weighting calls.

| Event fn | Topic | Trigger (contract call) | Payload |
|---|---|---|---|
| `emit_initialized` | `(ESC_INI,)` | `initialize` (first call only) | `admin: Address` |
| `emit_escrow_created` | `(ESC_CRT,)` | `create_escrow` | `(escrow_id, depositor, beneficiary, amount)` |
| `emit_escrow_funded` | `(ESC_FND,)` | `fund_escrow` | `(escrow_id, depositor, amount)` |
| `emit_funds_released` | `(ESC_REL,)` | `release_funds` | `(escrow_id, beneficiary, amount)` |
| `emit_refunded` | `(ESC_RFD,)` | `refund` | `(escrow_id, depositor, amount)` |
| `emit_dispute_raised` | `(ESC_DIS,)` | `raise_dispute` | `(escrow_id, raised_by)` |
| `emit_dispute_resolved` | `(ESC_RSV,)` | `resolve_dispute` | `(escrow_id, winner, amount)` |
| `emit_cancelled` | `(ESC_CAN,)` | `cancel_escrow` | `(escrow_id, cancelled_by)` |

`pause`/`unpause` currently emit no events — tracked separately (see the
paired "add lifecycle events" issue for this crate).

## The `Escrow` struct (the underlying state each event reflects)

Stored in persistent storage under `DataKey::Escrow(id)`, `id` being a
monotonically increasing counter (`DataKey::EscrowCount`, instance storage).
Fields: `id`, `depositor`, `beneficiary`, `token`, `amount`, `state`
(`EscrowState` enum), `deadline`, `created_at`. `get_escrow(id)` always
returns the current, authoritative value — events are a change feed, not a
replacement for reading storage.

## Why this feeds Top Contracts specifically

The ranking needs, per contract, a count/volume of calls over a time window.
Every `escrow` call that changes state publishes exactly one event carrying
`escrow_id` and (where relevant) `amount`, so an indexer can attribute
activity to this contract's address purely from its event stream without
having to poll `get_escrow_count()`/scan storage.

See `docs/EVENTS.md` for the cross-crate summary.
