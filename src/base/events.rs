use soroban_sdk::{symbol_short, Env};

pub fn publish_stream_created_event(e: &Env, id: u64) {
    e.events()
        .publish((symbol_short!("STREAM"), symbol_short!("CREATED")), id);
}

pub fn publish_stream_cancelled_event(e: &Env, id: u64) {
    e.events()
        .publish((symbol_short!("STREAM"), symbol_short!("CANCELLED")), id);
}
