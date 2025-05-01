use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebSocket, MessageEvent, ErrorEvent, CloseEvent};
use rts_core::protocol::{ClientMessage, ServerMessage};
use futures_util::{stream::StreamExt, SinkExt}; // For async WebSocket handling if needed
use std::sync::{Mutex, Arc}; // For sharing WebSocket state safely
use once_cell::sync::Lazy; // For static WebSocket instance

// --- Simple Static WebSocket approach (for demonstration) ---
// WARNING: Global statics can be tricky. Consider passing state explicitly or using a dedicated state manager.
// This also doesn't handle reconnection logic robustly.
static WS_INSTANCE: Lazy<Arc<Mutex<Option<WebSocket>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

// Function to establish the connection
pub async fn connect_websocket(url: &str) -> Result<(), JsValue> {
    let ws = WebSocket::new(url)?;
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer); // Or Blob if preferred

    // --- Event Handlers ---
    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            let txt_str: String = txt.into();
            web_sys::console::log_1(&format!("Message from server: {}", txt_str).into());
            match serde_json::from_str::<ServerMessage>(&txt_str) {
                Ok(server_msg) => {
                    handle_server_message(server_msg);
                }
                Err(e) => {
                     web_sys::console::error_1(&format!("Failed to parse server message: {}", e).into());
                }
            }
        } else if let Ok(_blob) = e.data().dyn_into::<web_sys::Blob>() {
             web_sys::console::warn_1(&"Received binary data (not handled)".into());
             // Handle binary data if using binary protocol
        } else {
             web_sys::console::warn_1(&"Received unknown message type".into());
        }
    });

    let onerror_callback = Closure::<dyn FnMut(_)>::new(|e: ErrorEvent| {
        web_sys::console::error_1(&format!("WebSocket error: {:?}", e).into());
    });

    let onopen_callback = Closure::<dyn FnMut()>::new(|| {
        web_sys::console::log_1(&"WebSocket connection opened.".into());
        // Connection is open, now safe to send authentication or other initial messages
        // Example: Send dummy auth token
        send_ws_message(ClientMessage::Authenticate { token: "dummy-token-for-123".to_string() }); // Replace with real token logic
    });

     let onclose_callback = Closure::<dyn FnMut(_)>::new(|e: CloseEvent| {
        web_sys::console::log_1(&format!("WebSocket closed: code={}, reason={}", e.code(), e.reason()).into());
        *WS_INSTANCE.lock().unwrap() = None; // Clear the static instance on close
        // Implement reconnection logic here if desired
    });

    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));

    // Keep closures alive
    onmessage_callback.forget();
    onerror_callback.forget();
    onopen_callback.forget();
    onclose_callback.forget();

    // Store the WebSocket instance
    *WS_INSTANCE.lock().unwrap() = Some(ws);

    Ok(())
}

// Function to send messages (called from lib.rs or other modules)
pub fn send_ws_message(message: ClientMessage) {
    if let Some(ws) = &*WS_INSTANCE.lock().unwrap() {
        match serde_json::to_string(&message) {
            Ok(msg_str) => {
                match ws.send_with_str(&msg_str) {
                    Ok(_) => web_sys::console::log_1(&format!("Sent message: {:?}", msg_str).into()),
                    Err(e) => web_sys::console::error_1(&format!("Error sending message: {:?}", e).into()),
                }
            }
            Err(e) => {
                 web_sys::console::error_1(&format!("Failed to serialize client message: {}", e).into());
            }
        }
    } else {
        web_sys::console::warn_1(&"WebSocket not connected, cannot send message.".into());
    }
}

// Placeholder function to process messages received from the server
fn handle_server_message(message: ServerMessage) {
     web_sys::console::log_1(&format!("Handling Server Message: {:?}", message).into());
    match message {
        ServerMessage::Authenticated { player_id, initial_position } => {
            web_sys::console::log_1(&format!("Authenticated as player {}, starting at {:?}", player_id, initial_position).into());
            // TODO: Update client game state, potentially move camera
        }
        ServerMessage::GameStateUpdate { updated_units, removed_entity_ids, .. } => {
             web_sys::console::log_1(&format!("Received game state update: {} units updated, {} removed", updated_units.len(), removed_entity_ids.len()).into());
            // TODO: Apply updates to client-side game state representation
            // TODO: Update rendering based on new state
        }
        ServerMessage::MapDataResponse { chunks } => {
             web_sys::console::log_1(&format!("Received {} map chunks", chunks.len()).into());
            // TODO: Process map chunk data, update local map representation/cache
            // TODO: Trigger rendering updates for the new map data
        }
        ServerMessage::ChatBroadcast { username, message, .. } => {
             web_sys::console::log_1(&format!("Chat [{}] {}", username, message).into());
            // TODO: Display chat message in the UI
        }
        ServerMessage::AuthenticationFailed { reason } => {
            web_sys::console::error_1(&format!("Authentication failed: {}", reason).into());
            // TODO: Show error to user, potentially redirect to login
        }
        // Handle other ServerMessage variants
        _ => {
             web_sys::console::warn_1(&"Unhandled server message type".into());
        }
    }
}
