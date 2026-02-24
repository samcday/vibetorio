use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use std::collections::hash_map::DefaultHasher;

pub const DEFAULT_TICKS_PER_SECOND: u32 = 60;
pub const CHUNK_SIZE: i32 = 16;
pub const DEFAULT_COMMAND_QUEUE_CAPACITY: usize = 128;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityFacing {
    #[default]
    North,
    East,
    South,
    West,
}

impl EntityFacing {
    pub const fn rotate_clockwise(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityFootprint {
    pub width: i32,
    pub height: i32,
}

impl EntityFootprint {
    fn validate(&self) -> Result<(), SimulationError> {
        if self.width <= 0 || self.height <= 0 {
            return Err(SimulationError::InvalidFootprint(self.width, self.height));
        }

        Ok(())
    }
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub ticks_per_second: u32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            ticks_per_second: DEFAULT_TICKS_PER_SECOND,
        }
    }
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct SimulationClock {
    pub tick: u64,
    pub ticks_per_second: u32,
}

impl Default for SimulationClock {
    fn default() -> Self {
        Self {
            tick: 0,
            ticks_per_second: DEFAULT_TICKS_PER_SECOND,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkCell {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEntity {
    pub id: u32,
    pub entity_type: String,
    pub origin: GridPosition,
    pub footprint: EntityFootprint,
    pub facing: EntityFacing,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Chunk {
    pub entities: HashMap<ChunkCell, u32>,
}

fn occupied_cells_for(
    origin: GridPosition,
    footprint: EntityFootprint,
    facing: EntityFacing,
) -> Vec<GridPosition> {
    let mut cells = Vec::with_capacity((footprint.width * footprint.height) as usize);

    for local_y in 0..footprint.height {
        for local_x in 0..footprint.width {
            let (x, y) = match facing {
                EntityFacing::North => (local_x, local_y),
                EntityFacing::East => (local_y, footprint.width - local_x - 1),
                EntityFacing::South => (
                    footprint.width - local_x - 1,
                    footprint.height - local_y - 1,
                ),
                EntityFacing::West => (footprint.height - local_y - 1, local_x),
            };

            cells.push(GridPosition {
                x: origin.x + x,
                y: origin.y + y,
            });
        }
    }

    cells
}

impl Chunk {
    fn is_occupied(&self, cell: ChunkCell) -> bool {
        self.entities.contains_key(&cell)
    }
}

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct WorldGrid {
    pub chunks: HashMap<ChunkCoord, Chunk>,
    pub entities: HashMap<u32, WorldEntity>,
    pub next_entity_id: u32,
}

impl Default for WorldGrid {
    fn default() -> Self {
        Self {
            chunks: HashMap::new(),
            entities: HashMap::new(),
            next_entity_id: 1,
        }
    }
}

impl WorldGrid {
    pub fn deterministic_state_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.next_entity_id.hash(&mut hasher);

        let mut sorted_entity_ids: Vec<u32> = self.entities.keys().copied().collect();
        sorted_entity_ids.sort_unstable();

        for entity_id in sorted_entity_ids {
            entity_id.hash(&mut hasher);

            if let Some(entity) = self.entities.get(&entity_id) {
                entity.origin.hash(&mut hasher);
                entity.facing.hash(&mut hasher);
                entity.id.hash(&mut hasher);
                entity.entity_type.hash(&mut hasher);
                entity.footprint.hash(&mut hasher);
            }
        }

        hasher.finish()
    }

    pub fn chunk_for(pos: GridPosition) -> ChunkCoord {
        ChunkCoord {
            x: pos.x.div_euclid(CHUNK_SIZE),
            y: pos.y.div_euclid(CHUNK_SIZE),
        }
    }

    pub fn local_cell_for(pos: GridPosition) -> ChunkCell {
        ChunkCell {
            x: pos.x.rem_euclid(CHUNK_SIZE),
            y: pos.y.rem_euclid(CHUNK_SIZE),
        }
    }

    pub fn has_entity_at(&self, pos: GridPosition) -> bool {
        let chunk_pos = Self::chunk_for(pos);
        let local_pos = Self::local_cell_for(pos);

        self.chunks
            .get(&chunk_pos)
            .is_some_and(|chunk| chunk.is_occupied(local_pos))
    }

    pub fn entity_id_at(&self, pos: GridPosition) -> Option<u32> {
        let chunk_pos = Self::chunk_for(pos);
        let local_pos = Self::local_cell_for(pos);

        self.chunks
            .get(&chunk_pos)
            .and_then(|chunk| chunk.entities.get(&local_pos).copied())
    }

    pub fn chunk_count(&self) -> u32 {
        self.chunks.len() as u32
    }

    pub fn entity_count(&self) -> u32 {
        self.entities.len() as u32
    }

    fn ensure_can_place(
        &self,
        origin: GridPosition,
        footprint: EntityFootprint,
        facing: EntityFacing,
        ignore_entity: Option<u32>,
    ) -> Result<Vec<GridPosition>, SimulationError> {
        footprint.validate()?;

        let cells = occupied_cells_for(origin, footprint, facing);

        for cell in &cells {
            if let Some(entity_id) = self.entity_id_at(*cell)
                && Some(entity_id) != ignore_entity
            {
                return Err(SimulationError::TileOccupied(cell.x, cell.y));
            }
        }

        Ok(cells)
    }

    fn place_entity_cells(&mut self, entity_id: u32, cells: &[GridPosition]) {
        for cell in cells {
            let chunk_coord = Self::chunk_for(*cell);
            let local_cell = Self::local_cell_for(*cell);
            let chunk = self.chunks.entry(chunk_coord).or_default();

            chunk.entities.insert(local_cell, entity_id);
        }
    }

    fn clear_entity_cells(&mut self, entity_id: u32, cells: &[GridPosition]) {
        for cell in cells {
            let chunk_coord = Self::chunk_for(*cell);
            let local_cell = Self::local_cell_for(*cell);

            if let Some(chunk) = self.chunks.get_mut(&chunk_coord)
                && chunk.entities.get(&local_cell).copied() == Some(entity_id)
            {
                chunk.entities.remove(&local_cell);
            }
        }

        let empty_chunks: Vec<ChunkCoord> = self
            .chunks
            .iter()
            .filter_map(|(coord, chunk)| {
                if chunk.entities.is_empty() {
                    Some(*coord)
                } else {
                    None
                }
            })
            .collect();

        for coord in empty_chunks {
            self.chunks.remove(&coord);
        }
    }

    pub fn place_entity(
        &mut self,
        position: GridPosition,
        entity_type: String,
        footprint: EntityFootprint,
        facing: EntityFacing,
    ) -> Result<u32, SimulationError> {
        let cells = self.ensure_can_place(position, footprint, facing, None)?;

        let entity_id = self.next_entity_id;
        if entity_id == u32::MAX {
            return Err(SimulationError::EntityIdExhausted);
        }

        self.next_entity_id = self.next_entity_id.saturating_add(1);

        let entity = WorldEntity {
            id: entity_id,
            entity_type,
            origin: position,
            footprint,
            facing,
        };

        self.place_entity_cells(entity_id, &cells);
        self.entities.insert(entity_id, entity);

        Ok(entity_id)
    }

    pub fn remove_entity(&mut self, entity_id: u32) -> Result<WorldEntity, SimulationError> {
        let entity = self
            .entities
            .remove(&entity_id)
            .ok_or(SimulationError::EntityNotFound(entity_id))?;

        let cells = occupied_cells_for(entity.origin, entity.footprint, entity.facing);
        self.clear_entity_cells(entity_id, &cells);

        Ok(entity)
    }

    pub fn remove_entity_at(
        &mut self,
        position: GridPosition,
    ) -> Result<WorldEntity, SimulationError> {
        let entity_id = self
            .entity_id_at(position)
            .ok_or(SimulationError::NoEntity(position.x, position.y))?;

        self.remove_entity(entity_id)
    }

    pub fn move_entity(
        &mut self,
        entity_id: u32,
        destination: GridPosition,
    ) -> Result<(), SimulationError> {
        let mut entity = self
            .entities
            .get(&entity_id)
            .cloned()
            .ok_or(SimulationError::EntityNotFound(entity_id))?;

        if entity.origin == destination {
            return Ok(());
        }

        let destination_cells = self.ensure_can_place(
            destination,
            entity.footprint,
            entity.facing,
            Some(entity_id),
        )?;

        let source_cells = occupied_cells_for(entity.origin, entity.footprint, entity.facing);
        self.clear_entity_cells(entity_id, &source_cells);

        entity.origin = destination;
        self.place_entity_cells(entity_id, &destination_cells);
        self.entities.insert(entity_id, entity);

        Ok(())
    }

    pub fn rotate_entity(&mut self, entity_id: u32, times: u8) -> Result<(), SimulationError> {
        let rotations = usize::from(times) % 4;
        if rotations == 0 {
            return Ok(());
        }

        let mut entity = self
            .entities
            .get(&entity_id)
            .cloned()
            .ok_or(SimulationError::EntityNotFound(entity_id))?;

        let source_cells = occupied_cells_for(entity.origin, entity.footprint, entity.facing);
        let target_facing = (0..rotations).fold(entity.facing, |acc, _| acc.rotate_clockwise());

        self.ensure_can_place(
            entity.origin,
            entity.footprint,
            target_facing,
            Some(entity_id),
        )?;
        self.clear_entity_cells(entity_id, &source_cells);

        entity.facing = target_facing;
        let target_cells = occupied_cells_for(entity.origin, entity.footprint, target_facing);
        self.place_entity_cells(entity_id, &target_cells);
        self.entities.insert(entity_id, entity);

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimulationCommand {
    PlaceEntity {
        entity_type: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        facing: EntityFacing,
    },
    RemoveEntity {
        x: i32,
        y: i32,
    },
    RemoveEntityById {
        entity_id: u32,
    },
    MoveEntity {
        entity_id: u32,
        x: i32,
        y: i32,
    },
    RotateEntity {
        entity_id: u32,
        times: u8,
    },
    SetTickRate(u32),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SimulationCommandStats {
    pub processed: u64,
    pub rejected: u64,
    pub dropped: u64,
}

#[derive(Default, Resource, Serialize, Deserialize)]
pub struct CommandQueue {
    pub commands: Vec<SimulationCommand>,
    pub stats: SimulationCommandStats,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum SimulationError {
    #[error("command queue is full")]
    QueueFull,
    #[error("tick rate must be greater than zero")]
    InvalidTickRate,
    #[error("entity footprint must be greater than zero (got {0}x{1})")]
    InvalidFootprint(i32, i32),
    #[error("tile ({0}, {1}) is already occupied")]
    TileOccupied(i32, i32),
    #[error("no entity at tile ({0}, {1})")]
    NoEntity(i32, i32),
    #[error("entity {0} not found")]
    EntityNotFound(u32),
    #[error("entity id space exhausted")]
    EntityIdExhausted,
}

impl CommandQueue {
    pub fn enqueue(&mut self, command: SimulationCommand) -> Result<(), SimulationError> {
        if self.commands.len() >= DEFAULT_COMMAND_QUEUE_CAPACITY {
            self.stats.dropped = self.stats.dropped.saturating_add(1);
            return Err(SimulationError::QueueFull);
        }

        self.commands.push(command);
        Ok(())
    }
}

#[derive(Resource, Clone, Default, Serialize, Deserialize)]
pub struct SimulationWorldState {
    pub initialized_tick: u64,
    pub ticks_simulated: u64,
    pub chunk_count: u32,
    pub entity_count: u32,
}

pub fn step_clock(mut clock: ResMut<SimulationClock>) {
    clock.tick = clock.tick.saturating_add(1);
    clock.ticks_per_second = clock.ticks_per_second.max(1);
}

pub fn init_world_state(mut world: ResMut<SimulationWorldState>, clock: Res<SimulationClock>) {
    world.initialized_tick = clock.tick;
}

pub fn step_world(mut world_state: ResMut<SimulationWorldState>, world: Res<WorldGrid>) {
    world_state.ticks_simulated = world_state.ticks_simulated.saturating_add(1);
    world_state.chunk_count = world.chunk_count();
    world_state.entity_count = world.entity_count();
}

pub fn process_command_queue(
    mut queue: ResMut<CommandQueue>,
    mut world: ResMut<WorldGrid>,
    mut config: ResMut<SimulationConfig>,
) {
    let commands = queue.commands.drain(..).collect::<Vec<_>>();

    for command in commands {
        if apply_command(command, &mut world, &mut config).is_ok() {
            queue.stats.processed = queue.stats.processed.saturating_add(1);
        } else {
            queue.stats.rejected = queue.stats.rejected.saturating_add(1);
        }
    }
}

pub fn apply_command(
    command: SimulationCommand,
    world: &mut WorldGrid,
    config: &mut SimulationConfig,
) -> Result<(), SimulationError> {
    match command {
        SimulationCommand::SetTickRate(tick_rate) => {
            if tick_rate == 0 {
                return Err(SimulationError::InvalidTickRate);
            }

            config.ticks_per_second = tick_rate;
        }
        SimulationCommand::PlaceEntity {
            entity_type,
            x,
            y,
            width,
            height,
            facing,
        } => {
            world.place_entity(
                GridPosition { x, y },
                entity_type,
                EntityFootprint { width, height },
                facing,
            )?;
        }
        SimulationCommand::RemoveEntity { x, y } => {
            world.remove_entity_at(GridPosition { x, y })?;
        }
        SimulationCommand::RemoveEntityById { entity_id } => {
            world.remove_entity(entity_id)?;
        }
        SimulationCommand::MoveEntity { entity_id, x, y } => {
            world.move_entity(entity_id, GridPosition { x, y })?;
        }
        SimulationCommand::RotateEntity { entity_id, times } => {
            world.rotate_entity(entity_id, times)?;
        }
    }

    Ok(())
}

pub fn deterministic_sim_state_hash(
    clock: &SimulationClock,
    config: &SimulationConfig,
    world: &WorldGrid,
) -> u64 {
    let mut hasher = DefaultHasher::new();

    clock.tick.hash(&mut hasher);
    clock.ticks_per_second.hash(&mut hasher);
    config.ticks_per_second.hash(&mut hasher);
    world.deterministic_state_hash().hash(&mut hasher);

    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clock_steps() {
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(step_clock);

        let mut world = bevy_ecs::world::World::new();
        world.insert_resource(SimulationClock::default());
        schedule.run(&mut world);

        let clock = world.resource::<SimulationClock>();
        assert_eq!(clock.tick, 1);
    }

    #[test]
    fn command_with_zero_tick_rate_errors() {
        let mut config = SimulationConfig::default();
        let mut world = WorldGrid::default();
        let result = apply_command(SimulationCommand::SetTickRate(0), &mut world, &mut config);

        assert!(matches!(result, Err(SimulationError::InvalidTickRate)));
    }

    #[test]
    fn chunk_mapping_handles_negative_positions() {
        let position = GridPosition { x: -1, y: -16 };
        let chunk = WorldGrid::chunk_for(position);
        let cell = WorldGrid::local_cell_for(position);

        assert_eq!(chunk, ChunkCoord { x: -1, y: -1 });
        assert_eq!(cell, ChunkCell { x: 15, y: 0 });
    }

    #[test]
    fn can_place_and_remove_entity() {
        let mut world = WorldGrid::default();

        let position = GridPosition { x: 4, y: 4 };
        world
            .place_entity(
                position,
                "smelter".to_string(),
                EntityFootprint {
                    width: 1,
                    height: 1,
                },
                EntityFacing::North,
            )
            .expect("failed to place entity");
        assert_eq!(world.entity_count(), 1);
        assert!(world.has_entity_at(position));

        let removed = world
            .remove_entity_at(position)
            .expect("failed to remove entity");
        assert_eq!(removed.entity_type, "smelter");
        assert_eq!(world.entity_count(), 0);
        assert!(!world.has_entity_at(position));
    }

    #[test]
    fn command_queue_rejects_full_backlog() {
        let mut queue = CommandQueue::default();
        for idx in 0..DEFAULT_COMMAND_QUEUE_CAPACITY {
            queue
                .enqueue(SimulationCommand::SetTickRate(idx.saturating_add(1) as u32))
                .expect("queue should have room during setup");
        }

        let overflow = queue.enqueue(SimulationCommand::SetTickRate(2));
        assert!(matches!(overflow, Err(SimulationError::QueueFull)));
        assert_eq!(queue.stats.dropped, 1);
    }

    #[test]
    fn placing_occupied_tile_errors() {
        let mut world = WorldGrid::default();
        let mut config = SimulationConfig::default();

        world
            .place_entity(
                GridPosition { x: 1, y: 1 },
                "a".to_string(),
                EntityFootprint {
                    width: 1,
                    height: 1,
                },
                EntityFacing::North,
            )
            .expect("expected to place first entity");

        let result = apply_command(
            SimulationCommand::PlaceEntity {
                entity_type: "b".to_string(),
                x: 1,
                y: 1,
                width: 1,
                height: 1,
                facing: EntityFacing::North,
            },
            &mut world,
            &mut config,
        );

        assert!(matches!(result, Err(SimulationError::TileOccupied(1, 1))));
        assert_eq!(world.entity_count(), 1);
    }

    #[test]
    fn moving_entity_uses_occupancy_validation() {
        let mut world = WorldGrid::default();

        let first = world
            .place_entity(
                GridPosition { x: 0, y: 0 },
                "belt".to_string(),
                EntityFootprint {
                    width: 2,
                    height: 1,
                },
                EntityFacing::North,
            )
            .expect("place first entity");
        world
            .place_entity(
                GridPosition { x: 3, y: 0 },
                "wall".to_string(),
                EntityFootprint {
                    width: 1,
                    height: 1,
                },
                EntityFacing::North,
            )
            .expect("place second entity");

        let collision = world.move_entity(first, GridPosition { x: 3, y: 0 });
        assert!(matches!(
            collision,
            Err(SimulationError::TileOccupied(3, 0))
        ));

        world
            .move_entity(first, GridPosition { x: 1, y: 2 })
            .expect("move should succeed");
        assert!(world.has_entity_at(GridPosition { x: 1, y: 2 }));
        assert!(!world.has_entity_at(GridPosition { x: 0, y: 0 }));
        assert!(
            world
                .entity_id_at(GridPosition { x: 1, y: 2 })
                .is_some_and(|id| id == first)
        );
    }

    #[test]
    fn rotating_entity_rejects_blocked_orientation() {
        let mut world = WorldGrid::default();

        world
            .place_entity(
                GridPosition { x: 0, y: 0 },
                "pole".to_string(),
                EntityFootprint {
                    width: 2,
                    height: 1,
                },
                EntityFacing::North,
            )
            .expect("place rotatable entity");
        world
            .place_entity(
                GridPosition { x: 0, y: 1 },
                "wall".to_string(),
                EntityFootprint {
                    width: 1,
                    height: 1,
                },
                EntityFacing::North,
            )
            .expect("place blocking entity");

        let mut id = 0;
        for (entity_id, entity) in &world.entities {
            if entity.entity_type == "pole" {
                id = *entity_id;
            }
        }

        let rotate_result = world.rotate_entity(id, 1);
        assert!(matches!(
            rotate_result,
            Err(SimulationError::TileOccupied(0, 1))
        ));

        assert_eq!(
            world.entities.get(&id).expect("entity must remain").facing,
            EntityFacing::North,
        );
    }

    #[test]
    fn process_command_queue_updates_state() {
        let mut queue = CommandQueue::default();
        queue
            .enqueue(SimulationCommand::SetTickRate(30))
            .expect("queue should accept tick-rate change");
        queue
            .enqueue(SimulationCommand::PlaceEntity {
                entity_type: "bench".to_string(),
                x: 10,
                y: 10,
                width: 1,
                height: 1,
                facing: EntityFacing::North,
            })
            .expect("queue should accept place command");

        let mut world = bevy_ecs::world::World::new();
        world.insert_resource(queue);
        world.insert_resource(WorldGrid::default());
        world.insert_resource(SimulationConfig::default());

        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(process_command_queue);
        schedule.run(&mut world);

        let queue = world.resource::<CommandQueue>();
        let config = world.resource::<SimulationConfig>();
        let world_state = world.resource::<WorldGrid>();

        assert_eq!(queue.stats.processed, 2);
        assert_eq!(queue.commands.len(), 0);
        assert_eq!(config.ticks_per_second, 30);
        assert_eq!(world_state.entity_count(), 1);
    }

    #[test]
    fn rotate_clockwise_covers_all_directions() {
        assert_eq!(EntityFacing::North.rotate_clockwise(), EntityFacing::East);
        assert_eq!(EntityFacing::East.rotate_clockwise(), EntityFacing::South);
        assert_eq!(EntityFacing::South.rotate_clockwise(), EntityFacing::West);
        assert_eq!(EntityFacing::West.rotate_clockwise(), EntityFacing::North);
    }

    #[test]
    fn occupied_cells_cover_each_facing() {
        let origin = GridPosition { x: 10, y: 20 };
        let footprint = EntityFootprint {
            width: 2,
            height: 3,
        };

        let north = occupied_cells_for(origin, footprint, EntityFacing::North);
        let east = occupied_cells_for(origin, footprint, EntityFacing::East);
        let south = occupied_cells_for(origin, footprint, EntityFacing::South);
        let west = occupied_cells_for(origin, footprint, EntityFacing::West);

        assert_eq!(north.len(), 6);
        assert_eq!(east.len(), 6);
        assert_eq!(south.len(), 6);
        assert_eq!(west.len(), 6);
        assert_ne!(north, east);
        assert_ne!(north, south);
        assert_ne!(north, west);

        let invalid = EntityFootprint {
            width: 0,
            height: 1,
        };
        assert!(matches!(
            invalid.validate(),
            Err(SimulationError::InvalidFootprint(0, 1))
        ));
    }

    #[test]
    fn init_world_state_and_step_world_update_stats() {
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(init_world_state);
        schedule.add_systems(step_world);

        let mut world = bevy_ecs::world::World::new();
        world.insert_resource(SimulationWorldState::default());
        world.insert_resource(SimulationClock {
            tick: 5,
            ticks_per_second: 30,
        });
        world.insert_resource(WorldGrid::default());

        let mut grid = WorldGrid::default();
        grid.place_entity(
            GridPosition { x: 0, y: 0 },
            "bench".to_string(),
            EntityFootprint {
                width: 1,
                height: 1,
            },
            EntityFacing::North,
        )
        .expect("place a test entity");
        world.insert_resource(grid);

        schedule.run(&mut world);

        let state = world.resource::<SimulationWorldState>();
        assert_eq!(state.initialized_tick, 5);
        assert_eq!(state.ticks_simulated, 1);
        assert_eq!(state.chunk_count, 1);
        assert_eq!(state.entity_count, 1);
    }

    #[test]
    fn command_apply_supports_entity_variants() {
        let mut config = SimulationConfig::default();
        let mut world = WorldGrid::default();

        apply_command(SimulationCommand::SetTickRate(24), &mut world, &mut config)
            .expect("set tick rate");
        assert_eq!(config.ticks_per_second, 24);

        apply_command(
            SimulationCommand::PlaceEntity {
                entity_type: "chest".to_string(),
                x: 2,
                y: 2,
                width: 1,
                height: 1,
                facing: EntityFacing::North,
            },
            &mut world,
            &mut config,
        )
        .expect("place entity");

        let id = 1;
        let second = world
            .entity_id_at(GridPosition { x: 2, y: 2 })
            .expect("entity exists");
        assert_eq!(second, id);

        apply_command(
            SimulationCommand::MoveEntity {
                entity_id: id,
                x: 3,
                y: 3,
            },
            &mut world,
            &mut config,
        )
        .expect("move entity");
        assert!(!world.has_entity_at(GridPosition { x: 2, y: 2 }));
        assert!(world.has_entity_at(GridPosition { x: 3, y: 3 }));

        apply_command(
            SimulationCommand::RotateEntity {
                entity_id: id,
                times: 1,
            },
            &mut world,
            &mut config,
        )
        .expect("rotate entity");
        assert_eq!(
            world
                .entities
                .get(&id)
                .expect("entity must still exist")
                .facing,
            EntityFacing::East,
        );

        apply_command(
            SimulationCommand::RemoveEntity { x: 3, y: 3 },
            &mut world,
            &mut config,
        )
        .expect("remove by position");
        assert_eq!(world.entity_count(), 0);
    }

    #[test]
    fn deterministic_sim_state_hash_is_reproducible_after_replay() {
        let mut config_a = SimulationConfig::default();
        let mut config_b = SimulationConfig::default();
        let mut clock_a = SimulationClock::default();
        let mut clock_b = SimulationClock::default();

        let mut world_a = WorldGrid::default();
        let mut world_b = WorldGrid::default();

        let script = [
            SimulationCommand::SetTickRate(45),
            SimulationCommand::PlaceEntity {
                entity_type: "belt".to_string(),
                x: 0,
                y: 0,
                width: 2,
                height: 1,
                facing: EntityFacing::East,
            },
            SimulationCommand::MoveEntity {
                entity_id: 1,
                x: 3,
                y: 4,
            },
            SimulationCommand::RotateEntity {
                entity_id: 1,
                times: 3,
            },
            SimulationCommand::PlaceEntity {
                entity_type: "chest".to_string(),
                x: -2,
                y: -1,
                width: 1,
                height: 1,
                facing: EntityFacing::North,
            },
            SimulationCommand::RemoveEntity { x: 3, y: 4 },
        ];

        for command in script {
            apply_command(command.clone(), &mut world_a, &mut config_a)
                .expect("script should execute");
            apply_command(command, &mut world_b, &mut config_b).expect("script should execute");

            clock_a.tick = clock_a.tick.saturating_add(1);
            clock_b.tick = clock_b.tick.saturating_add(1);
        }

        assert_eq!(
            deterministic_sim_state_hash(&clock_a, &config_a, &world_a),
            deterministic_sim_state_hash(&clock_b, &config_b, &world_b)
        );
    }

    #[test]
    fn process_command_queue_tracks_rejections() {
        let mut queue = CommandQueue::default();
        queue.commands.push(SimulationCommand::SetTickRate(0));
        queue.commands.push(SimulationCommand::SetTickRate(30));

        let mut ecs_world = bevy_ecs::world::World::new();
        ecs_world.insert_resource(queue);
        ecs_world.insert_resource(WorldGrid::default());
        ecs_world.insert_resource(SimulationConfig::default());

        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(process_command_queue);
        schedule.run(&mut ecs_world);

        let queue = ecs_world.resource::<CommandQueue>();
        assert_eq!(queue.stats.processed, 1);
        assert_eq!(queue.stats.rejected, 1);
        assert!(queue.commands.is_empty());
    }

    #[test]
    fn edge_case_world_commands_are_safe() {
        let mut world = WorldGrid {
            next_entity_id: u32::MAX,
            ..WorldGrid::default()
        };

        let full = world.place_entity(
            GridPosition { x: 0, y: 0 },
            "edge".to_string(),
            EntityFootprint {
                width: 1,
                height: 1,
            },
            EntityFacing::North,
        );
        assert!(matches!(full, Err(SimulationError::EntityIdExhausted)));

        let mut world = WorldGrid::default();
        let id = world
            .place_entity(
                GridPosition { x: 5, y: 5 },
                "same".to_string(),
                EntityFootprint {
                    width: 1,
                    height: 1,
                },
                EntityFacing::East,
            )
            .expect("place rotating entity");

        world
            .move_entity(id, GridPosition { x: 5, y: 5 })
            .expect("move to same tile is allowed");

        world.rotate_entity(id, 0).expect("rotate-by-zero is no-op");

        world.rotate_entity(id, 4).expect("full rotation is no-op");
    }
}
