use std::error::Error;
use warp::{Filter, Reply};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{WebSocketStream, accept_async};
use tokio::net::TcpStream;

// API Response types
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

// API Request types
#[derive(Debug, Deserialize)]
pub struct CreateWalletRequest {
    pub email: String,
    pub pin: String,
}

#[derive(Debug, Deserialize)]
pub struct TransferRequest {
    pub from: String,
    pub to: String,
    pub amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateProposalRequest {
    pub title: String,
    pub description: String,
    pub budget_amount: f64,
}

pub struct ApiServer {
    blockchain: Arc<crate::blockchain::Blockchain>,
    wallet: Arc<crate::wallet::Wallet>,
    market: Arc<crate::market::Market>,
    governance: Arc<crate::governance::Governance>,
    notification_tx: broadcast::Sender<serde_json::Value>,
}

impl ApiServer {
    pub fn new(
        blockchain: Arc<crate::blockchain::Blockchain>,
        wallet: Arc<crate::wallet::Wallet>,
        market: Arc<crate::market::Market>,
        governance: Arc<crate::governance::Governance>,
    ) -> Self {
        let (notification_tx, _) = broadcast::channel(100);
        ApiServer {
            blockchain,
            wallet,
            market,
            governance,
            notification_tx,
        }
    }

    pub async fn start(&self, port: u16) -> Result<(), Box<dyn Error>> {
        // REST API routes
        let api = warp::path("api")
            .and(
                // Wallet routes
                self.wallet_routes()
                    .or(self.transaction_routes())
                    .or(self.market_routes())
                    .or(self.governance_routes())
            );

        // WebSocket route
        let ws = warp::path("ws")
            .and(warp::ws())
            .and_then(self.handle_websocket);

        // Combine routes
        let routes = api.or(ws).with(warp::cors().allow_any_origin());

        // Start server
        warp::serve(routes).run(([0, 0, 0, 0], port)).await;

        Ok(())
    }

    fn wallet_routes(&self) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
        let blockchain = self.blockchain.clone();
        let wallet = self.wallet.clone();

        // Create wallet
        let create_wallet = warp::post()
            .and(warp::path("wallet"))
            .and(warp::body::json())
            .and_then(move |req: CreateWalletRequest| {
                let blockchain = blockchain.clone();
                let wallet = wallet.clone();
                async move {
                    // TODO: Implement wallet creation
                    Ok(warp::reply::json(&ApiResponse {
                        success: true,
                        data: Some("Wallet created successfully"),
                        error: None,
                    }))
                }
            });

        // Get wallet balance
        let get_balance = warp::get()
            .and(warp::path("wallet"))
            .and(warp::path("balance"))
            .and_then(move || {
                let wallet = wallet.clone();
                async move {
                    // TODO: Implement balance retrieval
                    Ok(warp::reply::json(&ApiResponse {
                        success: true,
                        data: Some(0.0),
                        error: None,
                    }))
                }
            });

        create_wallet.or(get_balance)
    }

    fn transaction_routes(&self) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
        let blockchain = self.blockchain.clone();

        // Create transaction
        let create_transaction = warp::post()
            .and(warp::path("transaction"))
            .and(warp::body::json())
            .and_then(move |req: TransferRequest| {
                let blockchain = blockchain.clone();
                async move {
                    // TODO: Implement transaction creation
                    Ok(warp::reply::json(&ApiResponse {
                        success: true,
                        data: Some("Transaction created successfully"),
                        error: None,
                    }))
                }
            });

        create_transaction
    }

    fn market_routes(&self) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
        let market = self.market.clone();

        // Get token price
        let get_price = warp::get()
            .and(warp::path("market"))
            .and(warp::path("price"))
            .and(warp::path::param::<String>())
            .and_then(move |symbol: String| {
                let market = market.clone();
                async move {
                    // TODO: Implement price retrieval
                    Ok(warp::reply::json(&ApiResponse {
                        success: true,
                        data: Some(1.5), // Default price
                        error: None,
                    }))
                }
            });

        get_price
    }

    fn governance_routes(&self) -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
        let governance = self.governance.clone();

        // Create proposal
        let create_proposal = warp::post()
            .and(warp::path("governance"))
            .and(warp::path("proposal"))
            .and(warp::body::json())
            .and_then(move |req: CreateProposalRequest| {
                let governance = governance.clone();
                async move {
                    // TODO: Implement proposal creation
                    Ok(warp::reply::json(&ApiResponse {
                        success: true,
                        data: Some("Proposal created successfully"),
                        error: None,
                    }))
                }
            });

        create_proposal
    }

    async fn handle_websocket(
        &self,
        ws: warp::ws::Ws,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        Ok(ws.on_upgrade(|socket| self.handle_websocket_connection(socket)))
    }

    async fn handle_websocket_connection(&self, ws: warp::ws::WebSocket) {
        let (ws_sender, mut ws_receiver) = ws.split();
        let mut notification_rx = self.notification_tx.subscribe();

        // Handle incoming WebSocket messages
        tokio::task::spawn(async move {
            while let Some(result) = ws_receiver.next().await {
                match result {
                    Ok(msg) => {
                        // TODO: Handle incoming WebSocket messages
                    }
                    Err(e) => {
                        eprintln!("WebSocket error: {}", e);
                        break;
                    }
                }
            }
        });

        // Send notifications to WebSocket client
        tokio::task::spawn(async move {
            while let Ok(notification) = notification_rx.recv().await {
                if let Err(e) = ws_sender.send(notification.into()).await {
                    eprintln!("Error sending notification: {}", e);
                    break;
                }
            }
        });
    }
}

// Client-side library for Python
#[cfg(feature = "python")]
pub mod python {
    use pyo3::prelude::*;

    #[pyclass]
    pub struct BlockchainClient {
        api_url: String,
    }

    #[pymethods]
    impl BlockchainClient {
        #[new]
        pub fn new(api_url: String) -> Self {
            BlockchainClient { api_url }
        }

        pub fn create_wallet(&self, email: String, pin: String) -> PyResult<String> {
            // TODO: Implement Python client methods
            Ok("Wallet created".to_string())
        }

        pub fn get_balance(&self) -> PyResult<f64> {
            // TODO: Implement Python client methods
            Ok(0.0)
        }

        pub fn transfer(&self, to: String, amount: f64) -> PyResult<String> {
            // TODO: Implement Python client methods
            Ok("Transfer successful".to_string())
        }
    }
}

// Client-side library for JavaScript
#[cfg(feature = "javascript")]
pub mod javascript {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct BlockchainClient {
        api_url: String,
    }

    #[wasm_bindgen]
    impl BlockchainClient {
        #[wasm_bindgen(constructor)]
        pub fn new(api_url: String) -> Self {
            BlockchainClient { api_url }
        }

        pub async fn create_wallet(&self, email: String, pin: String) -> Result<String, JsValue> {
            // TODO: Implement JavaScript client methods
            Ok("Wallet created".to_string())
        }

        pub async fn get_balance(&self) -> Result<f64, JsValue> {
            // TODO: Implement JavaScript client methods
            Ok(0.0)
        }

        pub async fn transfer(&self, to: String, amount: f64) -> Result<String, JsValue> {
            // TODO: Implement JavaScript client methods
            Ok("Transfer successful".to_string())
        }
    }
} 