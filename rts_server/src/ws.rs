use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use rts_core::protocol::{ClientMessage, ServerMessage};
use super::AppState;
use super::error::AppError; // Use your AppError type
use tracing;

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    // Perform authentication/validation here before upgrading?
    // E.g., require a valid token in the upgrade request headers or query params.
    Ok(ws.on_upgrade(move |socket| handle_socket(socket, state)))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    tracing::info!("WebSocket client connected");

    // --- Authentication Step ---
    // Wait for an initial ClientMessage::Authenticate message
    // Validate the token, fetch PlayerId, etc.
    let player_id = match authenticate_connection(&mut socket, &state).await {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("WebSocket authentication failed: {}", e);
            // Send error message before closing
            let _ = socket.send(Message::Text(serde_json::to_string(&ServerMessage::AuthenticationFailed { reason: e.to_string() }).unwrap_or_default())).await;
            let _ = socket.close().await;
            return;
        }
    };
    tracing::info!("WebSocket authenticated for player_id: {}", player_id);

    // --- Add client to shared state (e.g., DashMap) ---
    // let (sender, receiver) = unbounded(); // Or bounded channel
    // state.connected_clients.insert(player_id, sender);

    // --- Main Loop ---
    // Split the socket for concurrent reading and writing
    let (mut sender, mut receiver) = socket.split();

    // Task to handle receiving messages from this client
    let state_clone = state.clone();
    let receive_handle = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    match serde_json::from_str::<ClientMessage>(&text) {
                        Ok(client_msg) => {
                            if let Err(e) = process_client_message(player_id, client_msg, &state_clone).await {
                                tracing::error!("Error processing client message: {}", e);
                                // Optionally send an error back to the client
                                // let _ = sender.send(...).await;
                            }
                        }
                        Err(e) => {
                            tracing::warn!("Failed to deserialize client message: {}", e);
                        }
                    }
                }
                Message::Binary(_) => {
                    tracing::warn!("Received unexpected binary message");
                }
                Message::Ping(_) | Message::Pong(_) => { /* Handled by axum */ }
                Message::Close(_) => {
                    tracing::info!("Client {} disconnected.", player_id);
                    break;
                }
            }
        }
        // Cleanup when receive loop ends
        // state_clone.connected_clients.remove(&player_id);
        tracing::info!("Receive task finished for player {}", player_id);
    });

    // Task to handle sending messages *to* this client (e.g., from game state updates)
    // let send_handle = tokio::spawn(async move {
    //     while let Ok(server_msg) = receiver_from_gamestate.recv().await {
    //         let msg_text = serde_json::to_string(&server_msg).unwrap_or_default(); // Handle error
    //         if sender.send(Message::Text(msg_text)).await.is_err() {
    //             // Error sending, client likely disconnected
    //             break;
    //         }
    //     }
    //     tracing::info!("Send task finished for player {}", player_id);
    // });


    // Wait for tasks to complete (e.g., client disconnects)
    let _ = receive_handle.await;
    // let _ = send_handle.await; // If using a separate send task

    // --- Cleanup ---
    // state.connected_clients.remove(&player_id); // Ensure removal if not done elsewhere
    tracing::info!("WebSocket connection closed for player {}", player_id);
}

// Placeholder for the initial authentication handshake over WebSocket
async fn authenticate_connection(socket: &mut WebSocket, state: &AppState) -> Result<rts_core::PlayerId, Box<dyn std::error::Error + Send + Sync>> {
    // Expect a specific message (e.g., Authenticate with token)
    if let Some(Ok(Message::Text(text))) = socket.recv().await {
        match serde_json::from_str::<ClientMessage>(&text)? {
            ClientMessage::Authenticate { token } => {
                // --- Validate Token ---
                // This is where you'd verify the JWT or session token
                // For now, just parse it as a PlayerId for demonstration
                let player_id: rts_core::PlayerId = token.strip_prefix("dummy-token-for-").unwrap_or("0").parse()?;
                if player_id == 0 {
                    return Err("Invalid token format".into());
                }

                // --- Check if user exists / is valid ---
                // let user = db::find_user_by_id(&state.db_pool, player_id).await?; // Assuming db function exists

                // --- Send confirmation ---
                let welcome_msg = ServerMessage::Authenticated {
                    player_id,
                    initial_position: rts_core::Coordinates { x: 0, y: 0, z: 0, zoom_level: 10 }, // Fetch actual start pos
                };
                socket.send(Message::Text(serde_json::to_string(&welcome_msg)?)).await?;
                Ok(player_id)
            }
            _ => Err("Expected Authenticate message first".into()),
        }
    } else {
        Err("Failed to receive authentication message".into())
    }
}

// Placeholder for processing messages received from the client
async fn process_client_message(
    player_id: rts_core::PlayerId,
    msg: ClientMessage,
    state: &AppState,
) -> Result<(), AppError> {
    tracing::debug!("Received message from {}: {:?}", player_id, msg);
    match msg {
        ClientMessage::MoveCommand { entity_ids, target_position } => {
            // TODO: Validate command (ownership, distance, etc.)
            // TODO: Update game state (e.g., queue action for entities)
            // TODO: Broadcast necessary updates to other relevant players
            tracing::info!("Player {} move command: {:?} to {:?}", player_id, entity_ids, target_position);
        }
        ClientMessage::RequestMapData { area, zoom_level } => {
            // TODO: Query map data based on area and zoom level (using hierarchical system)
            // TODO: Fetch/generate chunks
            // TODO: Send MapDataResponse back to *this* client
            tracing::info!("Player {} requested map data for {:?} at zoom {}", player_id, area, zoom_level);
            // Example: let chunks = fetch_map_chunks(&state.db_pool, area, zoom_level).await?;
            // let response = ServerMessage::MapDataResponse { chunks };
            // state.connected_clients.get(&player_id).unwrap().send(response)?; // Send back to requester
        }
        ClientMessage::Chat { message } => {
            // TODO: Sanitize message
            // TODO: Broadcast ChatBroadcast to relevant players (e.g., in the same region/faction)
            tracing::info!("Player {} chat: {}", player_id, message);
        }
        // Handle other ClientMessage variants
        _ => {
            tracing::warn!("Unhandled client message type");
        }
    }
    Ok(())
}

