use axum::{
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum::extract::ws::{WebSocket, Message};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tower_http::services::ServeDir;
use crate::types::*;
use crate::metrics::{Metrics, StatusResponse};

#[derive(Clone)]
pub struct RpcState {
    pub chain: Arc<RwLock<ChainState>>,
    pub mempool: Arc<Mutex<Mempool>>,
    pub metrics: Arc<Mutex<Metrics>>,
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
    from_pubkey: String,
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
    let chain = state.chain.read().await;
    let balance = chain.get_balance(address);
    Json(BalanceResponse {
        address: address.to_string(),
        balance,
    })
}

async fn get_latest_slot(State(state): State<RpcState>) -> Json<u64> {
    let chain = state.chain.read().await;
    Json(chain.latest_slot)
}

async fn get_block(
    State(state): State<RpcState>,
    Json(payload): Json<serde_json::Value>,
) -> Json<Option<BlockResponse>> {
    let slot = payload["slot"].as_u64().unwrap_or(0);
    let chain = state.chain.read().await;
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

async fn submit_transaction(
    State(state): State<RpcState>,
    Json(payload): Json<SubmitTransactionRequest>,
) -> Json<SubmitTransactionResponse> {
    let tx = Transaction {
        from: payload.from,
        from_pubkey: payload.from_pubkey,
        to: payload.to,
        amount: payload.amount,
        signature: payload.signature,
    };
    let mut mempool = state.mempool.lock().await;
    mempool.add_transaction(tx);
    let len = mempool.len();
    Json(SubmitTransactionResponse {
        success: true,
        message: format!("Transaction added to mempool ({} pending)", len),
    })
}

async fn get_status(State(state): State<RpcState>) -> Json<StatusResponse> {
    let metrics = state.metrics.lock().await;
    Json(metrics.get_status())
}

async fn get_blocks(State(state): State<RpcState>) -> Json<Vec<crate::metrics::BlockMetric>> {
    let metrics = state.metrics.lock().await;
    Json(metrics.get_blocks())
}

async fn get_peers(State(state): State<RpcState>) -> Json<Vec<crate::metrics::PeerMetric>> {
    let metrics = state.metrics.lock().await;
    Json(metrics.get_peers())
}

async fn get_transactions(State(state): State<RpcState>) -> Json<Vec<crate::metrics::TxMetric>> {
    let metrics = state.metrics.lock().await;
    Json(metrics.get_transactions())
}

async fn get_logs(State(state): State<RpcState>) -> Json<Vec<crate::metrics::LogEntry>> {
    let metrics = state.metrics.lock().await;
    Json(metrics.get_logs())
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<RpcState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

async fn handle_websocket(socket: WebSocket, state: RpcState) {
    let (mut sender, mut receiver) = socket.split();

    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let status = {
                    let metrics = state.metrics.lock().await;
                    metrics.get_status()
                };

                if let Ok(json) = serde_json::to_string(&serde_json::json!({
                    "type": "status",
                    "current_slot": status.current_slot,
                    "blocks_produced": status.blocks_produced,
                    "mempool_size": status.mempool_size,
                    "connected_peers": status.connected_peers,
                    "uptime_seconds": status.uptime_seconds,
                    "avg_block_time": status.avg_block_time,
                    "current_tps": status.current_tps,
                    "avg_tps": status.avg_tps,
                    "memory_mb": status.memory_mb,
                    "cpu_percent": status.cpu_percent,
                })) {
                    if sender.send(Message::Text(json)).await.is_err() {
                        break;
                    }
                }
            }
            msg = receiver.next() => {
                if msg.is_none() {
                    break;
                }
            }
        }
    }
}

pub async fn start_rpc_server(
    addr: &str,
    chain: Arc<RwLock<ChainState>>,
    mempool: Arc<Mutex<Mempool>>,
    metrics: Arc<Mutex<Metrics>>,
) {
    let state = RpcState { chain, mempool, metrics };

    let app = Router::new()
        .route("/balance", post(get_balance))
        .route("/latest_slot", get(get_latest_slot))
        .route("/block", post(get_block))
        .route("/submit", post(submit_transaction))
        .route("/status", get(get_status))
        .route("/blocks", get(get_blocks))
        .route("/peers", get(get_peers))
        .route("/transactions", get(get_transactions))
        .route("/logs", get(get_logs))
        .route("/ws", get(websocket_handler))
        .nest_service("/dashboard", ServeDir::new("testing"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("RPC server listening on {}", addr);
    println!("Dashboard available at http://localhost:3000/dashboard");

    axum::serve(listener, app).await.unwrap();
}
