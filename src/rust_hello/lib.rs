use ic_cdk::{
    api::{
        canister_balance, canister_balance128, data_certificate, stable::{stable_size, stable64_grow}, time, call::RejectionCode,
    },
    caller,
    export::{
        candid::{CandidType, Deserialize},
        Principal,
    }, id, call, storage, trap,
};

use ic_cdk_macros::*;
use test_storage::Address;
use std::{cell::RefCell, vec};
use std::collections::BTreeMap;
use std::path::Path;

mod test_storage;

use crate::test_storage::{AddressBook};

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

#[update]
fn m_stable_size() -> candid::Nat {
    candid::Nat::from(stable_size())
}

#[query]
fn path_test() -> String {
    let string = String::from("\nfoo.txt/hahaha/diosjdsij/");
    let p = Path::new(&string);
    
    match p.to_string_lossy() {
        d => {
            d.to_string()
        },
    }
}

#[update]
fn add_address(address: Address) {
    storage::get_mut::<AddressBook>().insert(address.clone());
}

#[query]
fn get_address(id: u32) -> Option<Address> {
    Some(storage::get::<AddressBook>().find(id)?.clone())
}

#[query]
fn all_address() -> Vec<Address> {
    storage::get::<AddressBook>()
        .iter()
        .map(|e| e.to_owned())
        .collect()
}

#[derive(CandidType, Deserialize)]
struct HeaderField(String, String);

#[derive(CandidType, Deserialize)]
struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<HeaderField>,
    #[serde(with = "serde_bytes")]
    body: Vec<u8>
}

#[derive(CandidType, Deserialize)]
struct HttpResponse {
    status_code: u16,
    headers: Vec<HeaderField>,
    #[serde(with = "serde_bytes")]
    body: Vec<u8>,
}

fn get_path(url: &str) -> Option<&str> {
    url.split("?").next()
}

#[query]
fn http_request(request: HttpRequest) -> HttpResponse {
    let path = get_path(request.url.as_str()).unwrap_or("/");

    if path == "/haha" {
        return HttpResponse { 
            status_code: 200,
            headers: Vec::new(),
            body: path.as_bytes().to_vec(), 
        }
    }

    let h: HeaderField =  HeaderField (String::from("Location"), String::from("http://172.18.169.239:8453/haha?canisterId=r7inp-6aaaa-aaaaa-aaabq-cai"));
    HttpResponse { 
        status_code: 301,
        headers: vec![h],
        body: path.as_bytes().to_vec(), 
    }
}

#[update]
async fn raw_rand() -> Vec<u8> {
    match call(Principal::management_canister(), "raw_rand", ()).await {
        Ok((res, )) => res,
        Err((_, err)) => trap(&format!("failed to get seed: {}", err)),
    }
}

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
