use http_request::{
    HttpQuery, HttpQueryReponse, HttpRequest, HttpResponse, HTTP_HISTORY, HTTP_UPDATE_HISTORY,
};
use ic_cdk::export::candid::{CandidType, Deserialize};

use ic_cdk_macros::*;
use std::vec;

mod filesystem;
mod http_request;
mod ic0;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Profile {
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
}
#[derive(CandidType, Deserialize)]
struct StableStorage {
    vfs_root: serde_bytes::ByteBuf,
}

#[export_name = "canister_pre_upgrade"]
fn pre_upgrade() {}

#[export_name = "canister_post_upgrade"]
fn post_upgrade() {}

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

#[query]
fn http_request(_request: HttpRequest) -> HttpResponse {
    HttpResponse {
        status_code: 200,
        headers: Vec::new(),
        body: "a query call to canister".as_bytes().to_vec(),
        upgrade: Some(true),
    }
}

#[update]
fn http_request_update(request: HttpRequest) -> HttpResponse {
    HTTP_UPDATE_HISTORY.with(|history| {
        let mut h = history.borrow_mut();
        h.push(request.clone());
    });
    HttpResponse {
        status_code: 200,
        headers: Vec::new(),
        body: "update call".as_bytes().to_vec(),
        upgrade: Some(false),
    }
}
