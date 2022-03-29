use http_request::{HttpResponse, HttpRequest, HttpQuery, HttpQueryReponse, HTTP_HISTORY, HttpQueryHeaderField};
use ic_cdk::{
    call,
    export::{
        candid::{CandidType, Deserialize},
        Principal,
    },
    storage, trap,
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
mod http_request;
mod ic0;

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

#[query]
fn http_query(request: HttpQuery) -> HttpQueryReponse {
    HttpQueryReponse {
        status: 200,
        body: request.uri.as_bytes().to_vec(),
        headers: Vec::new(),
        upgrade: true,
    }
}

#[update]
fn http_update(request: HttpQuery) -> HttpQueryReponse {
    HTTP_HISTORY.with(|history| {
        let mut h = history.borrow_mut();
        h.push(request.clone());
    });
    http_request::http_request(request)
}