use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, WebSocketStream};
use futures_util::{SinkExt, StreamExt};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    NewBlock(Block),
    NewTransaction(Transaction),
    GetBlocks(Vec<String>),
    Blocks(Vec<Block>),
    GetPeers,
    Peers(Vec<PeerInfo>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub address: String,
    pub version: String,
    pub last_seen: chrono::DateTime<chrono::Utc>,
}

pub struct Network {
    peers: Arc<HashMap<String, WebSocketStream<TcpStream>>>,
    message_tx: broadcast::Sender<NetworkMessage>,
    message_rx: broadcast::Receiver<NetworkMessage>,
}

impl Network {
    pub fn new() -> Self {
        let (message_tx, message_rx) = broadcast::channel(100);
        Network {
            peers: Arc::new(HashMap::new()),
            message_tx,
            message_rx,
        }
    }

    pub async fn start(&self, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(addr).await?;
        println!("Network listening on {}", addr);

        while let Ok((stream, addr)) = listener.accept().await {
            println!("New connection from {}", addr);
            let message_tx = self.message_tx.clone();
            let peers = self.peers.clone();
            
            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, addr, message_tx, peers).await {
                    eprintln!("Error handling connection: {}", e);
                }
            });
        }

        Ok(())
    }

    pub async fn connect_to_peer(&self, addr: String) -> Result<(), Box<dyn Error>> {
        let stream = TcpStream::connect(&addr).await?;
        let ws_stream = accept_async(stream).await?;
        
        let message_tx = self.message_tx.clone();
        let peers = self.peers.clone();
        
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, addr.parse().unwrap(), message_tx, peers).await {
                eprintln!("Error handling connection: {}", e);
            }
        });

        Ok(())
    }

    pub async fn broadcast_message(&self, message: NetworkMessage) -> Result<(), Box<dyn Error>> {
        let message = serde_json::to_string(&message)?;
        let peers = self.peers.clone();
        
        for (_, peer) in peers.iter() {
            if let Err(e) = peer.send(message.clone().into()).await {
                eprintln!("Error broadcasting message: {}", e);
            }
        }

        Ok(())
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    message_tx: broadcast::Sender<NetworkMessage>,
    peers: Arc<HashMap<String, WebSocketStream<TcpStream>>>,
) -> Result<(), Box<dyn Error>> {
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    
    // Add peer to peers list
    peers.insert(addr.to_string(), ws_stream);
    
    // Handle incoming messages
    while let Some(msg) = ws_receiver.next().await {
        if let Ok(msg) = msg {
            if let Ok(text) = msg.to_text() {
                if let Ok(message) = serde_json::from_str::<NetworkMessage>(text) {
                    // Broadcast message to other peers
                    message_tx.send(message.clone())?;
                }
            }
        }
    }
    
    // Remove peer when disconnected
    peers.remove(&addr.to_string());
    
    Ok(())
}

// WebSocket Server for real-time notifications
pub struct WebSocketServer {
    clients: Arc<HashMap<String, WebSocketStream<TcpStream>>>,
}

impl WebSocketServer {
    pub fn new() -> Self {
        WebSocketServer {
            clients: Arc::new(HashMap::new()),
        }
    }

    pub async fn start(&self, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(addr).await?;
        println!("WebSocket server listening on {}", addr);

        while let Ok((stream, _)) = listener.accept().await {
            let clients = self.clients.clone();
            
            tokio::spawn(async move {
                if let Err(e) = handle_websocket_connection(stream, clients).await {
                    eprintln!("Error handling WebSocket connection: {}", e);
                }
            });
        }

        Ok(())
    }

    pub async fn broadcast_notification(&self, notification: serde_json::Value) -> Result<(), Box<dyn Error>> {
        let message = serde_json::to_string(&notification)?;
        let clients = self.clients.clone();
        
        for (_, client) in clients.iter() {
            if let Err(e) = client.send(message.clone().into()).await {
                eprintln!("Error broadcasting notification: {}", e);
            }
        }

        Ok(())
    }
}

async fn handle_websocket_connection(
    stream: TcpStream,
    clients: Arc<HashMap<String, WebSocketStream<TcpStream>>>,
) -> Result<(), Box<dyn Error>> {
    let ws_stream = accept_async(stream).await?;
    let addr = ws_stream.peer_addr()?.to_string();
    
    // Add client to clients list
    clients.insert(addr.clone(), ws_stream);
    
    // Remove client when disconnected
    clients.remove(&addr);
    
    Ok(())
} 