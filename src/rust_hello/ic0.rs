use std::{cell::RefCell, convert::TryInto};

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{
    api::{
        call::{call_with_payment128, msg_cycles_accept, msg_cycles_available, CallResult},
        canister_balance128, data_certificate,
        stable::{stable64_read, stable64_write, stable_grow, stable_size},
        time,
    },
    call, caller, id, trap,
};
use ic_cdk_macros::{query, update};

#[query]
fn balance128() -> candid::Nat {
    candid::Nat::from(canister_balance128())
}

#[update]
fn wallet_receive() {
    let amount = msg_cycles_available();

    if amount > 0 {
        msg_cycles_accept(amount);
    }
}

thread_local! {
    static MY_CANISTERS: RefCell<Vec<Principal>> = RefCell::default();
}

#[update]
async fn send_cycles_back() {
    let principal = Principal::from_text("4mj33-paaaa-aaaai-qivlq-cai").unwrap();
    let a: CallResult<()> =
        call_with_payment128(principal, "wallet_receive", (), 1_000_000_000_000).await;

    a.unwrap();
}

#[derive(CandidType, Deserialize)]
struct CreateCanisterResult {
    canister_id: Principal,
}

#[update]
async fn create_canister() -> Principal {
    let (result, ): (CreateCanisterResult,) = call_with_payment128(
        Principal::management_canister(),
        "create_canister",
        (),
        200_000_000_000,
    )
    .await
    .unwrap();

    MY_CANISTERS.with(|p| p.borrow_mut().push(result.canister_id));

    result.canister_id
}

#[derive(CandidType, Deserialize)]
enum InstallMode {
    #[serde(rename = "install")]
    Install,
    #[serde(rename = "reinstall")]
    Reinstall,
    #[serde(rename = "upgrade")]
    Upgrade,
}

#[derive(CandidType, Deserialize)]
struct CanisterInstallConfig {
    mode: InstallMode,
    canister_id: Principal,
    #[serde(with = "serde_bytes")]
    wasm_module: Vec<u8>,
    arg: Vec<u8>,
}

#[update]
async fn install_code_for_it(mode: InstallMode) {
    // 子罐头 ginsq-liaaa-aaaai-qjbrq-cai
    let principal = Principal::from_text("ginsq-liaaa-aaaai-qjbrq-cai").unwrap();

    let install_config = CanisterInstallConfig {
        mode,
        canister_id: principal,
        wasm_module: include_bytes!("./my_child_canister.wasm").to_vec(),
        arg: b" ".to_vec(),
    };

    match call(
        Principal::management_canister(),
        "install_code",
        (install_config,),
    )
    .await
    {
        Ok(x) => x,
        Err((_code, msg)) => trap(msg.as_str()),
    }
}

#[derive(CandidType, Deserialize)]
enum CanisterStatus {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "stopped")]
    Stopped,
}

#[derive(CandidType, Deserialize)]
struct CanisterSettings {
    controllers: Vec<Principal>,
    compute_allocation: candid::Nat,
    memory_allocation: candid::Nat,
    freezing_threshold: candid::Nat,
}

#[derive(CandidType, Deserialize)]
struct CanisterStatusResult {
    status: CanisterStatus,
    settings: CanisterSettings,
    module_hash: Option<Vec<u8>>,
    memory_size: candid::Nat,
    cycles: candid::Nat,
}

#[update]
async fn xcall_canister_greet(name: String) -> String {
    let principal = Principal::from_text("ginsq-liaaa-aaaai-qjbrq-cai").unwrap();
    let (result,) = (call(principal, "greet", (name,)).await as CallResult<(String,)>).unwrap();
    result
}

#[derive(Deserialize, CandidType)]
struct GetCanisterStatusArgs {
    canister_id: Principal,
}

#[update]
async fn get_status_of_my_sub_canister() -> CanisterStatusResult {
    let principal = Principal::from_text("ginsq-liaaa-aaaai-qjbrq-cai").unwrap();
    let (result,) = (call(
        Principal::management_canister(),
        "canister_status",
        (GetCanisterStatusArgs {
            canister_id: principal,
        },),
    )
    .await as CallResult<(CanisterStatusResult,)>)
        .unwrap();
    result
}

#[query]
fn get_all_my_canister() -> Vec<Principal> {
    MY_CANISTERS.with(|p| p.borrow().to_vec())
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
    stable_grow(1).unwrap_or(0);
}

#[update]
fn m_stable_write(offset: u64, data: Vec<u8>) {
    stable64_write(offset, &data)
}

#[query]
fn m_stable_read(offset: u64, len: u64) -> Vec<u8> {
    let mut p = [0].repeat(len as usize);

    stable64_read(offset, &mut p);
    p
}

thread_local! {
    static HEAP: RefCell<Vec<String>> = RefCell::default();
}

#[update]
fn grow_heap(s: String) {
    HEAP.with(|h| h.borrow_mut().push(s))
}

#[update]
async fn raw_rand() -> Vec<u8> {
    match call(Principal::management_canister(), "raw_rand", ()).await {
        Ok((res,)) => res,
        Err((_, err)) => trap(&format!("failed to get seed: {}", err)),
    }
}

#[update]
async fn raw_rand_code() -> u32 {
    let res: Vec<u8> = match call(Principal::management_canister(), "raw_rand", ()).await {
        Ok((res,)) => res,
        Err((_, err)) => trap(&format!("failed to get randomness: {}", err)),
    };

    let rand = u32::from_be_bytes(res[..4].try_into().unwrap_or_else(|_| {
        trap(&format!(
            "when creating random code from raw_rand output, expected raw randomness to be of length 32, got {}",
            res.len()
        ));
    }));

    rand
}

#[query]
fn test(val: u64) -> String {
    canister_id_from_u64(val).to_string()
}

fn canister_id_from_u64(val: u64) -> Principal {
    let mut data = [0_u8; 29];
    let val: [u8; 8] = val.to_be_bytes();

    data[0] = val[0];
    data[1] = val[1];
    data[2] = val[2];
    data[3] = val[3];
    data[4] = val[4];
    data[5] = val[5];
    data[6] = val[6];
    data[7] = val[7];
    data[8] = 0x01;

    let blob_lenght: usize = 8 + 1;

    new_opaque_from_array(data, blob_lenght)
}

fn new_opaque_from_array(mut blob: [u8; 29], len: usize) -> Principal {
    blob[len] = 0x01;
    Principal::from_slice(range(&blob, 0..len))
}

use std::ops::Range;

fn range(data: &[u8], r: Range<usize>) -> &[u8] {
    let (start, end) = (r.start, r.end);
    match get(data, r) {
        Some(v) => v,
        None => {
            let _ = &data[start];
            let _ = &data[end];
            let _ = &data[end - start];
            const ASSERT: [(); 1] = [()];
            #[allow(unconditional_panic)]
            let _ = ASSERT[1];

            data
        }
    }
}

fn get(mut data: &[u8], r: Range<usize>) -> Option<&[u8]> {
    if r.start > r.end || data.len() < r.end {
        return None;
    }

    while data.len() > r.end {
        match data {
            [x @ .., _] => data = x,
            [] => {}
        }
    }

    while data.len() > r.end - r.start {
        match data {
            [_, x @ ..] => data = x,
            [] => {}
        }
    }

    Some(data)
}
