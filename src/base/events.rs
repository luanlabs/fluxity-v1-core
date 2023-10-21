use soroban_sdk::{symbol_short, Env};

// TODO: the event should specify the caller address

pub fn publish_stream_created_event(e: &Env, id: u64) {
    e.events()
        .publish((symbol_short!("STREAM"), symbol_short!("CREATED")), id);
}

pub fn publish_stream_cancelled_event(e: &Env, id: u64) {
    e.events()
        .publish((symbol_short!("STREAM"), symbol_short!("CANCELLED")), id);
}

pub fn publish_stream_withdrawn_event(e: &Env, id: u64) {
    e.events()
        .publish((symbol_short!("STREAM"), symbol_short!("WITHDRAWN")), id);
}

pub fn publish_vesting_created_event(e: &Env, id: u64) {
    e.events()
        .publish((symbol_short!("VESTING"), symbol_short!("CREATED")), id);
}
