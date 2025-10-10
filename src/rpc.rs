use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::types::*;

#[derive(Clone)]
pub struct RpcState {
    pub chain: Arc<Mutex<ChainState>>,
}

#[derive(Serialize)]
struct BalanceResponse {
    address: String,
    balance: u64,
}

#[derive(Serialize)]
struct BlockResponse {
    slot: u64,
    hash: String,
    producer: String,
    timestamp: u64,
    transactions: Vec<Transaction>,
}

#[derive(Deserialize)]
struct SubmitTransactionRequest {
    from: String,
    to: String,
    amount: u64,
    signature: String,
}

#[derive(Serialize)]
struct SubmitTransactionResponse {
    success: bool,
    message: String,
}

async fn get_balance(
    State(state): State<RpcState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<BalanceResponse> {
    let address = payload["address"].as_str().unwrap_or("");
    let chain = state.chain.lock().unwrap();
    let balance = chain.get_balance(address);
    
    Json(BalanceResponse {
        address: address.to_string(),
        balance,
    })
}

async fn get_latest_slot(State(state): State<RpcState>) -> Json<u64> {
    let chain = state.chain.lock().unwrap();
    Json(chain.latest_slot)
}

async fn get_block(
    State(state): State<RpcState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<Option<BlockResponse>> {
    let slot = payload["slot"].as_u64().unwrap_or(0);
    let chain = state.chain.lock().unwrap();
    
    if let Some(block) = chain.blocks.get(&slot) {
        Json(Some(BlockResponse {
            slot: block.slot,
            hash: block.hash.clone(),
            producer: block.producer.clone(),
            timestamp: block.timestamp,
            transactions: block.transactions.clone(),
        }))
    } else {
        Json(None)
    }
}

pub async fn start_rpc_server(addr: &str, chain: Arc<Mutex<ChainState>>) {
    let state = RpcState { chain };
    
    let app = Router::new()
        .route("/balance", post(get_balance))
        .route("/latest_slot", get(get_latest_slot))
        .route("/block", post(get_block))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("RPC server listening on {}", addr);
    
    axum::serve(listener, app).await.unwrap();
}
