use soroban_sdk::{symbol_short, Env};

pub fn publish_lockup_created_event(e: &Env, id: u64) {
    e.events()
        .publish((symbol_short!("LOCKUP"), symbol_short!("CREATED")), id);
}

pub fn publish_lockup_cancelled_event(e: &Env, id: u64) {
    e.events()
        .publish((symbol_short!("LOCKUP"), symbol_short!("CANCELLED")), id);
}

pub fn publish_lockup_withdrawn_event(e: &Env, id: u64) {
    e.events()
        .publish((symbol_short!("LOCKUP"), symbol_short!("WITHDRAWN")), id);
}

pub fn publish_lockup_topup_event(e: &Env, id: u64) {
    e.events()
        .publish((symbol_short!("LOCKUP"), symbol_short!("TOPUP")), id);
}
