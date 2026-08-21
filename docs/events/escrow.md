# `escrow` — Event Reference

`escrow/src/lib.rs` (via `escrow/src/events.rs`) emits a dedicated event for
every lifecycle transition.

| Event fn | Topic | Trigger | Consumer |
|---|---|---|---|
| `emit_initialized` | `(ESC_INI,)` | First successful `initialize` call | New Deployments panel |
| `emit_escrow_created` | `(ESC_CRT,)` | Every successful `create_escrow` call | Top Contracts ranking / escrow activity feed |
| `emit_escrow_funded` | `(ESC_FND,)` | Every successful `fund_escrow` call | Escrow activity feed |
| `emit_funds_released` | `(ESC_REL,)` | Every successful `release_funds` call | Escrow activity feed |
| `emit_refunded` | `(ESC_RFD,)` | Every successful `refund` call | Escrow activity feed |
| `emit_dispute_raised` | `(ESC_DIS,)` | Every successful `raise_dispute` call | Escrow activity feed / dispute insights |
| `emit_dispute_resolved` | `(ESC_RSV,)` | Every successful `resolve_dispute` call | Escrow activity feed / dispute insights |
| `emit_cancelled` | `(ESC_CAN,)` | Every successful `cancel_escrow` call | Escrow activity feed |
| `emit_paused` (new) | `(ESC_PSD,)` | Every successful `pause` call (admin-only) | Status panel |
| `emit_unpaused` (new) | `(ESC_UNP,)` | Every successful `unpause` call (admin-only) | Status panel |

Notes:
- `pause`/`unpause` are contract-wide, not per-escrow: they don't change any
  individual `Escrow`'s state, they only block `create_escrow`/`fund_escrow`.
- All events use single-symbol topics (no additional filter topic beyond the
  event name), except none currently key by escrow ID in the topic itself —
  `escrow_id` is always in the payload, not the topic.

See `docs/EVENTS.md` for the cross-crate summary.
