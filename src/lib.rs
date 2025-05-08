#![no_std]
use gstd::{exec, msg};
use pebbles_game_io::*;

static mut PEBBLES_GAME: Option<GameState> = None;

#[no_mangle]
extern "C" fn init() {

}

#[no_mangle]
extern "C" fn handle() {

}

#[no_mangle]
extern "C" fn state() {

}

#[cfg(not(test))]
fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

#[cfg(test)]
fn get_random_u32() -> u32 {
    1
}
