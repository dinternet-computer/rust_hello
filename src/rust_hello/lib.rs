use ic_cdk::{
    api::{
        canister_balance, canister_balance128, data_certificate,
        stable::{stable_grow, stable_read, stable_size, stable_write},
        time,
    },
    call, caller,
    export::{
        candid::{CandidType, Deserialize},
        Principal,
    },
    id, storage, trap,
};

use ic_cdk_macros::*;
use std::{borrow::Borrow, collections::BTreeMap};
use std::{
    borrow::{BorrowMut, Cow},
    path::Path,
};
use std::{cell::RefCell, vec};
use test_storage::Address;

use vfs::{MemoryFS, VfsError, VfsPath};


use crate::test_storage::AddressBook;

mod test_storage;
mod filesystem;

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
    static VFS_ROOT: VfsPath = VFS::init().0.into();
}

struct VFS(MemoryFS);

impl VFS {
    fn init() -> Self {
        VFS(MemoryFS::new())
    }
}

#[derive(CandidType, Deserialize)]
struct StableStorage {
    vfs_root: serde_bytes::ByteBuf,
}

#[export_name = "canister_pre_upgrade"]
fn pre_upgrade() {}

#[export_name = "canister_post_upgrade"]
fn post_upgrade() {}

#[update]
fn get_all_file() -> Vec<String> {
    VFS_ROOT.with(|p| {
        p.join("a.txt")
            .unwrap()
            .create_file()
            .unwrap()
            .write_all(b"hahahahdsuahdsau")
            .unwrap();

        p.join("b.txt")
            .unwrap()
            .create_file()
            .unwrap()
            .write_all(b"hahahahdasuhdsaudhsa")
            .unwrap();

        p.read_dir()
            .unwrap()
            .map(|v| v.as_str().to_string())
            .collect::<Vec<String>>()
    })
}

#[update]
fn create_file(filename: String, content: String) -> Vec<String> {
    VFS_ROOT.with(|p| {
        p.join(filename)
            .unwrap()
            .create_file()
            .unwrap()
            .write_all(content.as_bytes())
            .unwrap();

        get_all_file()
    })
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

#[query]
fn path_test() -> String {
    let string = String::from("\nfoo.txt/hahaha/diosjdsij/");
    let p = Path::new(&string);

    match p.to_string_lossy() {
        d => d.to_string(),
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
    body: Vec<u8>,
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
        };
    }

    let h: HeaderField = HeaderField(
        String::from("Location"),
        String::from("http://172.18.169.239:8453/haha?canisterId=r7inp-6aaaa-aaaaa-aaabq-cai"),
    );
    HttpResponse {
        status_code: 301,
        headers: vec![h],
        body: path.as_bytes().to_vec(),
    }
}

#[update]
async fn raw_rand() -> Vec<u8> {
    match call(Principal::management_canister(), "raw_rand", ()).await {
        Ok((res,)) => res,
        Err((_, err)) => trap(&format!("failed to get seed: {}", err)),
    }
}

#[export_name = "canister_heartbeat"]
fn tick() {}

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
