use serde::{Deserialize, Serialize};
use super::{Coordinates, PlayerId, EntityId, MapChunk, Unit}; // Import types from lib.rs

// Messages sent from Client to Server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    Authenticate { token: String },
    RequestLogin { username: String, password_hash: String }, // Hash on client is debatable, often done server-side
    RequestRegister { username: String, email: String, password_hash: String },
    MoveCommand { entity_ids: Vec<EntityId>, target_position: Coordinates },
    BuildCommand { building_type: String, position: Coordinates },
    Chat { message: String },
    RequestMapData { area: Coordinates, zoom_level: u8 }, // Request data for a specific view
    // ... other commands
}

// Messages sent from Server to Client
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    Authenticated { player_id: PlayerId, initial_position: Coordinates },
    AuthenticationFailed { reason: String },
    LoginSuccess { token: String, player_id: PlayerId },
    LoginFailed { reason: String },
    RegisterSuccess,
    RegisterFailed { reason: String },
    GameError { message: String },
    GameStateUpdate {
        // Incremental updates are better for scale
        updated_units: Vec<Unit>,
        removed_entity_ids: Vec<EntityId>,
        // ... other state changes
    },
    MapDataResponse { chunks: Vec<MapChunk> }, // Send relevant map chunks
    ChatBroadcast { player_id: PlayerId, username: String, message: String },
    // ... other updates and events
}

// Consider using a more efficient binary format like Bincode, MessagePack, or Protobuf/FlatBuffers
// For now, using JSON via serde for simplicity.
