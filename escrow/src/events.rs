use soroban_sdk::{symbol_short, Address, Env};

/// Fires exactly once per contract deployment, at the end of a successful
/// `initialize` call. Payload is just the admin address. Consumed by the
/// New Deployments panel.
pub fn emit_initialized(env: &Env, admin: Address) {
    env.events().publish((symbol_short!("ESC_INI"),), admin);
}

/// Fires once per successful `create_escrow` call, after the new `Escrow`
/// is durably written and `EscrowCount` bumped. `escrow_id` is the newly
/// assigned ID; `amount` is the amount the depositor agreed to fund (not
/// yet transferred -- that happens on `fund_escrow`). Feeds the Top
/// Contracts ranking and the escrow activity feed.
pub fn emit_escrow_created(env: &Env, escrow_id: u64, depositor: Address, beneficiary: Address, amount: i128) {
    env.events().publish(
        (symbol_short!("ESC_CRT"),),
        (escrow_id, depositor, beneficiary, amount),
    );
}

/// Fires once per successful `fund_escrow` call, after the token transfer
/// from `depositor` to this contract has completed and the escrow's state
/// has moved from `Created` to `Funded`.
pub fn emit_escrow_funded(env: &Env, escrow_id: u64, depositor: Address, amount: i128) {
    env.events().publish(
        (symbol_short!("ESC_FND"),),
        (escrow_id, depositor, amount),
    );
}

/// Fires once per successful `release_funds` call (depositor-initiated,
/// only from the `Funded` state), after funds have been transferred to
/// `beneficiary` and the escrow's state moved to `Released`.
pub fn emit_funds_released(env: &Env, escrow_id: u64, beneficiary: Address, amount: i128) {
    env.events().publish(
        (symbol_short!("ESC_REL"),),
        (escrow_id, beneficiary, amount),
    );
}

/// Fires once per successful `refund` call (depositor-initiated, only after
/// the deadline has passed on a `Funded` escrow), after funds have been
/// transferred back to `depositor` and the escrow's state moved to `Refunded`.
pub fn emit_refunded(env: &Env, escrow_id: u64, depositor: Address, amount: i128) {
    env.events().publish(
        (symbol_short!("ESC_RFD"),),
        (escrow_id, depositor, amount),
    );
}

/// Fires once per successful `raise_dispute` call (depositor- or
/// beneficiary-initiated, only from the `Funded` state), after the escrow's
/// state moves to `Disputed`. `raised_by` is whichever of the two parties
/// called it.
pub fn emit_dispute_raised(env: &Env, escrow_id: u64, raised_by: Address) {
    env.events().publish(
        (symbol_short!("ESC_DIS"),),
        (escrow_id, raised_by),
    );
}

/// Fires once per successful `resolve_dispute` call (admin-only, only from
/// the `Disputed` state), after funds have been transferred to whichever
/// party won (`winner`) and the escrow's state moved to `Released` or
/// `Refunded` accordingly.
pub fn emit_dispute_resolved(env: &Env, escrow_id: u64, winner: Address, amount: i128) {
    env.events().publish(
        (symbol_short!("ESC_RSV"),),
        (escrow_id, winner, amount),
    );
}

/// Fires once per successful `cancel_escrow` call (depositor- or
/// admin-initiated, only from the `Created` state, i.e. before any funds
/// were deposited), after the escrow's state moves to `Cancelled`.
pub fn emit_cancelled(env: &Env, escrow_id: u64, cancelled_by: Address) {
    env.events().publish(
        (symbol_short!("ESC_CAN"),),
        (escrow_id, cancelled_by),
    );
}
