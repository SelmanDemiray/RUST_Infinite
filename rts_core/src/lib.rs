use serde::{Deserialize, Serialize};

pub mod protocol;

// Basic Identifiers
pub type PlayerId = u64;
pub type EntityId = u64;
pub type RegionId = u128; // Large enough for hierarchical IDs

// Represents coordinates within the vast universe map
// Might need multiple coordinate systems (local, region, galaxy, etc.)
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Coordinates {
    // Using i128 for potentially huge coordinate space. Consider alternatives
    // like hierarchical IDs + local floats/fixed-point if precision is needed.
    pub x: i128,
    pub y: i128,
    pub z: i128, // Optional 3rd dimension
    pub zoom_level: u8, // Indicates the scale/level of detail
}

// Example basic entity trait
pub trait Entity {
    fn id(&self) -> EntityId;
    fn position(&self) -> Coordinates;
    fn owner(&self) -> PlayerId;
}

// Example Unit struct (very basic)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Unit {
    pub id: EntityId,
    pub owner_id: PlayerId,
    pub position: Coordinates,
    pub hp: u32,
    // ... other unit properties (type, target, etc.)
}

impl Entity for Unit {
    fn id(&self) -> EntityId { self.id }
    fn position(&self) -> Coordinates { self.position }
    fn owner(&self) -> PlayerId { self.owner_id }
}

// Placeholder for map chunk data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MapChunk {
    pub region_id: RegionId,
    pub coordinates: Coordinates, // Top-left corner or center at a specific zoom level
    // pub terrain_data: Vec<u8>, // Simplified terrain representation
    // pub resources: Vec<(ResourceType, u32)>,
    // pub entities_summary: Vec<EntityId>, // IDs of entities within this chunk
}

// --- Add more core game logic structures ---
// - Buildings, Resources, Factions
// - Game state representation
// - Simulation tick logic (if running simulation in core)
// - Hierarchical map indexing logic
