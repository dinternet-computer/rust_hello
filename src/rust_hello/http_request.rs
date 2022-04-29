use std::{
    cell::RefCell,
    vec,
};

use candid::{CandidType, Deserialize};
use ic_cdk_macros::{query, update};

use git_packetline::encode::*;

#[derive(CandidType, Deserialize, Clone)]
pub struct HeaderField(String, String);

#[derive(CandidType, Deserialize, Clone, Default)]
pub struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<HeaderField>,
    #[serde(with = "serde_bytes")]
    body: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct HttpQueryHeaderField(Vec<u8>, Vec<u8>);

#[derive(CandidType, Deserialize, Clone)]
pub struct HttpQuery {
    pub method: String,
    pub headers: Vec<HttpQueryHeaderField>,
    pub uri: String,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct HttpQueryReponse {
    pub status: u16,
    pub headers: Vec<HttpQueryHeaderField>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
    pub upgrade: bool,
}

#[derive(CandidType, Clone, Default, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
    pub upgrade: Option<bool>,
}

type HttpRequestHistory = Vec<HttpQuery>;

thread_local! {
    pub static HTTP_UPDATE_HISTORY: RefCell<Vec<HttpRequest>> = RefCell::default();
    pub static HTTP_HISTORY: RefCell<HttpRequestHistory> = RefCell::default();
}

#[query]
fn get_http_request_history() -> Vec<HttpQuery> {
    HTTP_HISTORY.with(|s| s.borrow_mut().clone())
}

#[update]
fn clear_get_http_request_history() {
    HTTP_HISTORY.with(|s| s.borrow_mut().clear());
}

#[query]
fn get_http_update_request_history() -> Vec<HttpRequest> {
    HTTP_UPDATE_HISTORY.with(|s|s.borrow_mut().clone())
}

#[update]
fn clear_get_http_update_history() {
    HTTP_UPDATE_HISTORY.with(|s| s.borrow_mut().clear())
}

// it works!
fn get_text_body() -> Vec<u8> {
    let mut out = Vec::new();
    text_to_write(b"# service=git-upload-pack", &mut out).unwrap();
    flush_to_write(&mut out).unwrap();
    text_to_write(b"44b7c48848eadb91c330c2a104189d8dca5a393d HEAD\0multi_ack thin-pack side-band side-band-64k ofs-delta shallow deepen-since deepen-not deepen-relative no-progress include-tag multi_ack_detailed allow-tip-sha1-in-want allow-reachable-sha1-in-want no-done symref=HEAD:refs/heads/main filter object-format=sha1 agent=git/github-g979fda6922df", &mut out).unwrap();
    text_to_write(b"44b7c48848eadb91c330c2a104189d8dca5a393d refs/heads/main", &mut out).unwrap();

    flush_to_write(&mut out).unwrap();
    // let a1 = "001e# service=git-upload-pack\n0000";
    // let a2 = "015444b7c48848eadb91c330c2a104189d8dca5a393d HEAD\0multi_ack thin-pack side-band side-band-64k ofs-delta shallow deepen-since deepen-not deepen-relative no-progress include-tag multi_ack_detailed allow-tip-sha1-in-want allow-reachable-sha1-in-want no-done symref=HEAD:refs/heads/main filter object-format=sha1 agent=git/github-g979fda6922df\n";
    // let a3 = "003d44b7c48848eadb91c330c2a104189d8dca5a393d refs/heads/main\n";
    // let a4 = "0000";
    out
}

pub fn http_request(request: HttpQuery) -> HttpQueryReponse {
    let path = request.uri.as_str();

    if request.method.to_ascii_lowercase() == "get"
        && path == "/main.git/info/refs?service=git-upload-pack"
    {
        return HttpQueryReponse {
            status: 200,
            headers: vec![
                HttpQueryHeaderField(
                    "Cache-Control".as_bytes().to_vec(),
                    "no-cache".as_bytes().to_vec(),
                ),
                HttpQueryHeaderField(
                    "Content-Type".as_bytes().to_vec(),
                    "application/x-git-upload-pack-advertisement"
                        .as_bytes()
                        .to_vec(),
                ),
            ],
            body: get_text_body(),
            upgrade: false,
        };
    }

    if path == "/main.git/info/refs" {
        return HttpQueryReponse {
            status: 200,
            headers: vec![
                HttpQueryHeaderField(
                    "Cache-Control".as_bytes().to_vec(),
                    "no-cache".as_bytes().to_vec(),
                ),
                HttpQueryHeaderField(
                    "Content-Type".as_bytes().to_vec(),
                    "application/x-git-upload-pack-advertisement"
                        .as_bytes()
                        .to_vec(),
                ),
            ],
            body: get_text_body(),
            upgrade: false,
        };
    }

    if request.method.to_ascii_lowercase() == "post" {
        return HttpQueryReponse {
            status: 200,
            headers: vec![
                HttpQueryHeaderField(
                    "Cache-Control".as_bytes().to_vec(),
                    "no-cache".as_bytes().to_vec(),
                ),
                HttpQueryHeaderField(
                    "Content-Type".as_bytes().to_vec(),
                    "application/x-git-upload-pack-result".as_bytes().to_vec(),
                ),
            ],
            body: include_bytes!("./da.bin").to_vec(),
            upgrade: false,
        };
    }
    HttpQueryReponse {
        status: 401,
        headers: Vec::new(),
        body: path.as_bytes().to_vec(),
        upgrade: false,
    }
}
