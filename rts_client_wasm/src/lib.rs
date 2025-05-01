use wasm_bindgen::prelude::*;
use web_sys::console;
use rts_core::protocol::{ClientMessage, ServerMessage}; // Use shared protocol

mod network;
// mod rendering; // Add later for graphics
// mod game_state; // Add later for client-side state prediction/interpolation
// mod ui; // Add later for UI elements

// Called once when the WASM module is loaded.
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Set up a panic hook to log Rust panics to the browser console
    console_error_panic_hook::set_once();

    console::log_1(&"RTS WASM Client Initializing...".into());

    // --- Get Canvas Element (Example) ---
    // let document = web_sys::window().unwrap().document().unwrap();
    // let canvas = document.get_element_by_id("game-canvas").unwrap();
    // let canvas: web_sys::HtmlCanvasElement = canvas
    //     .dyn_into::<web_sys::HtmlCanvasElement>()?;
    // console::log_1(&"Found canvas".into());
    // Initialize rendering context here...

    // --- Connect to WebSocket Server ---
    wasm_bindgen_futures::spawn_local(async {
        match network::connect_websocket("ws://127.0.0.1:8080/ws").await { // Use server address
            Ok(_) => {
                console::log_1(&"WebSocket connection initiated.".into());
                // Send initial authentication message after connection is established in network.rs
            }
            Err(e) => {
                console::error_1(&format!("Failed to connect WebSocket: {:?}", e).into());
            }
        }
    });


    // --- Setup Game Loop (Example using requestAnimationFrame) ---
    // This would typically be handled by a game engine or a custom loop structure
    // rendering::start_game_loop();

    console::log_1(&"RTS WASM Client Initialized.".into());
    Ok(())
}

// Example function callable from JavaScript (if needed)
#[wasm_bindgen]
pub fn greet(name: &str) {
    console::log_1(&format!("Hello from Rust, {}!", name).into());
}

// Example function to send a message (called from UI or game logic)
pub fn send_message_to_server(message: ClientMessage) {
    // This needs access to the WebSocket connection managed in network.rs
    // Use a global static or pass the connection object around.
    console::log_1(&format!("Attempting to send message: {:?}", message).into());
    network::send_ws_message(message); // Assumes send_ws_message exists in network module
}
