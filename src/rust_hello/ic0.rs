use candid::Principal;
use ic_cdk::{api::{data_certificate, canister_balance128, canister_balance, time, stable::{stable_size, stable_grow, stable_write, stable_read}}, caller, id, call, trap};
use ic_cdk_macros::{query, update};

#[query]
fn balance() -> candid::Nat {
    candid::Nat::from(canister_balance())
}

#[query]
fn balance128() -> candid::Nat {
    candid::Nat::from(canister_balance128())
}

#[query]
fn m_data_certificate() -> String {
    format!("{:?}", data_certificate())
}

#[query]
fn m_time() -> candid::Nat {
    candid::Nat::from(time())
}

#[query]
fn m_caller() -> String {
    format!("{}", caller())
}

#[query]
fn m_id() -> String {
    format!("{}", id())
}

#[update]
fn m_stable_size() -> candid::Nat {
    candid::Nat::from(stable_size())
}

#[update]
fn m_stable_grow() {
    stable_grow(10000).unwrap_or(0);
}

#[update]
fn m_stable_write() {
    let p: Vec<u8> = vec![2, 2, 3];
    stable_write(21, &p)
}

#[query]
fn m_stable_read() -> Vec<u8> {
    let mut p = [0, 0, 0].repeat(2);
    stable_read(21, &mut p);
    p
}

#[update]
async fn raw_rand() -> Vec<u8> {
    match call(Principal::management_canister(), "raw_rand", ()).await {
        Ok((res,)) => res,
        Err((_, err)) => trap(&format!("failed to get seed: {}", err)),
    }
}