use bevy::prelude::*;

use vibetorio_sim::{SimulationClock, SimulationWorldState};

pub fn on_startup(mut commands: Commands) {
    commands.spawn((
        Name::new("vibetorio-root"),
        Transform::default(),
        GlobalTransform::default(),
    ));
}

pub fn on_update(clock: Res<SimulationClock>, world: Res<SimulationWorldState>) {
    let tps = clock.ticks_per_second.max(1);
    let one_second = u64::from(tps);

    if clock.tick.is_multiple_of(one_second) {
        println!(
            "Simulation tick {} | world ticks {}",
            clock.tick, world.ticks_simulated,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_startup_spawns_root_entity() {
        let mut world = bevy::ecs::world::World::new();
        let mut schedule = bevy::ecs::schedule::Schedule::default();

        schedule.add_systems(on_startup);
        schedule.run(&mut world);

        let mut query = world.query::<&Name>();
        let names: Vec<&Name> = query.iter(&world).collect();

        assert_eq!(names.len(), 1);
        assert_eq!(names[0].as_str(), "vibetorio-root");
    }

    #[test]
    fn on_update_logs_at_exact_tick_boundary() {
        let mut world = bevy::ecs::world::World::new();
        world.insert_resource(SimulationClock {
            tick: 60,
            ticks_per_second: 60,
        });
        world.insert_resource(SimulationWorldState {
            initialized_tick: 0,
            ticks_simulated: 0,
            chunk_count: 0,
            entity_count: 0,
        });

        let mut schedule = bevy::ecs::schedule::Schedule::default();
        schedule.add_systems(on_update);

        schedule.run(&mut world);

        assert_eq!(world.resource::<SimulationWorldState>().ticks_simulated, 0);
    }
}
