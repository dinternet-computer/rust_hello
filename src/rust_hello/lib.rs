use ic_cdk::{
    api::{
        canister_balance, canister_balance128, data_certificate, stable::{stable_size, stable64_grow}, time, call::RejectionCode,
    },
    caller,
    export::{
        candid::{CandidType, Deserialize},
        Principal,
    }, id, call,
};

use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::BTreeMap;

type IdStore = BTreeMap<String, Principal>;
type ProfileStore = BTreeMap<Principal, Profile>;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Profile {
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
}

thread_local! {
    static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
    static ID_STORE: RefCell<IdStore> = RefCell::default();
}

#[query(name = "getSelf")]
fn get_self() -> Profile {
    let id = ic_cdk::api::caller();

    PROFILE_STORE.with(|profile_store| {
        profile_store
            .borrow()
            .get(&id)
            .cloned()
            .unwrap_or_else(|| Profile::default())
    })
}

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

#[query]
fn m_stable_size() -> candid::Nat {
    candid::Nat::from(stable_size())
}

// #[update]
// async fn raw_rand() -> Option<(Vec<u8>,)> {
//     let v: Result<(Vec<u8>,), _> = call(Principal::management_canister(), "raw_rand", ()).await?;
    
//     match v {
//         Ok(u) => u,
//         Err(e) => panic!(e)
//     }
// }

#[export_name = "canister_heartbeat"]
fn tick() {
    
}

#[query]
fn get(name: String) -> Profile {
    ID_STORE.with(|id_store| {
        PROFILE_STORE.with(|profile_store| {
            id_store
                .borrow()
                .get(&name)
                .and_then(|id| profile_store.borrow().get(id).cloned())
                .unwrap_or_else(|| Profile::default())
        })
    })
}

#[update]
fn update(profile: Profile) {
    let principal_id = ic_cdk::api::caller();
    ID_STORE.with(|id_store| {
        id_store
            .borrow_mut()
            .insert(profile.name.clone(), principal_id);
    });
    PROFILE_STORE.with(|profile_store| {
        profile_store.borrow_mut().insert(principal_id, profile);
    });
}
